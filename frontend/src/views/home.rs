use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
};

#[derive(Default, Clone)]
pub struct Home {}

impl Widget for &Home {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Finnit ".bold());
        let instructions = Title::from(Line::from(vec!["Meh".bold().blue()]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Left)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);

        let text = Text::from(vec![Line::from(vec!["Hello world".into()])]);
        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
