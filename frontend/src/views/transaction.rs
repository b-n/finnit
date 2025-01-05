use finnit_abi::FrontendMessage;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::Stylize,
    symbols::border,
    text::Text,
    widgets::{block::Title, Block, Cell, Row, ScrollbarState, Table, TableState, Widget},
};
use std::sync::mpsc::Sender;

use crate::traits::{FinnitView, TableRow};

#[derive(Clone)]
pub struct Transaction {
    sender: Sender<FrontendMessage>,
    transactions: Vec<finnit_abi::Transaction>,
    state: TableState,
    scroll_state: ScrollbarState,
}

impl Transaction {
    pub fn set_transactions(&mut self, transactions: Vec<finnit_abi::Transaction>) {
        self.transactions = transactions;
    }
}

impl FinnitView for Transaction {
    fn with_sender(sender: Sender<FrontendMessage>) -> Self {
        Self {
            sender,
            state: TableState::default(),
            scroll_state: ScrollbarState::new(0),
            transactions: vec![],
        }
    }

    fn on_activate(&mut self) {
        self.sender.send(FrontendMessage::GetTransactions).unwrap();
    }
}

impl Widget for &Transaction {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Transaction ".bold());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK);

        let block_inner = block.inner(area);
        block.render(area, buf);

        let header = ["ID", "Amount", "Type", "Date", "Description"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .height(1);

        let rows = self.transactions.iter().map(|data| {
            let item = data.row();
            item.into_iter()
                .map(|content| Cell::from(Text::from(content)))
                .collect::<Row>()
                .height(1)
        });

        Table::new(
            rows,
            [
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Length(5),
            ],
        )
        .header(header)
        .render(block_inner, buf);
    }
}
