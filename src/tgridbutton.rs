use std::any::Any;

use iced::{Element, widget::Id};

pub trait TGridButton {
    fn inner(&self, external_data: Option<&dyn Any>) -> Element<'_, ()>;
    fn get_id(&self) -> Id;
}