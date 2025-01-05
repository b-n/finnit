use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::Stylize,
    style::Style,
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

#[derive(Default, Clone)]
pub struct Help {
    title: String,
}

impl Help {
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }
}

impl Widget for &Help {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        Clear.render(area, buffer);

        let content: Text = Line::from("Your mother").into();

        let block = Block::new()
            .title("Finnit Help")
            .borders(Borders::ALL)
            .border_style(Style::default().blue());

        Paragraph::new(content)
            .wrap(Wrap { trim: false })
            .block(block)
            .render(area, buffer);
    }
}
