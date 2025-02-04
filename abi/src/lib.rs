use std::sync::mpsc::Sender;
use tz::DateTime;

pub type FrontendMessageSender = Sender<FrontendMessage>;

#[derive(Debug)]
pub enum FrontendMessage {
    Stop,
    Ping,
    GetTransactions,
}

#[derive(Debug)]
pub enum BackendMessage {
    Pong,
    Transactions(Vec<Transaction>),
}

// Currency is dealt with the smallest unit of the currency. For the Euro, this is the cent.
// This is done to avoid floating point arithmetic.
#[derive(Debug, Clone, Copy)]
pub struct Currency(i64);

impl std::ops::Deref for Currency {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Currency> for String {
    fn from(currency: Currency) -> String {
        let characteristic = *currency / 100;
        let mantissa = *currency % 100;
        format!("{}.{:02}", characteristic, mantissa)
    }
}

impl From<i64> for Currency {
    fn from(value: i64) -> Currency {
        Currency(value)
    }
}

#[derive(Debug)]
pub struct Transaction {
    pub id: String,
    pub account: String,
    pub datetime: DateTime,
    pub amount: Currency,
    pub source: String,
    pub target: String,
    pub description: String,
}
