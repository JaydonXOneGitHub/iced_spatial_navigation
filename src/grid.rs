use std::marker::PhantomData;

use iced::{Element, Padding, Task, Theme, widget::{Column, Id, Row, button::Status, container, scrollable}};
use vector_x::Vector2;

use crate::{TGridButton, direction::Direction, message::Message, position::Position};
use iced::widget::button as button_fn;

use iced::widget::button::Style as ButtonStyle;
use iced::widget::container::Style as ContainerStyle;

/// Ideal structure for member names is:
/// (grid, x, y, theme, status, grid_button)
pub type StyleCallback<CustomMessage, GridButton> = Box<dyn Fn(
    &Grid<CustomMessage, GridButton>,
    usize, 
    usize, 
    &Theme,
    &GridButton
) -> ContainerStyle>;

pub type ButtonStyleCallback<CustomMessage, GridButton> = Box<dyn Fn(
    &Grid<CustomMessage, GridButton>,
    usize, 
    usize, 
    &Theme,
    Status,
    &GridButton
) -> ButtonStyle>;

pub type CullingCallback<CustomMessage, GridButton> = Box<dyn Fn(
    &Grid<CustomMessage, GridButton>,
    usize
) -> bool>;

/// The main struct for the [`Environment`]'s navigation
pub struct Grid<CustomMessage, GridButton: TGridButton> {
    pub locations: Vec<Vec<GridButton>>,
    pub position: Position,
    pub tile_size: Vector2<f32>,
    pub spacing: Vector2<f32>,
    pub padding: f32,
    pub container_style_callback: Option<StyleCallback<CustomMessage, GridButton>>,
    pub button_style_callback: Option<ButtonStyleCallback<CustomMessage, GridButton>>,
    pub culling_callback: Option<CullingCallback<CustomMessage, GridButton>>,
    pub grid_size: Option<Vector2<f32>>,
    pub scroll_id: Id,
    pub button_container_size: f32,
    _marker: PhantomData<CustomMessage>
}

impl<CustomMessage, GridButton: TGridButton> Grid<CustomMessage, GridButton> {
    pub fn new() -> Self {
        return Self {
            locations: Vec::new(),
            position: Position::zero(),
            tile_size: Vector2::new(0.0, 0.0),
            spacing: Vector2::new(0.0, 0.0),
            container_style_callback: Option::None,
            culling_callback: Option::None,
            button_style_callback: Option::None,
            grid_size: Option::None,
            padding: 0.0,
            scroll_id: Id::unique(),
            button_container_size: 0.0,
            _marker: PhantomData
        };
    }
}

impl<CustomMessage: Clone, GridButton: TGridButton> Grid<CustomMessage, GridButton> {
    /// Set the tile size
    pub fn with_tile_size(self, tile_size: f32) -> Self {
        return self.with_x_tile_size(tile_size).with_y_tile_size(tile_size);
    }

    /// Set the grid spacing
    pub fn with_spacing(self, spacing: f32) -> Self {
        return self.with_x_spacing(spacing).with_y_spacing(spacing);
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        return self;
    }

    pub fn with_button_container_size(mut self, button_container_size: f32) -> Self {
        self.button_container_size = button_container_size;
        return self;
    }

    pub fn with_x_spacing(mut self, x_spacing: f32) -> Self {
        self.spacing.one = x_spacing;
        return self;
    }

    pub fn with_y_spacing(mut self, y_spacing: f32) -> Self {
        self.spacing.two = y_spacing;
        return self;
    }

    pub fn with_x_tile_size(mut self, x_tile_size: f32) -> Self {
        self.tile_size.one = x_tile_size;
        return self;
    }

    pub fn with_y_tile_size(mut self, y_tile_size: f32) -> Self {
        self.tile_size.two = y_tile_size;
        return self;
    }

    pub fn with_container_style_callback(
        mut self, 
        callback: Option<StyleCallback<CustomMessage, GridButton>>
    ) -> Self {
        self.container_style_callback = callback;
        return self;
    }

    pub fn with_button_style_callback(
        mut self, 
        callback: Option<ButtonStyleCallback<CustomMessage, GridButton>>
    ) -> Self {
        self.button_style_callback = callback;
        return self;
    }

    pub fn with_culling_callback(
        mut self, 
        callback: Option<CullingCallback<CustomMessage, GridButton>>
    ) -> Self {
        self.culling_callback = callback;
        return self;
    }

    pub fn with_grid_size(
        mut self,
        grid_size: impl Into<Option<Vector2<f32>>>
    ) -> Self {
        self.grid_size = grid_size.into();
        return self;
    }

    fn is_okay_to_render_row(&self, row_index: usize) -> bool {
        return match &self.culling_callback {
            Option::Some(callback) => callback(self, row_index),
            Option::None => true
        };
    }
}

impl<CustomMessage: Clone, GridButton: TGridButton> Grid<CustomMessage, GridButton> {
    /// Convert the data to the button elements
    pub fn to_element(&self) -> Element<'_, Message<CustomMessage>> {
        return self.to_element_advanced(true);
    }

    pub fn to_element_advanced(&self, with_scrollable: bool) -> Element<'_, Message<CustomMessage>> {
        let e1 = self.locations.iter().enumerate();

        let mut rows: Vec<Element<'_, Message<CustomMessage>>> = Vec::with_capacity(self.locations.len());

        for (r, row) in e1 {
            if !self.is_okay_to_render_row(r) {
                continue;
            }

            let e2 = row.iter().enumerate();

            let mut buttons: Vec<Element<'_, Message<CustomMessage>>> = Vec::with_capacity(row.len()); 

            for (c, button) in e2 {
                let elem: Element<'_, Message<CustomMessage>> = container(
                        button_fn(
                            container(
                                button.inner().map(|_| Message::Nil)
                            )
                        )
                        .width(self.tile_size.one - (
                            self.button_container_size * 2.0
                        ))
                        .height(self.tile_size.two - (
                            self.button_container_size * 2.0
                        ))
                        .on_press(Message::ButtonPressed(Position::new(c, r)))
                        .style(move |t, s| {
                            return match &self.button_style_callback {
                                Option::Some(cb) => cb(
                                    self, c, r, 
                                    t, s, button
                                ),
                                Option::None => ButtonStyle::default()
                            };
                        })
                    )
                    .center_x(self.tile_size.one)
                    .center_y(self.tile_size.two)
                    .id(button.get_id().clone())
                    .style(move |t| -> ContainerStyle {
                            return match &self.container_style_callback {
                                Option::Some(callback) => callback(
                                    self,
                                    c, r,
                                    t, button
                                ),
                                Option::None => ContainerStyle::default()
                            };
                        })
                    .into();

                buttons.push(elem);
            }

            rows.push(
                Row::from_vec(buttons)
                .spacing(self.spacing.one)
                .into()
            );
        }

        let column = Column::from_vec(rows)
        .spacing(self.spacing.two);

        let content: Element<'_, Message<CustomMessage>> = if with_scrollable {
            scrollable(
                column
            )
            .id(self.scroll_id.clone())
            .height(
                match &self.grid_size {
                    Option::Some(gs) => {
                        gs.two
                    },
                    Option::None => 450.0
                }
            )
            .width(
                match &self.grid_size {
                    Option::Some(gs) => {
                        gs.one
                    },
                    Option::None => 800.0
                }
            ).into()
        } else {
            column.into()
        };

        return container(content)
        .padding(Padding::new(self.padding))
        .into();
    }

    pub fn move_on_grid(
        &mut self, 
        dir: Direction
    ) -> Task<Message<CustomMessage>> {
        return self.move_on_grid_with_callback(dir, 
            |_, _, _, _| Task::none()
        );
    }

    /// Callback corresponds to (grid, dir, grid_button, positions) 
    /// (positions.one is the old and positions.two is new)
    pub fn move_on_grid_with_callback<F>(
        &mut self, dir: Direction, callback: F
    ) -> Task<Message<CustomMessage>>
    where
        F: FnOnce(
            &Grid<CustomMessage, GridButton>, 
            Direction, 
            &GridButton,
            Vector2<Position>
        ) -> Task<Message<CustomMessage>> 
    {
        const OFFSET: usize = 1;
        
        let old_position: Position = self.position;

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

        return callback(
            &self, dir, 
            &self.locations[self.position.y][self.position.x],
            Vector2::new(old_position, self.position)
        );
    }
}