use iced::{Event, widget::Id};

use crate::{direction::Direction, position::Position};

#[derive(Clone)]
pub enum Message<CustomMessage: Clone> {
    ButtonPressed(Position),
    Event(Event),
    ItemSelected(Id),
    BoundsFound(),
    Navigate(Direction),
    Select,
    Back,
    Custom(CustomMessage),
    Nil
}

impl<CustomMessage: Clone> Message<CustomMessage> {
    pub fn get_custom(self) -> Option<CustomMessage> {
        return match self {
            Self::Custom(cm) => Option::Some(cm),
            _ => Option::None
        };
    }
}