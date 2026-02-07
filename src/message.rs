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