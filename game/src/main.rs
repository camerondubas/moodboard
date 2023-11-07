use std::sync::{Arc, Mutex};

use game::events::{DuplexEventsPlugin, SharedState};

fn main() {
    println!("Hello, world!");
    let (_events, event_plugin) = DuplexEventsPlugin::create();
    let shared = Arc::new(Mutex::new(SharedState {
        name: "Hello, world!".to_string(),
        window_size: (800., 600.),
    }));

    game::run(event_plugin, shared);
}
