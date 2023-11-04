use shared::{DuplexEventsPlugin, SharedState};
use std::sync::{Arc, Mutex};

fn main() {
    let shared = Arc::new(Mutex::new(SharedState {
        name: "This can be used for shared state".to_string(),
    }));
    let ((tx_events, rx_events), duplex_events_plugin) = DuplexEventsPlugin::create();

    web::run(tx_events, rx_events, shared.clone());
    game::run(duplex_events_plugin, shared);
}
