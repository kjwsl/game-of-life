#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Cell {
    Alive,
    Dead,
}

#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::Dead; width * height],
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Option<Cell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.cells[y * self.width + x])
        }
    }

    pub fn set_cell(&mut self, x: usize, y: usize, cell: Cell) {
        self.cells[y * self.width + x] = cell;
    }

    pub fn tick(&mut self) {
        let mut cells_clone = self.cells.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let alive_neighbors = self.count_alive_neighbors(x, y);
                let cell = self.get_cell(x, y).unwrap();
                if cell == Cell::Alive {
                    if !(2..=3).contains(&alive_neighbors) {
                        cells_clone[y * self.width + x] = Cell::Dead;
                    }
                } else if alive_neighbors == 3 {
                    cells_clone[y * self.width + x] = Cell::Alive;
                }
            }
        }
        self.cells = cells_clone;
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!(
                    "{}",
                    if self.get_cell(x, y) == Some(Cell::Alive) {
                        "O"
                    } else {
                        "."
                    }
                );
            }
            println!();
        }
    }

    fn count_alive_neighbors(&self, x: usize, y: usize) -> u32 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if self.get_cell((x as i32 + dx) as usize, (y as i32 + dy) as usize)
                    == Some(Cell::Alive)
                {
                    count += 1;
                }
            }
        }
        count
    }
}
