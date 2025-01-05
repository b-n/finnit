use crate::traits::FinnitView;
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

#[derive(Clone)]
pub struct Grouping {
    sender: Sender<FrontendMessage>,
}

impl FinnitView for Grouping {
    fn with_sender(sender: Sender<FrontendMessage>) -> Self {
        Self { sender }
    }
}

impl Widget for &Grouping {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Grouping ".bold());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK);

        let text = Text::from(vec![Line::from(vec!["Hello grouping".into()])]);
        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
