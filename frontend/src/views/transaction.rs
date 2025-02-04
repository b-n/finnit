use finnit_abi::{FrontendMessage, FrontendMessageSender};
use log::info;
use ratatui::{
    layout::{Constraint, Margin, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{
        Block, Cell, Row, ScrollDirection, Scrollbar, ScrollbarOrientation, ScrollbarState, Table,
        TableState,
    },
    Frame,
};
use std::sync::mpsc::Sender;

use crate::{FinnitView, InputEvent};

#[derive(Clone)]
struct Styles {
    selected_row: Style,
    selected_column: Style,
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            selected_row: Style::default().fg(Color::White).bg(Color::Blue),
            selected_column: Style::default().fg(Color::White).bg(Color::Red),
        }
    }
}

struct TransactionRow(finnit_abi::Transaction);

impl From<finnit_abi::Transaction> for TransactionRow {
    fn from(transaction: finnit_abi::Transaction) -> Self {
        Self(transaction)
    }
}

impl std::ops::Deref for TransactionRow {
    type Target = finnit_abi::Transaction;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TransactionRow {
    fn row<'a>(&self) -> Row<'a> {
        let row = vec![
            Cell::from(self.id.clone()),
            Cell::from(self.account.clone()),
            Cell::from(self.datetime.to_string()),
            Cell::from(self.source.clone()),
            Cell::from(self.target.clone()),
            Cell::from(String::from(self.amount)),
            Cell::from(self.description.clone()),
        ];

        row.into_iter().collect::<Row>().height(1)
    }
}

pub struct Transaction {
    sender: FrontendMessageSender,
    transactions: Vec<TransactionRow>,
    table_state: TableState,
    scroll_state: ScrollbarState,
    styles: Styles,
}

impl Transaction {
    pub fn set_transactions(&mut self, transactions: Vec<finnit_abi::Transaction>) {
        self.transactions = transactions.into_iter().map(TransactionRow::from).collect();
        self.scroll_state = self.scroll_state.content_length(self.transactions.len());
        self.table_state.select(Some(0));
        self.scroll_state = self.scroll_state.position(0);
    }

    fn move_row(&mut self, offset: usize, direction: ScrollDirection) {
        let row = match (self.table_state.selected(), direction) {
            (Some(i), ScrollDirection::Backward) => i.saturating_sub(offset),
            (Some(i), ScrollDirection::Forward) => i.saturating_add(offset),
            (None, ScrollDirection::Forward) => offset,
            (None, ScrollDirection::Backward) => self.transactions.len().saturating_sub(offset),
        };

        self.table_state.select(Some(row));
        self.scroll_state = self.scroll_state.position(row);
    }

    fn move_col(&mut self, offset: usize, direction: ScrollDirection) {
        let col = match (self.table_state.selected_column(), direction) {
            (Some(i), ScrollDirection::Backward) => {
                if i == 0 {
                    None
                } else {
                    Some(i.saturating_sub(offset))
                }
            }
            (Some(i), ScrollDirection::Forward) => {
                if i >= 6 {
                    None
                } else {
                    Some(i.saturating_add(offset))
                }
            }
            (None, ScrollDirection::Forward) => Some(offset - 1),
            (None, ScrollDirection::Backward) => {
                Some(self.transactions.len().saturating_sub(offset))
            }
        };

        self.table_state.select_column(col);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let title = Line::from(" Transaction ".bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let header = [
            "ID",
            "Account",
            "Date",
            "From",
            "To",
            "Amount",
            "Description",
        ]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .height(1);

        let rows = self.transactions.iter().map(|data| data.row());

        let table = Table::new(
            rows,
            [
                Constraint::Length(6),
                Constraint::Length(10),
                Constraint::Length(15),
                Constraint::Length(24),
                Constraint::Length(24),
                Constraint::Length(11),
                Constraint::Length(30),
            ],
        )
        .header(header)
        .block(block)
        .row_highlight_style(self.styles.selected_row)
        .column_highlight_style(self.styles.selected_column);

        frame.render_stateful_widget(table, area, &mut self.table_state);
    }

    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("⇑"))
            .end_symbol(Some("⇓"));

        frame.render_stateful_widget(
            scrollbar,
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.scroll_state,
        );
    }
}

impl FinnitView for Transaction {
    fn with_sender(sender: Sender<FrontendMessage>) -> Self {
        Self {
            sender,
            table_state: TableState::default(),
            scroll_state: ScrollbarState::new(0),
            transactions: vec![],
            styles: Styles::default(),
        }
    }

    fn on_activate(&mut self) {
        self.sender.send(FrontendMessage::GetTransactions).unwrap();
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.render_table(frame, area);
        self.render_scrollbar(frame, area);
    }

    fn on_input_event(&mut self, event: InputEvent) {
        info!("Transaction: {event:?}");
        match event {
            InputEvent::Up => self.move_row(1, ScrollDirection::Backward),
            InputEvent::Down => self.move_row(1, ScrollDirection::Forward),
            InputEvent::Left => self.move_col(1, ScrollDirection::Backward),
            InputEvent::Right => self.move_col(1, ScrollDirection::Forward),
            _ => {}
        }
    }
}
