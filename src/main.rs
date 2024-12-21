mod connectfour;

use colored::*;
use connectfour::{ConnectFour, Mark};
use crossterm::{execute, terminal, cursor}; 

macro_rules! print_flush {
    ($($arg:tt)*) => {
        {
            use std::io::{self, Write}; 
            print!($($arg)*); 
            io::stdout().flush().expect("Failed to flush stdout.");
        }
    };
}

fn error_msg(message: &str, position: (u16, u16)) {
    clear_line();
    println!("{}", message.red()); 
    move_cursor(position);
    clear_line(); 
}

fn move_cursor(position: (u16, u16)) {
    execute!(std::io::stdout(), cursor::MoveTo(position.0, position.1)).unwrap();
}

fn clear_line() {
    execute!(std::io::stdout(), terminal::Clear(terminal::ClearType::CurrentLine)).unwrap(); 
}

fn clear_screen() {
    use std::io; 
    execute!(io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap(); 
    move_cursor((0, 0));
}

fn display_menu() {
    println!("Welcome to connect four!"); 
    println!("Select the game mode you'd like to play:");
    println!("  1. Original. (6x7 grid. Connect 4 tokens to win)"); 
    println!("  2. Custom. (Select your own grid size and required number of connected tokens to win)"); 
}

fn get_dimensions() -> (usize, usize) {
    use std::io; 
    clear_line();

    let position = cursor::position().unwrap(); 
    let dim_rows: usize = loop {
        print_flush!("Number of {}: ", "rows".bold()); 
        let mut dim_m_string = String::new(); 
        io::stdin().read_line(&mut dim_m_string).expect("Failed to read line.");

        match dim_m_string.trim().parse::<usize>() {
            Ok(m) if m > 2 => break m,
            Ok(_) => error_msg("Minimum 3 rows required", position), 
            Err(_) => error_msg("Please enter a valid number.", position),
        }
    };

    clear_line();
    let position = cursor::position().unwrap(); 
    let dim_cols: usize = loop {
        print_flush!("Number of {}: ", "columns".bold());
        let mut dim_n_string = String::new(); 
        io::stdin().read_line(&mut dim_n_string).expect("Failed to read line."); 

        match dim_n_string.trim().parse::<usize>() {
            Ok(n) if n > 2  => break n, 
            Ok(_) => error_msg("Minimum 3 columns required.", position), 
            Err(_) => error_msg("Please enter a valid number.", position),
        }
    };
    (dim_rows, dim_cols)
}

fn get_seq(dim_rows: usize, dim_cols: usize) -> usize {
    clear_line();
    let position = cursor::position().unwrap(); 
    loop { 
        print_flush!("Number of tokens to be connected: "); 
        let mut seq_string = String::new(); 
        std::io::stdin().read_line(&mut seq_string).expect("Failed to read line."); 

        match seq_string.trim().parse::<usize>() {
            Ok(seq) if seq < 3 => error_msg("Sequence size must be at least 3.", position), 
            Ok(seq) if seq <= dim_rows && seq <= dim_cols => break seq, 
            Ok(_) => error_msg("Sequence size cannot be more than the grid sizes.", position), 
            Err(_) => error_msg("Please enter a valid number.", position),
        }
    }
}

fn main() {
    use std::io; 

    'main: loop {
        clear_screen();
        display_menu();

        let position = cursor::position().unwrap(); 
        let mode: u8 = loop{
            let mut mode_string = String::new(); 
            io::stdin().read_line(&mut mode_string).expect("Failed to read line."); 

            match mode_string.trim().parse::<u8>() {
                Ok(choice) if choice == 1 || choice == 2 => break choice, 
                Ok(_) | Err(_) => error_msg("Please choose a valid option.", position),
            }
        };

        let (dim_rows, dim_cols) = if mode == 1 { (6, 7) } else { get_dimensions() };
        let seq: usize = if mode == 1 { 4 } else { get_seq(dim_rows, dim_cols) };

        let marks = [Mark::Red, Mark::Yellow]; 
        let mut turn: usize = 0; 
        let mut game = ConnectFour::new(dim_rows, dim_cols, seq); 
    
        loop { 
            clear_screen();
            game.display_board(); 
            println!("\nIt's Player {}'s turn.", marks[turn % 2]); 
            
            let position = cursor::position().unwrap(); 
            let col = loop {
                print_flush!("Enter a column: "); 
                let mut col_string = String::new(); 
                io::stdin().read_line(&mut col_string).expect("Failed to read line."); 
    
                match col_string.trim().parse::<usize>() {
                    Ok(col) if col > 0 && game.is_valid(col - 1) => break col, 
                    Ok(_) => error_msg("Please enter a valid column number.", position), 
                    Err(_) => error_msg("Please enter a valid number.", position),
                }
            };

            game.update_board(col - 1, marks[turn % 2].clone());
            turn += 1; 
    
            if game.check_win(marks[1 - (turn % 2)].clone()) {
                clear_screen();
                game.display_board();
                println!("\nPlayer {} wins.", marks[1 - (turn % 2)]);
                break; 
            } else if turn == dim_rows*dim_cols { // check tie
                clear_screen();
                game.display_board();
                println!("\nIt's a tie."); 
                break; 
            }
        }

        let position = cursor::position().unwrap(); 
        loop {
            print_flush!("Would you like to play again? (y/n): "); 
            let mut again_choice = String::new();
            io::stdin().read_line(&mut again_choice).expect("Failed to read line."); 

            match again_choice.trim().to_lowercase().as_str() {
                "y" | "yes" => break, 
                "n" | "no" => {
                    println!("Thanks for playing."); 
                    break 'main;
                },
                _ => {
                    move_cursor(position);
                    clear_line();
                    continue; 
                },
            }
        }
    }
}
