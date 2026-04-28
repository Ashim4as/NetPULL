use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="mb-8">
            <h1 class="text-3xl font-bold text-green-500 tracking-tighter uppercase mb-1">
                "// NetPulL"
            </h1>
            <p class="text-[0.7rem] text-green-900 font-mono uppercase border-b border-green-900 pb-2">
                "STATUS: ONLINE | SECTOR: VIDEO_RECOVERY"
            </p>
        </header>
    }
}
