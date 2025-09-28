pub struct Board {
    cells: [[Option<char>; 3]; 3]
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [[None; 3]; 3]
        }
    }

    pub fn display(&self) {
        for row in &self.cells {
            for cell in row {
                match cell {
                    Some(symbol) => print!("| {} |", symbol),
                    None => print!("| . |"),
                }
            }
            println!();
        }
    }
}