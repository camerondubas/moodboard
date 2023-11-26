use leptos::*;

use game::{
    events::{AddItemEvent, InputEvent, ThemeEvent, TxInputEvent},
    theme::ThemeMode,
};

use crate::{
    button::IconButton,
    icons::{IconChatBubble, IconMoon, IconPencilSquare, IconStyle, IconSwatch},
};

#[component]
pub fn ControlPanel(events: TxInputEvent) -> impl IntoView {
    let theme = expect_context::<ReadSignal<ThemeMode>>();
    let set_theme = expect_context::<WriteSignal<ThemeMode>>();

    let evt_clone = events.clone();
    let evt_clone2 = events.clone();
    let evt_clone3 = events.clone();

    let add_post_it = move |_| {
        evt_clone2
            .send(InputEvent::AddItem(AddItemEvent::PostIt(String::from(
                "Hello World!",
            ))))
            .expect("could not send event");
    };

    let add_swatch = move |_| {
        events
            .clone()
            .send(InputEvent::AddItem(AddItemEvent::Swatch(String::from(
                "#f3f4f6",
            ))))
            .expect("could not send event");
    };

    let add_text = move |_| {
        evt_clone3
            .send(InputEvent::AddItem(AddItemEvent::Text(String::from(
                "Just another text box",
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

                <IconButton on:click=add_swatch>
                    <IconSwatch />
                </IconButton>

                <IconButton on:click=add_post_it>
                    <IconChatBubble />
                </IconButton>

                <IconButton on:click=add_text>
                    <IconPencilSquare />
                </IconButton>

                // <IconButton on:click=add_post_it.clone()>
                //     <IconPhoto />
                // </IconButton>

                // <Button on:click=move |_| {}>
                //     "Update Text"
                // </Button>

                <IconButton on:click=toggle_theme >
                    <IconMoon style={Box::new(icon)}/>
                </IconButton>

            </div>
        </div>
    }
}
