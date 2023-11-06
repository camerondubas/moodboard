mod app;
mod button;
mod control_panel;
mod moodboard;

use app::App;
use game::shared::{DuplexEventsPlugin, SharedState};
use leptos::*;
use std::sync::{Arc, Mutex};

fn main() {
    let width = window().inner_width().unwrap().as_f64().unwrap() as f32;
    let height = window().inner_height().unwrap().as_f64().unwrap() as f32;

    let shared = Arc::new(Mutex::new(SharedState {
        name: "This can be used for shared state".to_string(),
        window_size: (width, height),
    }));
    let ((tx_events, _rx_events), duplex_events_plugin) = DuplexEventsPlugin::create();

    leptos::mount_to_body(move || {
        view! {
            <App
                events={tx_events.clone()}
                plugin={duplex_events_plugin.clone()}
                shared={shared.clone()}
            />
        }
    });
}
