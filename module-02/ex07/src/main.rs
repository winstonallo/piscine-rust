use ftkit;

enum ParseError {
    InvalidWidth { arg: &'static str },
    InvalidHeight { arg: &'static str },
    InvalidPercentage { arg: &'static str },
    TooManyArguments,
    NotEnoughArguments,
}

#[derive(PartialEq, Clone)]
enum Cell {
    Dead,
    Alive,
}

impl Cell {
    pub fn is_alive(self) -> bool {
        self == Cell::Alive
    }

    pub fn is_dead(self) -> bool {
        self == Cell::Dead
    }
}

struct Board {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Board {
    fn new(width: usize, height: usize, percentage: u32) -> Self {
        let n_cells_total = width * height;
        let mut cells = vec![Cell::Dead; n_cells_total];
    
        let n_alive_cells = (n_cells_total * percentage as usize / 100).min(n_cells_total);

        for _ in 0..n_alive_cells {
            let idx = ftkit::random_number(0..=n_cells_total as i32) as usize;
            cells[idx] = Cell::Alive;
        }
        
        Board {
            width,
            height,
            cells,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
