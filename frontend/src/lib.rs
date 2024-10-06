use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use log::{error, info};
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
    DefaultTerminal, Frame,
};
use std::io;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::time::Duration;

use finnit_abi::{BackendMessage, FrontendMessage};

#[derive(Debug, Default)]
struct State {
    running: bool,
    exiting: bool,
}

pub struct App {
    tx: Sender<FrontendMessage>,
    rx: Option<Receiver<BackendMessage>>,
    state: State,
}

impl App {
    pub fn new() -> (Self, Receiver<FrontendMessage>) {
        let (tx, rx) = mpsc::channel();
        (
            App {
                tx,
                rx: None,
                state: State::default(),
            },
            rx,
        )
    }

    pub fn listen(&mut self, rx: Receiver<BackendMessage>) {
        self.rx = Some(rx)
    }

    pub fn run(&mut self) -> io::Result<()> {
        let terminal = ratatui::init();
        let res = self.ui_loop(terminal);
        ratatui::restore();
        res
    }

    fn ui_loop(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        self.state.running = true;
        self.tx.send(FrontendMessage::Ping).unwrap();

        while !self.state.exiting {
            self.handle_backend_events();
            self.handle_ui_events()?;
            // Render UI
            terminal.draw(|frame| self.draw_ui(frame))?;
        }

        self.tx.send(FrontendMessage::Stop).unwrap();
        Ok(())
    }

    fn handle_backend_events(&mut self) {
        if let Some(rx) = &self.rx {
            // Get messages from the backend
            let message = match rx.try_recv() {
                Ok(message) => Some(message),
                Err(TryRecvError::Empty) => None,
                Err(TryRecvError::Disconnected) => {
                    error!("Backend disconnected");
                    self.state.exiting = true;
                    None
                }
            };

            if let Some(message) = message {
                info!("Received: {:?}", message);
                match message {
                    BackendMessage::Pong => {}
                }
            }
        }
    }

    fn handle_ui_events(&mut self) -> io::Result<()> {
        let event = event::poll(Duration::from_millis(10));

        if let Ok(true) = event {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    if let KeyCode::Char('q') = key_event.code {
                        self.state.exiting = true;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn draw_ui(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from("Finnit".bold());
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
