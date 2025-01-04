use ratatui::{
    layout::{Constraint, Direction, Layout as RLayout},
    Frame,
};

mod budget;
mod common;
mod grouping;
mod home;
mod transaction;

#[derive(Eq, PartialEq, Hash, Clone, Default)]
pub enum View {
    #[default]
    Home,
    Budget,
    Grouping,
    Transaction,
}

#[derive(Default, Clone)]
pub struct Layout {
    pub view: View,
    home: home::Home,
    budget: budget::Budget,
    grouping: grouping::Grouping,
    transaction: transaction::Transaction,
    footer: common::Footer,
}

impl Layout {
    pub fn draw(&self, frame: &mut Frame) {
        let chunks = RLayout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(4)])
            .split(frame.area());

        match self.view {
            View::Home => {
                frame.render_widget(&self.home, chunks[0]);
            }
            View::Budget => {
                frame.render_widget(&self.budget, chunks[0]);
            }
            View::Grouping => {
                frame.render_widget(&self.grouping, chunks[0]);
            }
            View::Transaction => {
                frame.render_widget(&self.transaction, chunks[0]);
            }
        };
        frame.render_widget(&self.footer, chunks[1])
    }
}
