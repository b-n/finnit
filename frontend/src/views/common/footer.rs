use crate::FinnitView;
use finnit_abi::FrontendMessageSender;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Text},
    widgets::{Block, BorderType, Paragraph, Widget},
    Frame,
};

#[derive(Default, Clone)]
pub struct Footer {}

impl Footer {
    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self, area);
    }
}

impl FinnitView for Footer {
    fn with_sender(_sender: FrontendMessageSender) -> Self {
        Self::default()
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.render_footer(frame, area);
    }
}

impl Widget for &Footer {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let title = Line::from("Footer");

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title.left_aligned());

        let text = Text::from(vec![
            Line::from(vec![
                "<Home>".into(),
                "<Budget>".into(),
                " ".into(),
                "<Grouping>".into(),
                " ".into(),
                "<Transaction>".into(),
            ]),
            Line::from(vec![
                "<Info>".into(),
                " ".into(),
                "<Logs>".into(),
                " ".into(),
                " ".into(),
            ]),
        ]);

        Paragraph::new(text)
            .block(block)
            .left_aligned()
            .render(area, buffer);
    }
}
