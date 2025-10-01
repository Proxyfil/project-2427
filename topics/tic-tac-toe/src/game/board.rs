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
        println!("Choose a position (1-9):");
        let mut position = 1;
        for row in &self.cells {
            for cell in row {
                match cell {
                    Some(symbol) => print!("| {} |", symbol),
                    None => print!("| {} |", position),
                }
                position += 1;
            }
            println!();
        }
    }

    pub fn place_symbol_by_position(&mut self, position: usize, symbol: char) -> bool {
        if position < 1 || position > 9 {
            return false;
        }
        
        let (row, col) = self.position_to_coords(position);
        if self.cells[row][col].is_none() {
            self.cells[row][col] = Some(symbol);
            true
        } else {
            false
        }
    }

    fn position_to_coords(&self, position: usize) -> (usize, usize) {
        let index = position - 1; // Convert to 0-based index
        let row = index / 3;
        let col = index % 3;
        (row, col)
    }

    pub fn is_full(&self) -> bool {
        for row in &self.cells {
            for cell in row {
                if cell.is_none() {
                    return false;
                }
            }
        }
        true
    }
}