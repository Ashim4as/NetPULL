use leptos::*;
use crate::api;
use crate::models::ProgressUpdate;
use web_sys::EventSource;

#[component]
pub fn Downloader() -> impl IntoView {
    let (url, set_url) = create_signal(String::new());
    let (is_loading, set_loading) = create_signal(false);
    let (progress, set_progress) = create_signal(ProgressUpdate::default());
    let (active_stream, set_active_stream) = create_signal(Option::<EventSource>::None);
    let (quality, set_quality) = create_signal("best".to_string());

    // Get the shared refresh trigger from App context
    let (_, set_refresh_history) = use_context::<(ReadSignal<i32>, WriteSignal<i32>)>()
        .expect("App should provide refresh context");

    let on_download = move |_| {
        let current_url = url.get();
        set_loading.set(true);
        
        // Trigger history refresh so the "DOWNLOADING" status shows up immediately
        set_refresh_history.update(|n| *n += 1);

        if let Ok(es) = api::start_progress_stream(current_url, quality.get(), move |update| {
            set_progress.set(update.clone());
            if update.status == "COMPLETED" || update.status == "FAILED" {
                set_loading.set(false);
            }
        }) {
            set_active_stream.set(Some(es));
        }
    };

    // Effect to handle completion and auto-refresh
    create_effect(move |_| {
        let current_progress = progress.get();
        
        if current_progress.status == "COMPLETED" {
            // Trigger history refresh
            set_refresh_history.update(|n| *n += 1);

            // 1. Close the stream
            if let Some(es) = active_stream.get() {
                es.close();
                set_active_stream.set(None);
            }

            // 2. Wait 3 seconds and then Refresh the state!
            set_timeout(move || {
                set_url.set(String::new());
                set_progress.set(ProgressUpdate::default());
                set_loading.set(false);
                println!(">>> STATE REFRESHED FOR NEXT DOWNLOAD");
            }, std::time::Duration::from_secs(3));
        } else if current_progress.status == "FAILED" {
            // Trigger history refresh
            set_refresh_history.update(|n| *n += 1);

            if let Some(es) = active_stream.get() {
                es.close();
                set_active_stream.set(None);
            }
        }
    });

    let on_cancel = move |_| {
        if let Some(es) = active_stream.get() {
            es.close();
            set_active_stream.set(None);
            set_loading.set(false);
            set_progress.set(ProgressUpdate {
                status: "CANCELLED_BY_USER".to_string(),
                ..Default::default()
            });
        }
    };

    view! {
        <div class="space-y-8">
            <div class="relative flex items-center group">
                <span class="mr-3 text-green-500 font-bold">">"</span>
                <input 
                    type="text" 
                    placeholder="ENTER_SOURCE_URL"
                    class="w-full bg-transparent border-b border-green-900 text-green-400 placeholder-green-900 py-2 outline-none focus:border-green-500 transition-colors font-mono text-sm"
                    on:input=move |ev| set_url.set(event_target_value(&ev))
                    prop:value=url
                />
            </div>

            <div class="relative flex items-center group">
                <span class="mr-3 text-green-500 font-bold">"*"</span>
                <select 
                    class="w-full bg-black border-b border-green-900 text-green-400 py-2 outline-none focus:border-green-500 transition-colors font-mono text-sm appearance-none cursor-pointer"
                    on:change=move |ev| set_quality.set(event_target_value(&ev))
                    prop:value=quality
                >
                    <option value="best">"BEST_QUALITY (DEFAULT)"</option>
                    <option value="1080p">"1080P"</option>
                    <option value="720p">"720P"</option>
                    <option value="480p">"480P"</option>
                    <option value="audio">"AUDIO_ONLY"</option>
                </select>
                <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-green-500">
                    <svg class="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z"/></svg>
                </div>
            </div>

            {move || if is_loading.get() && active_stream.get().is_some() {
                view! {
                    <button 
                        class="w-full border-2 border-red-500 text-red-500 py-3 text-sm font-bold uppercase tracking-widest hover:bg-red-500 hover:text-black transition-all"
                        on:click=on_cancel
                    >
                        "Cancel_Download"
                    </button>
                }.into_view()
            } else {
                view! {
                    <button 
                        class="w-full border-2 border-green-500 py-3 text-sm font-bold uppercase tracking-widest hover:bg-green-500 hover:text-black transition-all disabled:opacity-30"
                        on:click=on_download 
                        disabled=move || is_loading.get() || url.get().is_empty()
                    >
                        "Execute_Download"
                    </button>
                }.into_view()
            }}

            <div class="space-y-4 font-mono">
                <div class="w-full bg-green-950/30 border border-green-900 h-6 relative overflow-hidden">
                    <div 
                        class="bg-green-500 h-full transition-all duration-300 shadow-[0_0_15px_#22c55e]"
                        class:animate-pulse=move || progress.get().status == "INITIATING..."
                        style:width=move || {
                            let p = progress.get().percent;
                            if p.is_empty() { "0%".to_string() } else { format!("{}%", p) }
                        }
                    ></div>
                    <span class="absolute inset-0 flex items-center justify-center text-[0.6rem] font-bold text-white mix-blend-difference">
                        {move || {
                            let p = progress.get().percent;
                            if p.is_empty() { "0.0%".to_string() } else { format!("{}%", p) }
                        }}
                    </span>
                </div>

                <div class="grid grid-cols-3 gap-4 text-[0.65rem] text-green-600">
                    <div class="border border-green-900/30 p-2">
                        <span class="block text-green-900 mb-1">"SPEED"</span>
                        {move || {
                            let s = progress.get().speed;
                            if s.is_empty() { "---".to_string() } else { s }
                        }}
                    </div>
                    <div class="border border-green-900/30 p-2">
                        <span class="block text-green-900 mb-1">"ETA"</span>
                        {move || {
                            let e = progress.get().eta;
                            if e.is_empty() { "--:--".to_string() } else { e }
                        }}
                    </div>
                    <div class="border border-green-900/30 p-2">
                        <span class="block text-green-900 mb-1">"STATUS"</span>
                        <span class="truncate block">
                            {move || {
                                let s = progress.get().status;
                                if s.is_empty() { "IDLE".to_string() } else { s }
                            }}
                        </span>
                    </div>
                </div>
            </div>

            <div class="bg-green-500/5 p-4 border border-green-900/30">
                <p class="text-[0.6rem] text-green-700 break-all uppercase">
                    "[LOG] SYSTEM_STATUS: " {move || progress.get().status}
                </p>
            </div>
        </div>
    }
}
