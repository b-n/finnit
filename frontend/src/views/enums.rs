use finnit_abi::FrontendMessageSender;
use ratatui::{layout::Rect, Frame};

use crate::views::{Budget, Footer, Grouping, Header, Help, Transaction};
use crate::{FinnitView, InputEvent};

#[derive(Eq, PartialEq, Hash, Clone, Default, Debug)]
pub enum View {
    #[default]
    Budget,
    Grouping,
    Transaction,
}

pub enum LoadedView {
    Budget(Budget),
    Grouping(Grouping),
    Transaction(Transaction),
    Footer(Footer),
    Header(Header),
    Help(Help),
}

pub fn all_views(sender: FrontendMessageSender) -> crate::views::Views {
    crate::views::Views {
        budget: LoadedView::Budget(Budget::with_sender(sender.clone())),
        grouping: LoadedView::Grouping(Grouping::with_sender(sender.clone())),
        transaction: LoadedView::Transaction(Transaction::with_sender(sender.clone())),
        header: LoadedView::Header(Header::default()),
        footer: LoadedView::Footer(Footer::default()),
        help: LoadedView::Help(Help::default()),
    }
}

impl FinnitView for LoadedView {
    // Needed for blanket implementation, but we will never need/use this.
    fn with_sender(_sender: FrontendMessageSender) -> Self {
        unreachable!()
    }

    fn on_activate(&mut self) {
        match self {
            LoadedView::Budget(v) => v.on_activate(),
            LoadedView::Grouping(v) => v.on_activate(),
            LoadedView::Transaction(v) => v.on_activate(),
            LoadedView::Header(v) => v.on_activate(),
            LoadedView::Footer(v) => v.on_activate(),
            LoadedView::Help(v) => v.on_activate(),
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        match self {
            LoadedView::Budget(v) => v.draw(frame, area),
            LoadedView::Grouping(v) => v.draw(frame, area),
            LoadedView::Transaction(v) => v.draw(frame, area),
            LoadedView::Header(v) => v.draw(frame, area),
            LoadedView::Footer(v) => v.draw(frame, area),
            LoadedView::Help(v) => v.draw(frame, area),
        }
    }

    fn on_input_event(&mut self, event: InputEvent) {
        match self {
            LoadedView::Budget(v) => v.on_input_event(event),
            LoadedView::Grouping(v) => v.on_input_event(event),
            LoadedView::Transaction(v) => v.on_input_event(event),
            LoadedView::Header(v) => v.on_input_event(event),
            LoadedView::Footer(v) => v.on_input_event(event),
            LoadedView::Help(v) => v.on_input_event(event),
        }
    }
}
