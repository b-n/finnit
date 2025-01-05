use finnit_abi::FrontendMessage;
use std::sync::mpsc::Sender;

pub trait TableRow {
    fn row(&self) -> Vec<String>;
}

pub trait FinnitView {
    /// All finnit views need to be able to send messages to the backend
    /// This constructor ensures the sender is stored
    fn with_sender(sender: Sender<FrontendMessage>) -> Self;

    /// This function is called when the view is changed
    fn on_activate(&mut self) {}
}
