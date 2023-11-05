mod app;
mod button;
mod control_panel;
mod moodboard;

use app::App;
use leptos::*;
use shared::{DuplexEventsPlugin, SharedState};
use std::sync::{Arc, Mutex};

fn main() {
    let shared = Arc::new(Mutex::new(SharedState {
        name: "This can be used for shared state".to_string(),
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
