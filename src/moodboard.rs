use leptos::*;

use shared::{DuplexEventsPlugin, Shared, SharedState};

#[component]
pub fn Moodboard(plugin: DuplexEventsPlugin, shared: Shared<SharedState>) -> impl IntoView {
    let a = store_value(plugin);
    let b = store_value(shared);
    create_effect(move |_| {
        game::run(a.get_value(), b.get_value());
    });

    view! {
        <canvas id="bevy"></canvas>
    }
}
