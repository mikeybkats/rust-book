use std::io::{self, Write};

struct Board {
    cells: [Option<char>; 9],
}

impl Board {
    fn new() -> Board {
        Board { cells: [None; 9] }
    }

    fn display(&self) {
        println!(
            " {} | {} | {}",
            self.symbol(0),
            self.symbol(1),
            self.symbol(2)
        );
        println!("---+---+---");
        println!(
            " {} | {} | {}",
            self.symbol(3),
            self.symbol(4),
            self.symbol(5)
        );
        println!("---+---+---");
        println!(
            " {} | {} | {}",
            self.symbol(6),
            self.symbol(7),
            self.symbol(8)
        );
    }

    fn symbol(&self, index: usize) -> char {
        self.cells[index].unwrap_or(' ')
    }

    fn make_move(&mut self, index: usize, symbol: char) -> Result<(), &'static str> {
        if index < 0 || index >= 9 {
            return Err("Invalid move. Index out of range.");
        }

        if self.cells[index].is_some() {
            return Err("Invalid move. Cell already occupied.");
        }

        self.cells[index] = Some(symbol);
        Ok(())
    }

    fn has_won(&self, symbol: char) -> bool {
        let win_conditions = [
            // Rows
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            // Columns
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            // Diagonals
            [0, 4, 8],
            [2, 4, 6],
        ];

        for condition in &win_conditions {
            if self.cells[condition[0]] == Some(symbol)
                && self.cells[condition[1]] == Some(symbol)
                && self.cells[condition[2]] == Some(symbol)
            {
                return true;
            }
        }

        false
    }

    fn is_full(&self) -> bool {
        self.cells.iter().all(Option::is_some)
    }
}

fn main() {
    let mut board = Board::new();
    let mut current_player = 'X';

    loop {
        board.display();
        println!("Player {}, enter your move (0-8):", current_player);

        let mut input = String::new();
        io::stdout().flush().expect("Failed to flush stdout.");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        let index = input.trim().parse::<usize>().unwrap();

        match board.make_move(index, current_player) {
            Ok(_) => {
                if board.has_won(current_player) {
                    println!("Player {} wins!", current_player);
                    break;
                } else if board.is_full() {
                    println!("It's a draw!");
                    break;
                } else {
                    current_player = if current_player == 'X' { 'O' } else { 'X' };
                }
            }
            Err(msg) => println!("{}", msg),
        }
    }
}
