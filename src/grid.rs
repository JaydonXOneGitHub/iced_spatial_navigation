use std::marker::PhantomData;

use iced::{Element, Padding, Task, Theme, widget::{Column, Row, button::{Status, Style}, container, scrollable}};

use crate::{TGridButton, direction::Direction, message::Message, position::Position};
use iced::widget::button as button_fn;

/// Ideal structure for member names is:
/// (grid, x, y, theme, status, grid_button)
pub type ButtonCallback<CustomMessage, GridButton> = Box<dyn Fn(
    &Grid<CustomMessage, GridButton>,
    usize, 
    usize, 
    &Theme,
    Status,
    &GridButton
) -> Style>;

/// The main struct for the [`Environment`]'s navigation
pub struct Grid<CustomMessage, GridButton: TGridButton> {
    pub locations: Vec<Vec<GridButton>>,
    pub position: Position,
    pub tile_size: f32,
    pub spacing: f32,
    pub button_callback: Option<ButtonCallback<CustomMessage, GridButton>>,
    _marker: PhantomData<CustomMessage>
}

impl<CustomMessage, GridButton: TGridButton> Grid<CustomMessage, GridButton> {
    pub fn new() -> Self {
        return Self {
            locations: Vec::new(),
            position: Position::zero(),
            tile_size: 0.0,
            spacing: 0.0,
            button_callback: Option::None,
            _marker: PhantomData
        };
    }
}

impl<CustomMessage: Clone, GridButton: TGridButton> Grid<CustomMessage, GridButton> {
    /// Set the tile size
    pub fn with_tile_size(mut self, tile_size: f32) -> Self {
        self.tile_size = tile_size;
        return self;
    }

    /// Set the grid spacing
    pub fn with_spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        return self;
    }

    /// Set the button style callback
    pub fn with_button_callback(mut self, callback: Option<ButtonCallback<CustomMessage, GridButton>>) -> Self {
        self.button_callback = callback;
        return self;
    }
}

impl<CustomMessage: Clone, GridButton: TGridButton> Grid<CustomMessage, GridButton> {
    /// Convert the data to the button elements
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
                        .style(move |t, s| -> Style {
                            return match &self.button_callback {
                                Option::Some(callback) => callback(
                                    self,
                                    r, c,
                                    t, s, btn_data
                                ),
                                Option::None => Style::default()
                            };
                        })
                        .on_press(Message::ButtonPressed(Position::new(c, r)))
                    )
                    .center_x(self.tile_size)
                    .center_y(self.tile_size)
                    .id(btn_data.get_id().clone())
                    .into();
                })
                .collect();

                return Row::from_vec(buttons)
                .spacing(self.spacing)
                .into();
            }
        )
        .collect();

        let column = Column::from_vec(rows)
        .spacing(self.spacing);

        return container(
            scrollable(
                column
            )
        )
        .padding(Padding::new(self.spacing))
        .into();
    }

    pub fn move_on_grid(&mut self, dir: Direction) -> Task<Message<CustomMessage>> {
        const OFFSET: usize = 1;

        match dir {
            Direction::Up => {
                if self.position.x > 0 {
                    self.position.x -= OFFSET;
                }
            },
            Direction::Left => {
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
            Direction::Down => {
                let row_len: usize = self.locations[self.position.y].len();

                if self.position.x < row_len - OFFSET {
                    self.position.x += OFFSET;
                }
            },
            Direction::Right => {
                if self.position.y < self.locations.len() - OFFSET {
                    self.position.y += OFFSET;
                }

                let row_len: usize = self.locations[self.position.y].len();

                self.position.x = usize::clamp(
                    self.position.x, 
                    0, 
                    row_len - OFFSET
                );
            }
        }

        println!("New grid position: {}", self.position);

        return Task::none();
    }
}