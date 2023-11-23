use leptos::*;

use game::{
    events::{AddPostItEvent, InputEvent, ThemeEvent, TxInputEvent},
    theme::ThemeMode,
};

use crate::{
    button::{Button, IconButton},
    icons::{IconMoon, IconPlus, IconStyle},
};

#[component]
pub fn ControlPanel(events: TxInputEvent) -> impl IntoView {
    let theme = expect_context::<ReadSignal<ThemeMode>>();
    let set_theme = expect_context::<WriteSignal<ThemeMode>>();

    let evt_clone = events.clone();

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
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        })
    };

    let icon = move || match theme.get() {
        ThemeMode::Light => IconStyle::Outline,
        ThemeMode::Dark => IconStyle::Solid,
    };

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
