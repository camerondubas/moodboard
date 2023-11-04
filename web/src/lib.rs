use leptos::*;

use shared::{CounterEvent, InputEvent, RxOutputEvent, Shared, SharedState, TxInputEvent};

pub fn run(tx_events: TxInputEvent, rx_events: RxOutputEvent, shared: Shared<SharedState>) {
    let name = shared.lock().unwrap().name.clone();
    let (count, set_count) = create_signal(0);

    create_effect(move |_| {
        // immediately prints "Value: 0" and subscribes to `count`
        tx_events
            .send(InputEvent::Counter(CounterEvent { value: count.get() }))
            .expect("could not send event");
    });

    mount_to_body(move || {
        view! {
            <p>"Hello, world! Message: "{name.clone()}</p>
            <button on:click=move |_| {
                set_count.update(|x| *x += 1);
            }>
            "Increment"
            </button>
            <button on:click=move |_| {
                set_count.update(|x| *x -= 1);
            }>
            "Decrement"
            </button>

            <span>"Value: " {move || count()}</span>
        }
    })
}
