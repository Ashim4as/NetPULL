use crate::models::{ProgressUpdate, DownloadRecord};
use gloo_net::http::Request;
use web_sys::{EventSource, MessageEvent};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub async fn fetch_history() -> Result<Vec<DownloadRecord>, String> {
    let response = Request::get("http://127.0.0.1:3000/history")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        response.json::<Vec<DownloadRecord>>()
            .await
            .map_err(|e| e.to_string())
    } else {
        Err(format!("Error: {}", response.status()))
    }
}

pub fn start_progress_stream<F>(url: String, quality: String, on_update: F) -> Result<EventSource, JsValue>
where
    F: Fn(ProgressUpdate) + 'static,
{
    let encoded_url = js_sys::encode_uri_component(&url);
    let endpoint = format!("http://127.0.0.1:3000/download?url={}&quality={}", encoded_url, quality);
    
    let es = EventSource::new(&endpoint)?;

    let on_message = Closure::wrap(Box::new(move |event: MessageEvent| {
        if let Some(data) = event.data().as_string() {
            if let Ok(update) = serde_json::from_str::<ProgressUpdate>(&data) {
                on_update(update);
            }
        }
    }) as Box<dyn FnMut(MessageEvent)>);

    es.set_onmessage(Some(on_message.as_ref().unchecked_ref()));
    on_message.forget();

    Ok(es)
}
