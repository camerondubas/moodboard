use leptos::*;

use shared::{CounterEvent, InputEvent, TxInputEvent};

use crate::button::Button;

#[component]
pub fn ControlPanel(events: TxInputEvent) -> impl IntoView {
    let (count, set_count) = create_signal(0);

    create_effect(move |_| {
        events
            .send(InputEvent::Counter(CounterEvent { value: count.get() }))
            .expect("could not send event");
    });

    view! {
        <div class="absolute top-[50px] left-[50px] p-6 mx-auto bg-white dark:bg-slate-800 rounded-xl shadow-lg flex items-center space-x-4 text-xl font-medium text-black">
            <Button on:click=move |_| set_count.update(|x| *x += 1)>
                "Increment"
            </Button>
            <Button on:click=move |_| set_count.update(|x| *x -= 1)>
                "Decrement"
            </Button>
            <span class="text-slate-500 dark:text-slate-400">"Value: " {move || count()}</span>
            <Button on:click=move |_| {
                let html_element = document().query_selector("html").unwrap().unwrap();
                let class_list = html_element.class_list();
                if class_list.contains("dark") {
                    class_list.remove_1("dark");
                } else {
                    class_list.add_1("dark");
                }
            }>
                "Toggle Theme"
            </Button>
        </div>
    }
}
