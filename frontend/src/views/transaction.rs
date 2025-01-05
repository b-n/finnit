use finnit_abi::FrontendMessage;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{block::Title, Block, Paragraph, Widget},
};
use std::sync::mpsc::Sender;

use crate::traits::FinnitView;

#[derive(Clone)]
pub struct Transaction {
    sender: Sender<FrontendMessage>,
}

impl FinnitView for Transaction {
    fn with_sender(sender: Sender<FrontendMessage>) -> Self {
        Self { sender }
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

        let text = Text::from(vec![Line::from(vec!["Hello transaction".into()])]);
        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
