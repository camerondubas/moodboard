use crate::prelude::*;
use std::sync::{Arc, Mutex};

use crate::theme::ThemeMode;

#[derive(Debug)]
pub struct SharedState {
    pub name: String,
    pub window_size: (f32, f32),
}

pub type Shared<T> = Arc<Mutex<T>>;

#[derive(Debug)]
pub enum InputEvent {
    Theme(ThemeEvent),
    Resize(ResizeEvent),
    AddItem(AddItemEvent),
}

#[derive(Clone, Debug, Event)]
pub struct ThemeEvent {
    pub theme: ThemeMode,
}

#[derive(Clone, Debug, Event)]
pub struct ResizeEvent {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Debug, Event)]
pub enum AddItemEvent {
    Text(String),
    Image(String),
    Swatch(String),
    PostIt(String),
}

#[derive(Debug)]
pub enum OutputEvent {
    Click,
}

#[derive(Clone, Resource, Deref)]
pub struct TxInputEvent(pub crossbeam_channel::Sender<InputEvent>);
#[derive(Clone, Resource, Deref)]
pub struct RxInputEvent(pub crossbeam_channel::Receiver<InputEvent>);

#[derive(Resource, Deref, DerefMut, Clone)]
pub struct TxOutputEvent(pub crossbeam_channel::Sender<OutputEvent>);
#[derive(Resource, Deref, DerefMut)]
pub struct RxOutputEvent(pub crossbeam_channel::Receiver<OutputEvent>);

pub struct DuplexEventsPlugin {
    rx_input_event: RxInputEvent,
    tx_output_event: TxOutputEvent,
}

type ExternalChannels = (TxInputEvent, RxOutputEvent);

impl DuplexEventsPlugin {
    pub fn create() -> (ExternalChannels, Self) {
        let (tx_input_event, rx_input_event) = crossbeam_channel::bounded(50);
        let (tx_output_event, rx_output_event) = crossbeam_channel::bounded(50);
        (
            (TxInputEvent(tx_input_event), RxOutputEvent(rx_output_event)),
            Self {
                rx_input_event: RxInputEvent(rx_input_event),
                tx_output_event: TxOutputEvent(tx_output_event),
            },
        )
    }
}

impl Clone for DuplexEventsPlugin {
    fn clone(&self) -> Self {
        Self {
            rx_input_event: self.rx_input_event.clone(),
            tx_output_event: self.tx_output_event.clone(),
        }
    }
}

impl DuplexEventsPlugin {
    fn internal_channels(&self) -> (TxOutputEvent, RxInputEvent) {
        (self.tx_output_event.clone(), self.rx_input_event.clone())
    }
}

impl Plugin for DuplexEventsPlugin {
    fn build(&self, app: &mut App) {
        let (tx_output_event, rx_input_event) = self.internal_channels();

        app.insert_resource(rx_input_event)
            .insert_resource(tx_output_event)
            .init_resource::<Events<ThemeEvent>>()
            .init_resource::<Events<ResizeEvent>>()
            .init_resource::<Events<AddItemEvent>>()
            .add_systems(PreUpdate, input_events_system);
    }
}

fn input_events_system(
    rx_input_event: Res<RxInputEvent>,
    mut theme_event_writer: EventWriter<ThemeEvent>,
    mut resize_event_writer: EventWriter<ResizeEvent>,
    mut add_item_event_writer: EventWriter<AddItemEvent>,
) {
    for input_event in rx_input_event.try_iter() {
        match input_event {
            InputEvent::Theme(event) => {
                theme_event_writer.send(event);
            }
            InputEvent::Resize(event) => {
                resize_event_writer.send(event);
            }

            InputEvent::AddItem(event) => {
                add_item_event_writer.send(event);
            }
        }
    }
}
