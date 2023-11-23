mod app;
mod button;
mod control_panel;
mod icons;
mod message_box;
mod moodboard;

use app::App;
use game::{
    events::{DuplexEventsPlugin, SharedState},
    theme::ThemeMode,
};
use leptos::*;
use leptos_meta::Html;
use std::sync::{Arc, Mutex};

fn main() {
    let width = window().inner_width().unwrap().as_f64().unwrap() as f32;
    let height = window().inner_height().unwrap().as_f64().unwrap() as f32;

    let shared = Arc::new(Mutex::new(SharedState {
        name: "This can be used for shared state".to_string(),
        window_size: (width, height),
    }));
    let ((tx_events, _rx_events), duplex_events_plugin) = DuplexEventsPlugin::create();

    let theme_signal = create_signal(ThemeMode::Light);
    let (theme, set_theme) = theme_signal;

    let theme_class = move || match theme.get() {
        ThemeMode::Dark => "dark",
        ThemeMode::Light => "",
    };

    provide_context(theme);
    provide_context(set_theme);

    leptos::mount_to_body(move || {
        view! {
            <Html class=theme_class  />
            <App
                events={tx_events.clone()}
                plugin={duplex_events_plugin.clone()}
                shared={shared.clone()}
            />
        }
    });
}
