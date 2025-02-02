use crate::FinnitView;
use finnit_abi::FrontendMessageSender;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Stylize,
    text::{Line, Text},
    widgets::{Paragraph, Widget},
    Frame,
};

#[derive(Default, Clone)]
pub struct Header {
    title: String,
}

impl Header {
    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget(self, area);
    }

    pub fn statistics<'a>(&self) -> Text<'a> {
        let kvp = |key: &'a str, value: &'a str| {
            vec![
                format!("{:<15}", format!("{key}:")).bold().light_cyan(),
                " ".into(),
                value.into(),
            ]
            .into()
        };

        Text::from(vec![
            kvp("Year", "2025"),
            kvp("YTD", "EUR 100"),
            kvp("Classified", "EUR 100"),
            kvp("Un-classified", "EUR 100"),
        ])
    }

    pub fn help<'a>(&self) -> Text<'a> {
        let kvp = |key: &'a str, value: &'a str| {
            vec![format!("{:>9}", key).red(), " ".into(), value.into()].into()
        };
        Text::from(vec![
            kvp("q", "Quit"),
            kvp("?", "Help"),
            kvp("b", "Budget"),
            kvp("g", "Grouping"),
            kvp("t", "Transaction"),
            kvp("<ctrl+d>", "Delete"),
        ])
    }

    pub fn logo(&self) -> Text {
        Text::from(vec![
            Line::from("     ####$$####    "),
            Line::from("  ##\"####$$###### 9"),
            Line::from("#########$$####### "),
            Line::from("#######FINNIT##### "),
            Line::from("   ##############  "),
            Line::from("     ###########   "),
            Line::from("      ##    ##     "),
        ])
    }
}

impl FinnitView for Header {
    fn with_sender(_sender: FrontendMessageSender) -> Self {
        Self::default()
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        self.render_header(frame, area)
    }
}

impl Widget for &Header {
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(40),
                Constraint::Min(1),
                Constraint::Length(19),
            ])
            .split(area);

        let (stats, help, logo) = (chunks[0], chunks[1], chunks[2]);

        Paragraph::new(self.statistics()).render(stats, buffer);
        Paragraph::new(self.help()).render(help, buffer);
        Paragraph::new(self.logo()).render(logo, buffer);
    }
}
