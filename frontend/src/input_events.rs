use crossterm::event::{Event, KeyCode, KeyEventKind};

use crate::views::View;
use std::convert::TryFrom;

pub enum InputEvent {
    Exit,
    ChangeView(View),
    ToggleModal,
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<Event> for InputEvent {
    type Error = &'static str;
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => Ok(InputEvent::Exit),
                    KeyCode::Char('b') => Ok(InputEvent::ChangeView(View::Budget)),
                    KeyCode::Char('g') => Ok(InputEvent::ChangeView(View::Grouping)),
                    KeyCode::Char('t') => Ok(InputEvent::ChangeView(View::Transaction)),
                    KeyCode::Char('?') => Ok(InputEvent::ToggleModal),
                    KeyCode::Char('k') => Ok(InputEvent::Up),
                    KeyCode::Char('j') => Ok(InputEvent::Down),
                    KeyCode::Char('h') => Ok(InputEvent::Left),
                    KeyCode::Char('l') => Ok(InputEvent::Right),
                    KeyCode::Up => Ok(InputEvent::Up),
                    KeyCode::Down => Ok(InputEvent::Down),
                    KeyCode::Left => Ok(InputEvent::Left),
                    KeyCode::Right => Ok(InputEvent::Right),
                    _ => Err("Not a key code we care about"),
                }
            }
            _ => Err("Not a key event we care about"),
        }
    }
}
