use crate::{TGridButton, grid::Grid};

pub struct Environment<CustomMessage: Clone, GridButton: TGridButton> {
    pub grid: Grid<CustomMessage, GridButton>,
    pub render_target: (f32, f32),
    pub window_size: (f32, f32)
}

impl<CustomMessage: Clone, GridButton: TGridButton> Environment<CustomMessage, GridButton> {
    pub fn new(grid_fn: impl FnOnce() -> Grid<CustomMessage, GridButton>) -> Self {
        let ws: (f32, f32) = (1280.0, 720.0);

        let grid = grid_fn();

        return Self {
            grid: grid,
            render_target: ws.clone(),
            window_size: ws.clone()
        };
    }
}

impl<CustomMessage: Clone, GridButton: TGridButton> Default for Environment<CustomMessage, GridButton> {
    fn default() -> Self {
        return Self::new(|| Grid::new(150.0));
    }
}

impl<CustomMessage: Clone, GridButton: TGridButton> Environment<CustomMessage, GridButton> {
    pub fn get_grid(&self) -> &Grid<CustomMessage, GridButton> {
        return &self.grid;
    }

    pub fn get_scale_factor(&self) -> f32 {
        return f32::min(
            self.window_size.0 / self.render_target.0,
            self.window_size.1 / self.render_target.1
        );
    }

    pub fn get_grid_mut(&mut self) -> &mut Grid<CustomMessage, GridButton> {
        return &mut self.grid;
    }
}