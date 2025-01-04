use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    text::{Line, Text},
    widgets::{block::Title, Block, BorderType, Paragraph, Widget},
};

#[derive(Default, Clone)]
pub struct Footer {
    title: String,
}

impl Footer {
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }
}

impl Widget for &Footer {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let title = Title::from(self.title.clone()).alignment(Alignment::Left);

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
