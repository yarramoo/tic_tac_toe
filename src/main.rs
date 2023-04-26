use std::fmt::Display;
use std::fmt::Write;

#[derive(Clone, Copy, PartialEq)]
enum Player {
    X, O,
}

impl Player {
    fn opponent(&self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
    fn to_char(&self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
    }
}

fn show_cell(player: Option<Player>) -> char {
    if let Some(p) = player {
        return p.to_char();
    };
    return ' ';
}

struct TicTacToe {
    board: [[Option<Player>; 3]; 3],
    turn: Player,
}

enum GameState {
    Won(Player),
    Full,
    Ongoing,
}

impl TicTacToe {
    fn new() -> Self {
        TicTacToe { 
            board: [[None; 3]; 3], 
            turn: Player::X,
        }
    }

    fn to_string(&self, f: &mut String) -> std::fmt::Result {
        writeln!(f, "┏━━━┳━━━┳━━━┓")?;
        for row in 0 .. 3 {
            writeln!(f,
                "┃ {} ┃ {} ┃ {} ┃", 
                show_cell(self.board[row][0]),
                show_cell(self.board[row][1]),
                show_cell(self.board[row][2]),
            )?;
            if row != 2 { writeln!(f, "┣━━━╋━━━╋━━━┫")?; }
        }
        writeln!(f, "┗━━━┻━━━┻━━━┛")?;
        Ok(())
    }

    fn has_won(&self, player: Player) -> bool {
        macro_rules! has {
            ($player:expr, $row:expr, $col:expr) => {
                self.board[$row][$col] == Some($player)
            };
        }
        // Check rows
        for row in 0..3 {
            if has!(player,row,0) && has!(player,row,1) && has!(player,row,2) {return true;}
        }
        // Check cols
        for col in 0..3 {
            if has!(player,0,col) && has!(player,1,col) && has!(player,2,col) {return true;}
        }
        // Check diags
        if has!(player,0,0) && has!(player,1,1) && has!(player,2,2) {return true;}
        return has!(player,0,2) && has!(player,1,1) && has!(player,2,0);
    }

    fn is_full(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|cell| cell.is_some()))
    }

    fn make_move(&mut self, row: usize, col: usize) -> Result<GameState, &'static str> {
        // return Ok(is_finished, is_won) to tell whether either the player won the game or the board is full
        if row > 2 || col > 2 { 
            return Err("Tile out of bounds!"); 
        }
        if self.board[row][col].is_some() {
            return Err("Tile already taken!")
        }
        // Move valid - make the move
        self.board[row][col] = Some(self.turn);
        // Check if the game is over
        if self.has_won(self.turn) {
            return Ok(GameState::Won(self.turn));
        }
        if self.is_full() {
            return Ok(GameState::Full);
        }
        // If not won, next player moves
        self.turn = self.turn.opponent();
        Ok(GameState::Ongoing)
    }

    fn play(&mut self) {
        use std::io;
        let mut buf = String::new();
        let stdin = io::stdin();
        loop {
            // Display board
            print!("{}[2J", 27 as char);
            println!("{}", self);
            // Display cur player
            println!("Player to move: {}", self.turn.to_char());
            // Get move
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            let mut chars = buf.chars();
            let input_row = chars.next().unwrap();
            chars.next();
            let input_col = chars.next().unwrap();
            let row = input_row as usize - '0' as usize;
            let col = input_col as usize - '0' as usize;
            // Display result of move (good move or bad move)
            let result = self.make_move(row, col);
            match result {
                Ok(GameState::Ongoing) => {},
                Ok(GameState::Full) => {
                    println!("{}", self);
                    println!("Game over! It's a draw");
                    break;
                }
                Ok(GameState::Won(player)) => {
                    println!("{}", self);
                    println!("Game over! Player {} has won!", player.to_char());
                    break;
                }
                Err(e) => println!("{}", e),
            }
        }

    }
}

impl Display for TicTacToe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        let mut s = String::new();
        self.to_string(&mut s)?;
        write!(f, "{}", s)
    }
}

fn main() {
    let mut tictactoe = TicTacToe::new();
    tictactoe.play();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_won_false() {
        let mut tictactoe = TicTacToe::new();
        assert!(!tictactoe.has_won(Player::O));
        assert!(!tictactoe.has_won(Player::X));
    }

    #[test]
    fn test_has_won_false_filled() {
        use Player::*;
        let mut tictactoe = TicTacToe {
            board: [[None, Some(X), None], [None, None, Some(O)], [Some(O), None, None]],
            turn: X,
        };
        assert!(!tictactoe.has_won(Player::O));
        assert!(!tictactoe.has_won(Player::X));
    }

    #[test]
    fn test_has_won_true_row() {
        use Player::*;
        let mut tictactoe = TicTacToe {
            board: [[Some(X), Some(X), Some(X)], [None, None, None], [None, None, None]],
            turn: X,
        }; 
        assert!(tictactoe.has_won(X));
        assert!(!tictactoe.has_won(O));
    }
    #[test]
    fn test_has_won_true_col() {
        use Player::*;
        let mut tictactoe = TicTacToe {
            board: [[Some(X), None, None], [Some(X), None, None], [Some(X), None, None]],
            turn: X,
        }; 
        assert!(tictactoe.has_won(X));
        assert!(!tictactoe.has_won(O));
    }
    #[test]
    fn test_has_won_true_diag() {
        use Player::*;
        let mut tictactoe = TicTacToe {
            board: [[Some(X), None, None], [None, Some(X), None], [None, None, Some(X)]],
            turn: X,
        }; 
        assert!(tictactoe.has_won(X));
        assert!(!tictactoe.has_won(O));
    }
    #[test]
    fn is_full() {
        use Player::*;
        let mut tictactoe = TicTacToe {
            board: [[Some(X), None, None], [None, Some(X), None], [None, None, Some(X)]],
            turn: X,
        }; 
        assert!(!tictactoe.is_full());
        let mut tictactoe = TicTacToe {
            board: [[Some(X), Some(X), Some(X)], [Some(X), Some(X), Some(X)], [Some(X), Some(X), Some(X)]],
            turn: X,
        };
        assert!(tictactoe.is_full());
    }
}