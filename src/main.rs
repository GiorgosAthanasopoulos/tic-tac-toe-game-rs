use std::io;

struct TicTacToeGame {
    board: Vec<String>,
}

impl TicTacToeGame {
    pub fn new() -> TicTacToeGame {
        let mut game = TicTacToeGame { board: vec![] };
        for i in 0..9 {
            game.board.push((i + 1).to_string());
        }

        assert!(game.board.len() == 9);

        return game;
    }

    fn print_board(&self) {
        assert!(self.board.len() == 9);

        let mut counter = 0;
        for i in 0..7 {
            if i % 2 == 0 {
                println!("|---|---|---|");
            } else {
                println!(
                    "| {} | {} | {} |",
                    self.board[counter],
                    self.board[counter + 1],
                    self.board[counter + 2]
                );
                counter += 3;
            }
        }
    }

    fn check_winner(&self) -> String {
        assert!(self.board.len() == 9);

        let mut counter = 0;
        let mut line: String;

        for i in 0..9 {
            match i {
                0 => {
                    line = vec![
                        self.board[0].clone(),
                        self.board[1].clone(),
                        self.board[2].clone(),
                    ]
                    .into_iter()
                    .collect();
                }
                1 => {
                    line = vec![
                        self.board[3].clone(),
                        self.board[4].clone(),
                        self.board[5].clone(),
                    ]
                    .into_iter()
                    .collect();
                }
                2 => {
                    line = vec![
                        self.board[6].clone(),
                        self.board[7].clone(),
                        self.board[8].clone(),
                    ]
                    .into_iter()
                    .collect();
                }
                3 => {
                    line = vec![
                        self.board[0].clone(),
                        self.board[3].clone(),
                        self.board[6].clone(),
                    ]
                    .into_iter()
                    .collect();
                }
                4 => {
                    line = vec![
                        self.board[1].clone(),
                        self.board[4].clone(),
                        self.board[7].clone(),
                    ]
                    .into_iter()
                    .collect();
                }
                5 => {
                    line = vec![
                        self.board[2].clone(),
                        self.board[4].clone(),
                        self.board[8].clone(),
                    ]
                    .into_iter()
                    .collect();
                }
                6 => {
                    line = vec![
                        self.board[0].clone(),
                        self.board[4].clone(),
                        self.board[8].clone(),
                    ]
                    .into_iter()
                    .collect();
                }
                7 => {
                    line = vec![
                        self.board[2].clone(),
                        self.board[4].clone(),
                        self.board[6].clone(),
                    ]
                    .into_iter()
                    .collect();
                }
                _ => {
                    line = String::from("");
                }
            }

            if line == "XXX" {
                return String::from("X");
            } else if line == "OOO" {
                return String::from("O");
            }

            if (i + 1).to_string() == self.board[i] {
                counter += 1;
            }
        }

        if counter == 0 {
            return String::from("Draw");
        } else {
            return String::from("None");
        }
    }

    pub fn start_game(&mut self) {
        assert!(self.board.len() == 9);

        let mut winner = String::from("None");
        let mut turn = String::from("X");
        let mut choice: usize = 0;
        let stdin = io::stdin();

        self.print_board();

        while winner == "None" {
            println!("Enter integer to place {} into: ", turn);
            let mut input = String::new();
            match stdin.read_line(&mut input) {
                Ok(_) => {
                    input = input.trim().replace("\n", "");
                    match input.parse::<usize>() {
                        Ok(x) => {
                            choice = x;
                        }
                        Err(error) => {
                            println!("An error occurred: {}", error);
                        }
                    }
                }
                Err(error) => {
                    println!("An error occurred: {}", error);
                }
            }

            if choice < 1 || choice > 9 || self.board[choice - 1] != choice.to_string() {
                println!("Invalid input!");
                continue;
            }

            self.board[choice - 1] = turn.clone();
            turn = if turn == "X" {
                String::from("O")
            } else {
                String::from("X")
            };

            winner = self.check_winner();
            self.print_board();
        }

        println!("Winner: {}", winner);
    }
}

fn main() {
    let mut game = TicTacToeGame::new();
    assert!(game.board.len() == 9);
    game.start_game();
}
