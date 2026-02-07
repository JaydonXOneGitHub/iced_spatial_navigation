use iced::{Element, widget::Id};

pub trait TGridButton {
    fn inner(&self) -> Element<'_, ()>;
    fn get_id(&self) -> Id;
}