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
    fn make_win(&mut self) {
        *self = match self {
            _ => Mark::Win(Box::new(self.clone()))
        };
    }
}

pub struct ConnectFour {
    dim_rows: usize, 
    dim_cols: usize, 
    seq: usize, 
    board: Vec<Vec<Mark>>,
}

impl ConnectFour {
    pub fn new(dim_rows: usize, dim_cols: usize, seq: usize) -> Self {
        ConnectFour {
            dim_rows,
            dim_cols, 
            seq, 
            board: vec![vec![Mark::Empty; dim_cols]; dim_rows],
        }
    }

    fn print_col_nums(col_size: usize) {
        for col in 1..=col_size {
            print!(" {}", col);
        }
        print!("\n");
    }

    pub fn display_board(&self) {
        Self::print_col_nums(self.dim_cols); 
        for i in 0..self.dim_rows {
            print!("|");
            for j in 0..self.dim_cols - 1 {
                print!("{} ", self.board[i][j]); 
            }
            print!("{}|\n", self.board[i][self.dim_cols - 1]); 
        }
        Self::print_col_nums(self.dim_cols); 
    }

    fn is_vertical_match(&mut self, row: usize, col: usize, mark: &Mark) -> bool {
        if (0..self.seq).all(|offset| self.board[row + offset][col] == *mark) {
            (0..self.seq).for_each(|offset| self.board[row + offset][col].make_win()); 
            return true; 
        } false
    }

    fn check_cols(&mut self, mark: &Mark) -> bool {
        for col in 0..self.dim_cols {
            for row in 0..=(self.dim_rows - self.seq) {
                if self.is_vertical_match(row, col, mark) { return true; }
            }
        } false 
    }

    fn check_diagonals(&mut self, mark: &Mark) -> bool {
        let bound = self.seq - 1; 
        for row in 0..=(self.dim_rows - self.seq) {
            for shift in 0..=(self.dim_cols - self.seq) {
                let diagonal: bool = 
                    (0..self.seq).all(|offset| self.board[row + offset][shift + (bound-offset)] == *mark);
                let antidiagonal: bool = 
                    (0..self.seq).all(|offset| self.board[row + offset][(self.dim_cols-1) - (shift + (bound-offset))] == *mark);

                if diagonal || antidiagonal { 
                    (0..self.seq).for_each(|offset| {
                        let col = if antidiagonal { (self.dim_cols-1) - (shift + (bound-offset)) } else { shift + (bound-offset) };
                        self.board[row + offset][col].make_win()
                    });
                    return true; 
                }
            }
        }  
        false
    }

    pub fn check_win(&mut self, mark: Mark) -> bool {
        for row in 0..self.dim_rows { // rows
            for i in 0..=(self.dim_cols - self.seq) {
                if self.board[row][i..i+self.seq].iter().all(|x| x == &mark) {
                    self.board[row][i..i+self.seq].iter_mut().for_each(|x| x.make_win());
                    return true;
                }
            }
        }
        self.check_cols(&mark) || self.check_diagonals(&mark)
    }

    pub fn update_board(&mut self, col: usize, mark: Mark)  {
        let mut row: usize = self.dim_rows - 1; 

        for i in 0..(self.dim_rows - 1) {
            if self.board[i + 1][col] != Mark::Empty {
                row = i; 
                break; 
            }
        }
        self.board[row][col] = mark;
    }

    pub fn is_valid(&self, col: usize) -> bool {
        col < self.dim_cols && self.board[0][col] == Mark::Empty
    }
}