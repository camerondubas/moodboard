use leptos::*;

use game::{
    events::{AddPostItEvent, CounterEvent, InputEvent, ThemeEvent, TxInputEvent},
    theme::Theme,
};

use crate::{
    button::{Button, IconButton},
    icons::{IconArrowDown, IconArrowUp, IconMoon, IconPlus, IconStyle},
};

#[component]
pub fn ControlPanel(events: TxInputEvent) -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let theme = expect_context::<ReadSignal<Theme>>();
    let set_theme = expect_context::<WriteSignal<Theme>>();

    let increment = move |_| set_count.update(|x| *x += 1);
    let decrement = move |_| set_count.update(|x| *x -= 1);
    let evt_clone = events.clone();
    let evt_clone2 = events.clone();

    let add_post_it = move |_| {
        events
            .clone()
            .send(InputEvent::AddPostIt(AddPostItEvent(String::from(
                "Hello World!",
            ))))
            .expect("could not send event");
    };

    let toggle_theme = move |_| {
        set_theme.set(match theme() {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        })
    };

    let icon = move || match theme.get() {
        Theme::Light => IconStyle::Outline,
        Theme::Dark => IconStyle::Solid,
    };

    create_effect(move |_| {
        evt_clone2
            .send(InputEvent::Counter(CounterEvent { value: count.get() }))
            .expect("could not send event");
    });

    create_effect(move |_| {
        evt_clone
            .send(InputEvent::Theme(ThemeEvent { theme: theme.get() }))
            .expect("could not send event");
    });

    view! {
        <div class="flex mt-6">
            <div class="pointer-events-auto flex-initial p-6 mx-auto bg-white dark:bg-slate-800 rounded-xl shadow-lg flex items-center space-x-4 text-xl font-medium text-black">

                <IconButton on:click=add_post_it>
                    <IconPlus />
                </IconButton>

                <IconButton on:click=increment>
                    <IconArrowUp />
                </IconButton>

                <IconButton on:click=decrement>
                    <IconArrowDown />
                </IconButton>

                <Button on:click=move |_| {}>
                    "Update Text"
                </Button>

                <IconButton on:click=toggle_theme >
                    <IconMoon style={Box::new(icon)}/>
                </IconButton>

            </div>
        </div>
    }
}
