use crate::FinnitView;
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

#[derive(Clone)]
pub struct Grouping {
    _sender: FrontendMessageSender,
}

impl Grouping {
    fn render_grouping(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self, area);
    }
}

impl FinnitView for Grouping {
    fn with_sender(sender: FrontendMessageSender) -> Self {
        Self { _sender: sender }
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.render_grouping(frame, area);
    }
}

impl Widget for &Grouping {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Grouping ".bold());
        let block = Block::bordered()
            .title_top(title.centered())
            .border_set(border::THICK);

        let text = Text::from(vec![Line::from(vec!["Hello grouping".into()])]);
        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
