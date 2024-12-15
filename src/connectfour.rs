use colored::*; 

#[derive(PartialEq)]
pub enum Mark { Empty, Red, Yellow, Win(Box<Mark>) }
impl std::fmt::Display for Mark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mark = match self {
            Mark::Empty => "*".white(), 
            Mark::Red => "O".red(), 
            Mark::Yellow => "O".yellow(),
            Mark::Win(original) => format!("{}", *original).bold(),
        };
        write!(f, "{}", mark)
    }
}



impl Clone for Mark {
    fn clone(&self) -> Self {
        match self {
            Mark::Empty => Mark::Empty, 
            Mark::Red => Mark::Red, 
            Mark::Yellow => Mark::Yellow, 
            Mark::Win(original) => Mark::Win(original.clone()), 
        }
    }
}

impl Mark {
    fn toggle(&mut self) {
        *self = match self {
            _ => Mark::Win(Box::new(self.clone()))
        };
    }
}

pub struct ConnectFour {
    board: [[Mark; 7]; 6]
}

impl ConnectFour {
    pub fn new() -> Self {
        ConnectFour {
            board: [ // initialise to 6x7 of empty
                [Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty], 
                [Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty], 
                [Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty], 
                [Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty], 
                [Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty], 
                [Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty, Mark::Empty]
            ]
        }
    }

    pub fn display_board(&self) {
        println!(" 1 2 3 4 5 6 7");
        for i in 0..6 {
            print!("|");
            for j in 0..6 {
                print!("{} ", self.board[i][j]); 
            }
            print!("{}|\n", self.board[i][6]); 
        }
        println!(" 1 2 3 4 5 6 7");
    }

    fn is_vertical_match(&mut self, row: usize, col: usize, mark: &Mark) -> bool {
        if (0..4).all(|offset| self.board[row + offset][col] == *mark) {
            (0..4).for_each(|offset| self.board[row + offset][col].toggle()); 
            return true; 
        } false
    }

    fn check_cols(&mut self, mark: &Mark) -> bool {
        for col in 0..7 {
            for row in 0..3 {
                if Self::is_vertical_match(self, row, col, mark) { return true; }
            }
        } false 
    }

    fn check_diagonals(&mut self, mark: &Mark) -> bool {
        for row in 0..3 {
            for shift in 0..4 {
                let diagonal: bool = {
                    (0..4).all(|offset| self.board[row + offset][shift + (3-offset)] == *mark)
                };
                let antidiagonal: bool = {
                    (0..4).all(|offset| self.board[row + offset][6 - (shift + (3-offset))] == *mark)
                };

                if diagonal || antidiagonal { 
                    let _ = (0..4).for_each(|offset| {
                        let col = if antidiagonal { 6 - (shift + (3-offset)) } else { shift + (3-offset) };
                        self.board[row + offset][col].toggle()
                    });
                    return true; 
                }
            }
        }  
        false
    }

    pub fn check_win(&mut self, mark: Mark) -> bool {
        for row in 0..6 { // rows
            for i in 0..4 {
                if self.board[row][i..i+4].iter().all(|x| x == &mark) {
                    self.board[row][i..i+4].iter_mut().for_each(|x| x.toggle());
                    return true;
                }
            }
        }

        Self::check_cols(self, &mark) || Self::check_diagonals(self, &mark)
    }

    pub fn update_board(&mut self, col: usize, mark: Mark)  {
        let mut row: usize = 5; 

        for i in 0..5 {
            if self.board[i + 1][col] != Mark::Empty {
                row = i; 
                break; 
            }
        }
        self.board[row][col] = mark;
    }

    pub fn is_valid(&self, col: usize) -> bool {
        self.board[0][col] == Mark::Empty
    }
}