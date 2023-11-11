use leptos::*;
use leptos_meta::Html;

use crate::{control_panel::ControlPanel, moodboard::Moodboard};

use game::events::{DuplexEventsPlugin, Shared, SharedState, TxInputEvent};

#[component]
pub fn App(
    events: TxInputEvent,
    plugin: DuplexEventsPlugin,
    shared: Shared<SharedState>,
) -> impl IntoView {
    let (shared, _set_shared) = create_signal(shared.clone());
    let name = shared.get().lock().unwrap().name.clone();

    view! {
        <Html class="pointer-events-none" />

        <div class="static">
            <div class="absolute bottom-0 right-0 bg-white dark:bg-slate-800 rounded-xl shadow-lg m-4">
                <p class="p-4 text-slate-500 dark:text-slate-400">"Hello, world! Message: "{name.clone()}</p>
            </div>
            <ControlPanel events={events.clone()} />
            <Moodboard plugin={plugin} shared={shared} events={events.clone()}/>
        </div>
    }
}
