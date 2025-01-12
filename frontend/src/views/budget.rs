use finnit_abi::FrontendMessageSender;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{block::Title, Block, Paragraph, Widget},
    Frame,
};

use crate::traits::FinnitView;

#[derive(Clone)]
pub struct Budget {
    sender: FrontendMessageSender,
}

impl FinnitView for Budget {
    fn with_sender(sender: FrontendMessageSender) -> Self {
        Self { sender }
    }

    fn draw(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self, area);
    }
}

impl Widget for &Budget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Budget ".bold());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_set(border::THICK);

        let text = Text::from(vec![Line::from(vec!["Hello budget".into()])]);
        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
