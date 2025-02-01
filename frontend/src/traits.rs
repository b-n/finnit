use finnit_abi::FrontendMessageSender;
use ratatui::{layout::Rect, Frame};

use crate::input_events::InputEvent;

pub trait TableRow {
    fn row(&self) -> Vec<String>;
}

pub trait FinnitView {
    /// All finnit views should be able to send messages to the backend
    fn with_sender(sender: FrontendMessageSender) -> Self;

    /// This function is called when the view is changed
    /// Default: do nothing
    fn on_activate(&mut self) {}

    /// Draw the FinnitView in the `frame`'s `area`
    fn draw(&self, frame: &mut Frame, area: Rect);

    /// Handle a key press event
    fn on_input_event(&mut self, _key: InputEvent) {}
}
