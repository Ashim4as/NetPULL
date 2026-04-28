use leptos::*;
use crate::api;

use crate::models::DownloadRecord;

#[component]
pub fn History(resource: Resource<i32, Vec<DownloadRecord>>) -> impl IntoView {
    // Get the shared refresh trigger from App context
    let (_, set_refresh_history) = use_context::<(ReadSignal<i32>, WriteSignal<i32>)>()
        .expect("App should provide refresh context");

    let refresh = move |_| {
        set_refresh_history.update(|n| *n += 1);
    };

    view! {
        <div class="mt-12 space-y-6">
            <div class="flex items-center justify-between border-b border-green-900 pb-2">
                <h2 class="text-xs font-bold text-green-500 uppercase tracking-tighter">
                    "//_DOWNLOAD_HISTORY"
                </h2>
                <button 
                    on:click=refresh
                    class="text-[0.6rem] text-green-700 hover:text-green-400 transition-colors"
                >
                    "[REFRESH]"
                </button>
            </div>

            <div class="overflow-x-auto">
                <table class="w-full text-left font-mono text-[0.65rem]">
                    <thead>
                        <tr class="text-green-900 border-b border-green-950">
                            <th class="pb-2 font-normal">"ID"</th>
                            <th class="pb-2 font-normal">"TITLE"</th>
                            <th class="pb-2 font-normal">"STATUS"</th>
                            <th class="pb-2 font-normal">"DATE"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-green-950/30">
                        <Suspense fallback=move || view! { <tr><td colspan="4" class="py-4 text-center text-green-900 animate-pulse">"FETCHING_DATA..."</td></tr> }>
                            {move || {
                                resource.get().map(|records| {
                                    if records.is_empty() {
                                        view! {
                                            <tr>
                                                <td colspan="4" class="py-4 text-center text-green-900 italic">
                                                    "NO_RECORDS_FOUND"
                                                </td>
                                            </tr>
                                        }.into_view()
                                    } else {
                                        records.into_iter().take(15).map(|record| {
                                            let status_class = match record.status.as_str() {
                                                "COMPLETED" => "text-green-400",
                                                "FAILED" => "text-red-500",
                                                "DOWNLOADING" => "text-blue-400 animate-pulse",
                                                _ => "text-green-700",
                                            };

                                            view! {
                                                <tr class="hover:bg-green-500/5 transition-colors group">
                                                    <td class="py-3 text-green-900">
                                                        {format!("#{:03}", record.id)}
                                                    </td>
                                                    <td class="py-3 pr-4 max-w-xs truncate text-green-300">
                                                        {record.title.unwrap_or_else(|| record.url.clone())}
                                                    </td>
                                                    <td class="py-3">
                                                        <span class=status_class>{record.status}</span>
                                                    </td>
                                                    <td class="py-3 text-green-900">
                                                        {record.created_at.unwrap_or_default()}
                                                    </td>
                                                </tr>
                                            }
                                        }).collect_view()
                                    }
                                })
                            }}
                        </Suspense>
                    </tbody>
                </table>
            </div>
        </div>
    }
}
