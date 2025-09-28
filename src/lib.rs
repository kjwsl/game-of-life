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
    pub is_infinite: bool,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::Dead; width * height],
            is_infinite: false,
        }
    }

    pub fn set_infinite(&mut self, is_infinite: bool) {
        self.is_infinite = is_infinite;
    }

    pub fn from_str(input: &str) -> Self {
        let height = input.lines().count();
        let width = input.lines().map(|l| l.len()).max().unwrap();

        let mut canvas = Self::new(width, height);

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == 'O' {
                    canvas.set_cell(x, y, Cell::Alive);
                }
            }
        }

        canvas
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
                let cell = match self.is_infinite {
                    true => {
                        let new_x = (x as i32 + dx + self.width as i32) % self.width as i32;
                        let new_y = (y as i32 + dy + self.height as i32) % self.height as i32;
                        self.get_cell(new_x as usize, new_y as usize)
                    }
                    false => {
                        if x as i32 + dx < 0 || x as i32 + dx >= self.width as i32 {
                            None
                        } else if y as i32 + dy < 0 || y as i32 + dy >= self.height as i32 {
                            None
                        } else {
                            self.get_cell((x as i32 + dx) as usize, (y as i32 + dy) as usize)
                        }
                    }
                };
                if cell == Some(Cell::Alive) {
                    count += 1;
                }
            }
        }
        count
    }
}
