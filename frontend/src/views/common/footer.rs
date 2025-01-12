use crate::traits::FinnitView;
use finnit_abi::FrontendMessageSender;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::{Line, Text},
    widgets::{block::Title, Block, BorderType, Paragraph, Widget},
    Frame,
};

#[derive(Default, Clone)]
pub struct Footer {}

impl FinnitView for Footer {
    fn with_sender(_sender: FrontendMessageSender) -> Self {
        Self::default()
    }

    fn draw(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self, area);
    }
}

impl Widget for &Footer {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let title = Title::from("Footer").alignment(Alignment::Left);

        let block = Block::bordered()
            .border_type(BorderType::Rounded)
            .title(title);

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
            .alignment(Alignment::Left)
            .render(area, buffer);
    }
}
