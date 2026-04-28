use std::process::Stdio;
use tokio::process::Command;
use tokio::io::{AsyncBufReadExt, BufReader};
use crate::models::ProgressUpdate;
use async_stream::stream;
use futures::Stream;
use sqlx::SqlitePool;

pub fn stream_download(url: String, quality: Option<String>, pool: SqlitePool) -> impl Stream<Item = ProgressUpdate> {
    stream! {
        yield ProgressUpdate { status: "INITIATING...".to_string(), ..Default::default() };
        
        let row_id = sqlx::query(
            "INSERT INTO downloads (url, title, status) VALUES (?, ?, ?)"
        )
        .bind(&url)
        .bind("Fetching Metadata...")
        .bind("DOWNLOADING")
        .execute(&pool)
        .await
        .map(|r| r.last_insert_rowid())
        .unwrap_or(0);

        let q = quality.unwrap_or_else(|| "best".to_string());
        let output_template = format!("../downloads/%(title)s [{}].%(ext)s", q.to_uppercase());
        
        let mut format_arg = "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best";
        let mut merge_arg = vec!["--merge-output-format", "mp4"];

        if q == "1080p" {
            format_arg = "bestvideo[ext=mp4][height<=1080]+bestaudio[ext=m4a]/best[ext=mp4][height<=1080]/best";
        } else if q == "720p" {
            format_arg = "bestvideo[ext=mp4][height<=720]+bestaudio[ext=m4a]/best[ext=mp4][height<=720]/best";
        } else if q == "480p" {
            format_arg = "bestvideo[ext=mp4][height<=480]+bestaudio[ext=m4a]/best[ext=mp4][height<=480]/best";
        } else if q == "audio" {
            format_arg = "bestaudio[ext=m4a]/best";
            merge_arg = vec![]; // No merging needed for audio only
        }

        let mut args = vec![
            "--newline",
            "--no-colors",
            "--no-simulate",
            "--trim-filenames", "100",
            "-f", format_arg,
        ];
        args.extend(merge_arg);
        args.extend(vec![
            "--print", "TITLE:%(title)s",
            "--progress",
            "--progress-template", "PROGRESS:%(progress._percent_str)s|%(progress._speed_str)s|%(progress._eta_str)s",
            "-o", &output_template,
            &url
        ]);

        // We use custom sentinels (TITLE: and PROGRESS:) to make parsing 100% reliable
        let mut child = Command::new("yt-dlp")
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .expect("Failed to start yt-dlp");

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();
        
        let mut reader = BufReader::new(stdout).lines();
        let mut err_reader = BufReader::new(stderr).lines();

        let mut final_status = "COMPLETED";
        let mut stdout_done = false;
        let mut stderr_done = false;

        loop {
            tokio::select! {
                result = reader.next_line(), if !stdout_done => {
                    match result {
                        Ok(Some(line)) => {
                            let line = line.trim();
                            if line.is_empty() { continue; }

                            // 1. Detect Title
                            if line.starts_with("TITLE:") {
                                let title = line.replace("TITLE:", "").trim().to_string();
                                println!(">>> TITLE_SYNC: {}", title);
                                if row_id > 0 {
                                    let _ = sqlx::query("UPDATE downloads SET title = ? WHERE id = ?")
                                        .bind(&title)
                                        .bind(row_id)
                                        .execute(&pool)
                                        .await;
                                }
                                continue;
                            }

                            // 2. Detect Progress (Format: PROGRESS:percent|speed|eta)
                            if line.starts_with("PROGRESS:") {
                                let data = line.replace("PROGRESS:", "");
                                let parts: Vec<&str> = data.split('|').collect();
                                
                                if parts.len() == 3 {
                                    yield ProgressUpdate {
                                        percent: parts[0].replace('%', "").trim().to_string(),
                                        speed: parts[1].to_string(),
                                        eta: parts[2].to_string(),
                                        status: "DOWNLOADING".to_string(),
                                    };
                                }
                                continue;
                            }
                        }
                        _ => { stdout_done = true; }
                    }
                }
                result = err_reader.next_line(), if !stderr_done => {
                    match result {
                        Ok(Some(line)) => {
                            if line.contains("ERROR") {
                               println!(">>> YTDL_ERROR: {}", line);
                               final_status = "FAILED";
                               yield ProgressUpdate { status: "FAILED".to_string(), ..Default::default() };
                               break;
                            }
                        }
                        _ => { stderr_done = true; }
                    }
                }
            }

            if stdout_done && stderr_done {
                break;
            }
        }

        let _ = child.wait().await;
        
        if row_id > 0 {
            let _ = sqlx::query("UPDATE downloads SET status = ? WHERE id = ?")
                .bind(final_status)
                .bind(row_id)
                .execute(&pool)
                .await;
        }

        if final_status == "COMPLETED" {
            yield ProgressUpdate { percent: "100".to_string(), speed: "0".to_string(), eta: "0".to_string(), status: "COMPLETED".to_string() };
        }
    }
}
