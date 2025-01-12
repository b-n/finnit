use core::time::Duration;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use log::{error, info};
use ratatui::DefaultTerminal;
use std::io;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::sync::{Arc, RwLock};

use finnit_abi::{BackendMessage, FrontendMessage};

mod models;
mod traits;
mod views;

pub use traits::FinnitView;

#[derive(Debug, Default)]
pub(crate) struct State {
    running: bool,
    exiting: bool,
}

pub struct App {
    tx: Sender<FrontendMessage>,
    rx: Option<Receiver<BackendMessage>>,
    state: Arc<RwLock<State>>,
    layout: views::Layout,
}

impl App {
    pub fn new() -> (Self, Receiver<FrontendMessage>) {
        let state = Arc::new(RwLock::new(State::default()));
        let (tx, rx) = mpsc::channel();
        (
            App {
                tx: tx.clone(),
                rx: None,
                state,
                layout: views::Layout::with_sender(tx.clone()),
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
        {
            let mut state = self.state.write().unwrap();
            state.running = true;
        }
        self.tx.send(FrontendMessage::Ping).unwrap();

        while self.running() {
            self.handle_backend_events();
            self.handle_ui_events()?;

            // Render UI
            terminal.draw(|frame| {
                self.layout.draw(frame, frame.area());
            })?;
        }

        self.tx.send(FrontendMessage::Stop).unwrap();
        Ok(())
    }

    fn running(&self) -> bool {
        !self.state.read().unwrap().exiting
    }

    fn handle_backend_events(&mut self) {
        if let Some(rx) = &self.rx {
            // Get messages from the backend
            let message = match rx.try_recv() {
                Ok(message) => Some(message),
                Err(TryRecvError::Empty) => None,
                Err(TryRecvError::Disconnected) => {
                    error!("Backend disconnected");
                    let mut state = self.state.write().unwrap();
                    state.exiting = true;
                    None
                }
            };

            if let Some(message) = message {
                info!("Received: {:?}", message);
                match message {
                    BackendMessage::Pong => {}
                    BackendMessage::Transactions(t) => {
                        self.layout.set_transactions(t);
                    }
                }
            }
        }
    }

    fn handle_ui_events(&mut self) -> io::Result<()> {
        let event = event::poll(Duration::from_millis(10));

        if let Ok(true) = event {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Char('q') => {
                            let mut state = self.state.write().unwrap();
                            state.exiting = true;
                        }
                        KeyCode::Char('b') => self.layout.set_view(views::View::Budget),
                        KeyCode::Char('g') => self.layout.set_view(views::View::Grouping),
                        KeyCode::Char('t') => self.layout.set_view(views::View::Transaction),
                        KeyCode::Char('?') => self.layout.toggle_help(),
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
