use crate::traits::FinnitView;
use crate::views::{Budget, Footer, Grouping, Header, Help, Transaction};
use finnit_abi::FrontendMessage;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use std::sync::mpsc::Sender;

#[derive(Eq, PartialEq, Hash, Clone, Default)]
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

pub fn all_views(sender: Sender<FrontendMessage>) -> crate::views::Views {
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
    fn with_sender(_sender: Sender<FrontendMessage>) -> Self {
        unreachable!()
    }

    fn on_activate(&mut self) {
        match self {
            LoadedView::Budget(b) => b.on_activate(),
            LoadedView::Grouping(g) => g.on_activate(),
            LoadedView::Transaction(t) => t.on_activate(),
            _ => {
                unreachable!()
            }
        }
    }
}

impl Widget for &LoadedView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            LoadedView::Budget(v) => v.render(area, buf),
            LoadedView::Grouping(v) => v.render(area, buf),
            LoadedView::Transaction(v) => v.render(area, buf),
            LoadedView::Footer(v) => v.render(area, buf),
            LoadedView::Header(v) => v.render(area, buf),
            LoadedView::Help(v) => v.render(area, buf),
        }
    }
}
