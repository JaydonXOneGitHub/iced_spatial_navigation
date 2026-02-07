use std::marker::PhantomData;

use iced::{Element, Padding, Task, widget::{Column, Row, container, scrollable}};

use crate::{TGridButton, direction::Direction, message::Message, position::Position};
use iced::widget::button as button_fn;

pub const SPACING: f32 = 10.0;

pub struct Grid<CustomMessage, GridButton: TGridButton> {
    pub locations: Vec<Vec<GridButton>>,
    pub position: Position,
    pub tile_size: f32,
    _marker: PhantomData<CustomMessage>
}

impl<CustomMessage, GridButton: TGridButton> Grid<CustomMessage, GridButton> {
    pub fn new(tile_size: f32) -> Self {
        return Self {
            locations: Vec::new(),
            position: Position::zero(),
            tile_size: tile_size,
            _marker: PhantomData
        };
    }
}

impl<CustomMessage: Clone, GridButton: TGridButton> Grid<CustomMessage, GridButton> {
    pub fn to_element(&self) -> Element<'_, Message<CustomMessage>> {
        let rows: Vec<Element<'_, Message<CustomMessage>>> = self.locations.iter()
        .enumerate()
        .map(
            |(r, row)| {
                let buttons: Vec<Element<'_, Message<CustomMessage>>> = row.iter()
                .enumerate()
                .map(|(c, btn_data)| {
                    return container(
                        button_fn(
                            container(
                                btn_data.inner().map(|_| Message::Nil)
                            )
                        )
                        .width(self.tile_size)
                        .height(self.tile_size)
                        .on_press(Message::ButtonPressed(Position::new(c, r)))
                    )
                    .center_x(self.tile_size)
                    .center_y(self.tile_size)
                    .id(btn_data.get_id().clone())
                    .into();
                })
                .collect();

                return Row::from_vec(buttons)
                .spacing(SPACING)
                .into();
            }
        )
        .collect();

        let column = Column::from_vec(rows)
        .spacing(SPACING);

        return container(
            scrollable(
                column
            )
        )
        .padding(Padding::new(SPACING))
        .into();
    }

    pub fn move_on_grid(&mut self, dir: Direction) -> Task<Message<CustomMessage>> {
        const OFFSET: usize = 1;

        match dir {
            Direction::Up => {
                if self.position.y > 0 {
                    self.position.y -= OFFSET;
                }

                let row_len: usize = self.locations[self.position.y].len();

                self.position.x = usize::clamp(
                    self.position.x, 
                    0, 
                    row_len - OFFSET
                );
            },
            Direction::Left => {
                if self.position.x > 0 {
                    self.position.x -= OFFSET;
                }
            },
            Direction::Down => {
                if self.position.y < self.locations.len() - OFFSET {
                    self.position.y += OFFSET;
                }

                let row_len: usize = self.locations[self.position.y].len();

                self.position.x = usize::clamp(
                    self.position.x, 
                    0, 
                    row_len - OFFSET
                );
            },
            Direction::Right => {
                let row_len: usize = self.locations[self.position.y].len();

                if self.position.x < row_len - OFFSET {
                    self.position.x += OFFSET;
                }
            }
        }

        println!("New grid position: {}", self.position);

        return Task::none();
    }
}