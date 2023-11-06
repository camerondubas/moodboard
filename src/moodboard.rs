use leptos::{leptos_dom::logging::console_log, *};

use game::shared::{
    DuplexEventsPlugin, InputEvent, ResizeEvent, Shared, SharedState, TxInputEvent,
};
use web_sys::wasm_bindgen::{prelude::Closure, JsCast};

#[component]
pub fn Moodboard(
    plugin: DuplexEventsPlugin,
    shared: ReadSignal<Shared<SharedState>>,
    events: TxInputEvent,
) -> impl IntoView {
    let plugin_value = store_value(plugin);

    create_effect(move |_| {
        game::run(plugin_value.get_value(), shared.get());
    });

    let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::Event| {
        let window = window();
        events
            .clone()
            .send(InputEvent::Resize(ResizeEvent {
                width: window.inner_width().unwrap().as_f64().unwrap() as f32,
                height: window.inner_height().unwrap().as_f64().unwrap() as f32,
            }))
            .expect("could not send event");
    });

    window()
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .expect("could not add event listener");
    closure.forget();

    view! {
        <canvas id="bevy"></canvas>
    }
}
