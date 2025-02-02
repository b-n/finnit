use finnit_abi::FrontendMessageSender;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    Frame,
};

use crate::traits::FinnitView;

#[derive(Clone)]
pub struct Budget {
    _sender: FrontendMessageSender,
}

impl Budget {
    fn render_budget(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self, area);
    }
}

impl FinnitView for Budget {
    fn with_sender(sender: FrontendMessageSender) -> Self {
        Self { _sender: sender }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.render_budget(frame, area);
    }
}

impl Widget for &Budget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Budget ".bold());
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        let text = Text::from(vec![Line::from(vec!["Hello budget".into()])]);
        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
