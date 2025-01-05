use ratatui::{
    layout::{Constraint, Direction, Layout as RLayout, Rect},
    Frame,
};
use std::cmp::min;

mod budget;
mod common;
mod grouping;
mod transaction;

#[derive(Eq, PartialEq, Hash, Clone, Default)]
pub enum View {
    #[default]
    Budget,
    Grouping,
    Transaction,
}

#[derive(Default, Clone)]
pub struct Layout {
    pub view: View,
    budget: budget::Budget,
    grouping: grouping::Grouping,
    transaction: transaction::Transaction,
    footer: common::Footer,
    header: common::Header,
    help: common::Help,
    show_help: bool,
}

impl Layout {
    pub fn draw(&self, frame: &mut Frame) {
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

        frame.render_widget(&self.header, header);

        match self.view {
            View::Budget => {
                frame.render_widget(&self.budget, content);
            }
            View::Grouping => {
                frame.render_widget(&self.grouping, content);
            }
            View::Transaction => {
                frame.render_widget(&self.transaction, content);
            }
        };
        frame.render_widget(&self.footer, footer);

        if self.show_help {
            let popup_area = Rect {
                x: area.width.saturating_sub(60) / 2,
                y: area.height.saturating_sub(15) / 2,
                width: min(60, area.width),
                height: min(15, area.height),
            };
            frame.render_widget(&self.help, popup_area);
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }
}
