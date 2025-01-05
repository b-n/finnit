use tz::DateTime;

#[derive(Debug, Clone)]
pub enum FrontendMessage {
    Stop,
    Ping,
    GetTransactions,
}

#[derive(Debug, Clone)]
pub enum BackendMessage {
    Pong,
    Transactions(Vec<Transaction>),
}

// Currency is dealt with the smallest unit of the currency. For the Euro, this is the cent.
// This is done to avoid floating point arithmetic.
pub type Currency = i64;

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub account: String,
    pub datetime: DateTime,
    pub amount: Currency,
    pub source: String,
    pub target: String,
    pub description: String,
}
