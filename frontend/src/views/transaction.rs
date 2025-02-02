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

use crate::{traits::TableRow, FinnitView, InputEvent};

#[derive(Clone)]
struct Styles {
    selected_row: Style,
    selected_column: Style,
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            selected_row: Style::default().fg(Color::White).bg(Color::Blue),
            selected_column: Style::default().fg(Color::White).bg(Color::Blue),
        }
    }
}

#[derive(Clone)]
pub struct Transaction {
    sender: FrontendMessageSender,
    transactions: Vec<finnit_abi::Transaction>,
    table_state: TableState,
    scroll_state: ScrollbarState,
    styles: Styles,
}

impl Transaction {
    pub fn set_transactions(&mut self, transactions: Vec<finnit_abi::Transaction>) {
        self.transactions = transactions;
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
            (Some(i), ScrollDirection::Forward) => Some(i.saturating_add(offset)),
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

        let table = Table::new(
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
        .block(block)
        .row_highlight_style(self.styles.selected_row)
        .column_highlight_style(self.styles.selected_column);

        frame.render_stateful_widget(table, area, &mut self.table_state);
    }

    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("B"))
            .end_symbol(Some("E"));

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
