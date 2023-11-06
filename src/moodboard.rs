use leptos::*;

use game::shared::{DuplexEventsPlugin, Shared, SharedState};

#[component]
pub fn Moodboard(
    plugin: DuplexEventsPlugin,
    shared: ReadSignal<Shared<SharedState>>,
) -> impl IntoView {
    let plugin_value = store_value(plugin);

    create_effect(move |_| {
        game::run(plugin_value.get_value(), shared.get());
    });

    view! {
        <canvas id="bevy"></canvas>
    }
}
