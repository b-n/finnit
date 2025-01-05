use log::info;
use std::sync::mpsc::{self, Receiver, Sender};

use finnit_abi::{BackendMessage, FrontendMessage};

#[derive(Debug, Default)]
struct State {
    running: bool,
    exiting: bool,
}

pub struct App {
    rx: Option<Receiver<FrontendMessage>>,
    tx: Sender<BackendMessage>,
    state: State,
}

impl App {
    pub fn new() -> (Self, Receiver<BackendMessage>) {
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

    pub fn listen(&mut self, rx: Receiver<FrontendMessage>) {
        self.rx = Some(rx);
    }

    pub fn run(&mut self) {
        self.state.running = true;

        loop {
            if let Some(rx) = &self.rx {
                // Block until a message is received
                let message = rx.recv().unwrap();
                info!("Received: {:?}", message);
                match message {
                    FrontendMessage::Stop => {
                        self.state.exiting = true;
                        break;
                    }
                    FrontendMessage::Ping => {
                        self.tx.send(BackendMessage::Pong).unwrap();
                    }
                    FrontendMessage::GetTransactions => {
                        todo!()
                    }
                }
            }
        }
    }
}
