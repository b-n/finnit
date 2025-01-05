use finnit_abi::FrontendMessage;
use ratatui::{
    layout::{Constraint, Direction, Layout as RLayout, Rect},
    Frame,
};
use std::cmp::min;
use std::sync::mpsc::Sender;

use crate::traits::FinnitView;

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
    pub fn with_sender(sender: Sender<FrontendMessage>) -> Self {
        let views = all_views(sender);

        Self {
            view: View::Budget,
            views,
            show_help: false,
        }
    }

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

    pub fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let chunks = RLayout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(7),
                Constraint::Min(1),
                Constraint::Length(4),
            ])
            .split(area);

        let (header, content, footer) = (chunks[0], chunks[1], chunks[2]);

        frame.render_widget(&self.views.header, header);

        let active_view: &LoadedView = self.active_view();
        frame.render_widget(active_view, content);

        frame.render_widget(&self.views.footer, footer);

        if self.show_help {
            let popup_area = Rect {
                x: area.width.saturating_sub(60) / 2,
                y: area.height.saturating_sub(15) / 2,
                width: min(60, area.width),
                height: min(15, area.height),
            };
            frame.render_widget(&self.views.help, popup_area);
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }
}
