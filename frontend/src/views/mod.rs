use finnit_abi::FrontendMessage;
use log::info;
use ratatui::{
    layout::{Constraint, Direction, Layout as RLayout, Rect},
    Frame,
};
use std::cmp::min;
use std::sync::mpsc::Sender;

use crate::{FinnitView, InputEvent};

mod budget;
mod common;
mod enums;
mod grouping;
mod transaction;

pub use enums::{all_views, View};

use budget::Budget;
use common::{Footer, Header, Help};
use enums::LoadedView;
use grouping::Grouping;
use transaction::Transaction;

pub struct Layout {
    view: View,
    views: Views,
    show_help: bool,
}

pub struct Views {
    budget: LoadedView,
    grouping: LoadedView,
    transaction: LoadedView,
    footer: LoadedView,
    header: LoadedView,
    help: LoadedView,
}

impl Layout {
    pub fn set_view(&mut self, view: View) {
        self.view = view;
        self.active_view().on_activate();
    }

    fn active_view(&mut self) -> &mut LoadedView {
        match self.view {
            View::Budget => &mut self.views.budget,
            View::Grouping => &mut self.views.grouping,
            View::Transaction => &mut self.views.transaction,
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }
}

impl FinnitView for Layout {
    fn with_sender(sender: Sender<FrontendMessage>) -> Self {
        let views = all_views(sender);

        Self {
            view: View::Budget,
            views,
            show_help: false,
        }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let chunks = RLayout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7),
                Constraint::Min(1),
                Constraint::Length(4),
            ])
            .split(area);

        let (header, content, footer) = (chunks[0], chunks[1], chunks[2]);

        // Render the main page chunks
        self.views.header.draw(frame, header);
        self.active_view().draw(frame, content);
        self.views.footer.draw(frame, footer);

        // Render popups
        if self.show_help {
            let popup_area = Rect {
                x: area.width.saturating_sub(60) / 2,
                y: area.height.saturating_sub(15) / 2,
                width: min(60, area.width),
                height: min(15, area.height),
            };
            self.views.help.draw(frame, popup_area);
        }
    }

    fn on_input_event(&mut self, event: InputEvent) {
        info!("Layout: {event:?}");
        match event {
            InputEvent::ChangeView(view) => self.set_view(view),
            InputEvent::ToggleModal => self.toggle_help(),
            _ => self.active_view().on_input_event(event),
        }
    }
}

impl Layout {
    pub fn set_transactions(&mut self, transactions: Vec<finnit_abi::Transaction>) {
        if let LoadedView::Transaction(view) = &mut self.views.transaction {
            view.set_transactions(transactions);
        }
    }
}
