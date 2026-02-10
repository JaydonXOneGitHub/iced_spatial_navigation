use iced::{Event, widget::{Id, scrollable::Viewport}};

use crate::{direction::Direction, position::Position};

#[derive(Clone)]
pub enum Message<CustomMessage: Clone> {
    ButtonPressed(Position),
    Custom(CustomMessage),
    Event(Event),
    ItemSelected(Id),
    Navigate(Direction),
    GotViewport(Viewport),
    BoundsFound(),
    Select,
    Back,
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