#[derive(Debug, Clone)]
pub enum FrontendMessage {
    Stop,
    Ping,
}

#[derive(Debug, Clone)]
pub enum BackendMessage {
    Pong,
}
