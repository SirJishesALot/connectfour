mod connectfour;

use connectfour::{ConnectFour, Mark};
use termion::{clear, cursor}; 

macro_rules! print_flush {
    ($($arg:tt)*) => {
        {
            use std::io::{self, Write}; 
            print!($($arg)*); 
            io::stdout().flush().expect("Failed to flush stdout.");
        }
    };
}

fn clear_screen() {
    use std::io::Write;
    print!("{}", clear::All); 
    print!("{}", cursor::Goto(1, 1));
    std::io::stdout().flush().unwrap();
}

fn get_dimensions() -> (usize, usize) {
    use std::io; 
    let dim_m: usize = loop {
        print_flush!("Enter the number of rows: "); 
        let mut dim_m_string = String::new(); 
        io::stdin().read_line(&mut dim_m_string).expect("Failed to read line.");

        match dim_m_string.trim().parse::<usize>() {
            Ok(m) => {
                if m > 2 { break m; } 
                else { println!("Minimum 3 rows required."); }
            }, 
            Err(_) => println!("Please enter a valid number."),
        }
    };

    let dim_n: usize = loop {
        print_flush!("Enter the number of columns: ");
        let mut dim_n_string = String::new(); 
        io::stdin().read_line(&mut dim_n_string).expect("Failed to read line."); 

        match dim_n_string.trim().parse::<usize>() {
            Ok(n) => {
                if n > 2 { break n; }
                else { println!("Minimum 3 columns required."); }
            }
            Err(_) => println!("Please enter a valid number."),
        }
    };
    (dim_m, dim_n)
}

fn main() {
    use std::io; 

    'main: loop {
        let marks = [Mark::Red, Mark::Yellow]; 
        let mut turn: usize = 0; 
        let mut game = ConnectFour::new(); 
    
        loop { 
            clear_screen();
            game.display_board(); 
            println!("\nIt's Player {}'s turn.", marks[turn % 2]); 
    
            let col = loop {
                print_flush!("Enter a column: "); 
                let mut col_string = String::new(); 
                io::stdin().read_line(&mut col_string).expect("Failed to read line."); 
    
                match col_string.trim().parse::<usize>() {
                    Ok(col) => {
                        if col > 0 && col < 8 && game.is_valid(col - 1) {
                            break col;
                        } else {
                            println!("Please enter a valid column number."); 
                        }
                    }
                    Err(_) => println!("Please enter a number."),
                }
            };

            game.update_board(col - 1, marks[turn % 2].clone());
            turn += 1; 
    
            if game.check_win(marks[1 - (turn % 2)].clone()) {
                clear_screen();
                game.display_board();
                println!("\nPlayer {} wins.", marks[1 - (turn % 2)]);
                break; 
            } else if turn == 6*7 { // check tie
                clear_screen();
                game.display_board();
                println!("\nIt's a tie."); 
                break; 
            }
        }

        loop {
            print_flush!("Would you like to play again? (y/n)?: "); 
            let mut again_choice = String::new();
            io::stdin().read_line(&mut again_choice).expect("Failed to read line."); 

            match again_choice.trim().to_lowercase().as_str() {
                "y" | "yes" => break, 
                "n" | "no" => {
                    println!("Thanks for playing."); 
                    break 'main;
                },
                _ => continue,
            }
        }
    }
    
}
