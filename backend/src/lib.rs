use log::info;
use std::sync::mpsc::{self, Receiver, Sender};
use tz::{DateTime, TimeZone};

use finnit_abi::{BackendMessage, FrontendMessage, Transaction};

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
                        let mut transactions = vec![];
                        for i in 0..150 {
                            transactions.push(Transaction {
                                id: format!("{i}"),
                                account: format!("{i}"),
                                datetime: DateTime::now(TimeZone::utc().as_ref()).unwrap(),
                                description: format!("Transaction {}", i),
                                source: format!("NL00 IBAN 0000 {i:04} 00"),
                                target: format!("NL00 IBAN {i:04} 0000 00"),
                                amount: (i * 7).into(),
                            });
                        }
                        self.tx
                            .send(BackendMessage::Transactions(transactions))
                            .unwrap();
                    }
                }
            }
        }
    }
}
