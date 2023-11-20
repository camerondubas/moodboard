use game::events::{Shared, SharedState};
use leptos::*;

#[component]
pub fn MessageBox(shared: ReadSignal<Shared<SharedState>>) -> impl IntoView {
    #[cfg(feature = "debug")]
    let name = shared.get().lock().unwrap().name.clone();

    #[cfg(feature = "debug")]
    view! {
        <div class="pointer-events-auto absolute bottom-0 right-0 bg-white dark:bg-slate-800 rounded-xl shadow-lg m-4">
            <p class="p-4 text-slate-500 dark:text-slate-400">"Message: "{name.clone()}</p>
        </div>
    }
}
