use leptos::*;
use crate::components::header::Header;
use crate::components::downloader::Downloader;
use crate::components::history::History;
use crate::api;

#[component]
pub fn App() -> impl IntoView {
    // Shared signal to trigger history refresh
    let (refresh_history, set_refresh_history) = create_signal(0);
    provide_context((refresh_history, set_refresh_history));

    let history_resource = create_resource(
        move || refresh_history.get(), 
        |_| async move {
            api::fetch_history().await.unwrap_or_default()
        }
    );

    view! {
        // 'h-screen w-full' makes it immersive
        <main class="h-screen w-full bg-black flex flex-col overflow-hidden">
            // TOP BAR (Decorative but makes it look pro)
            <div class="w-full bg-green-500/10 border-b border-green-500/30 px-6 py-2 flex justify-between items-center text-[0.65rem] text-green-500/50 font-mono">
                <div>"SESSION: 0X9F2A_RUST_DL"</div>
                <div class="animate-pulse">"● CONNECTION_ENCRYPTED"</div>
                <div>"TIME: " {move || "LOCAL_NODE_CLOCK"}</div>
            </div>

            <div class="flex-1 flex overflow-hidden">
                // LEFT SIDEBAR (System Info)
                <aside class="w-64 border-r border-green-500/20 p-6 hidden md:block space-y-8 overflow-y-auto">
                    <div>
                        <h3 class="text-green-500/40 text-[0.6rem] mb-2 uppercase">"System Stats"</h3>
                        <div class="space-y-1 text-[0.7rem] text-green-800">
                            <p>"CPU: NOMINAL"</p>
                            <p>"MEM: 452MB"</p>
                            <p>"UPTIME: 12:44:02"</p>
                        </div>
                    </div>
                    <details class="group">
                        <summary class="text-green-500/40 text-[0.6rem] mb-2 uppercase cursor-pointer list-none flex justify-between items-center outline-none">
                            "All Downloads"
                            <span class="transition group-open:rotate-180">"▼"</span>
                        </summary>
                        <div class="space-y-1 text-[0.7rem] text-green-900 italic mt-2 overflow-y-auto max-h-[60vh] pr-2">
                            <Suspense fallback=move || view! { <p class="animate-pulse">"LOADING..."</p> }>
                                {move || history_resource.get().map(|records| {
                                    if records.is_empty() {
                                        view! { <p>"NO_RECORDS"</p> }.into_view()
                                    } else {
                                        records.into_iter().map(|record| {
                                            view! {
                                                <p class="truncate" title={record.title.clone().unwrap_or_else(|| record.url.clone())}>
                                                    {format!("> {}", record.title.unwrap_or_else(|| record.url))}
                                                </p>
                                            }
                                        }).collect_view()
                                    }
                                })}
                            </Suspense>
                        </div>
                    </details>
                </aside>

                // MAIN CONTENT AREA
                <section class="flex-1 p-8 overflow-y-auto">
                    <div class="max-w-4xl mx-auto">
                        <Header />
                        <Downloader />
                        <History resource=history_resource />
                    </div>
                </section>
            </div>

            // BOTTOM STATUS BAR
            <footer class="bg-green-500 text-black px-6 py-1 text-[0.6rem] font-bold uppercase tracking-widest">
                "Ready for new command sequence... Terminal v1.2.0-STABLE"
            </footer>
        </main>
    }
}
