mod event;

use sdl2::event::Event as SdlEvent;
use std::time::{Duration, Instant};

use crate::ui::command::UiCommand;
use crate::ui::Ui;

pub use event::AppEvent;

pub struct HandleAppEvent<'a> {
    pub event: AppEvent,
    pub ui: &'a mut Ui,
}

pub struct Update<'a> {
    pub time: Instant,
    pub delta: Duration,
    pub ui: &'a mut Ui,
    pub out: &'a mut Vec<AppEvent>,
}

pub trait AppState {
    fn handle_app_event(&mut self, ctx: HandleAppEvent);
    fn handle_input(&mut self, event: &SdlEvent, ui: &mut Ui) -> bool;
    fn handle_ui_command(&mut self, command: UiCommand, ui: &mut Ui);
    fn update(&mut self, ctx: Update);
}
