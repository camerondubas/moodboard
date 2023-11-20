use leptos::*;
use leptos_meta::Html;

use crate::{control_panel::ControlPanel, message_box::MessageBox, moodboard::Moodboard};

use game::events::{DuplexEventsPlugin, Shared, SharedState, TxInputEvent};

#[component]
pub fn App(
    events: TxInputEvent,
    plugin: DuplexEventsPlugin,
    shared: Shared<SharedState>,
) -> impl IntoView {
    let (shared, _set_shared) = create_signal(shared.clone());

    view! {
        <Html class="pointer-events-none" />

        <div class="static">
            <ControlPanel events={events.clone()} />
            <Moodboard plugin={plugin} shared={shared} events={events.clone()}/>
            <MessageBox shared={shared} />
        </div>
    }
}
