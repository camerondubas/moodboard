use leptos::*;

use game::{
    events::{CounterEvent, InputEvent, ThemeEvent, TxInputEvent},
    theme::Theme,
};

use crate::{
    button::{Button, IconButton},
    icons::{IconArrowDown, IconArrowUp, IconMoon, IconStyle},
};

#[component]
pub fn ControlPanel(events: TxInputEvent) -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let theme = expect_context::<ReadSignal<Theme>>();
    let set_theme = expect_context::<WriteSignal<Theme>>();

    let increment = move |_| set_count.update(|x| *x += 1);
    let decrement = move |_| set_count.update(|x| *x -= 1);

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

    let evt_clone = events.clone();

    create_effect(move |_| {
        events
            .clone()
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
