use std::result::Result;
use std::time::SystemTime;
//number of rows
const ROWS: i32 = 6;
//number of columns
const COLUMNS: i32 = 7;
 
fn main() {
    println!("Hello, world!");
    let mut board = Board::new();
    let now = SystemTime::now();
    let mut column = 0;
    /*loop{
        match board.add_piece(column, Spot::Red){
            Ok(_) => (),
            Err(_) => column += 1
        }
        if column > 7{
            break;
        }
    }
    println!("{}", now.elapsed().unwrap().as_nanos());
    */
    loop {
        //player
        println!("you are red");
        println!("enter your position");
        let mut user_input = String::new();
        let _ = std::io::stdin().read_line(&mut user_input);
        println!("{}", user_input.pop().unwrap());
        let position: i32 = match user_input.parse::<i32>(){
            Ok(value) => match board.add_piece(value, Spot::Red){
                Ok(position) => position,
                Err(..) => {
                    println!("you bad (not a valid number)");
                    continue
                }
            },
            _ => {
                println!("you bad! (not a number)");
                continue;
            }
        };
        board.display();
        if board.check_win(Spot::Red, position){
            println!("red wins");
            break;
        }
        println!(" . ");
        /*let position = match  board.add_piece(0, Spot::Yellow){
            Ok(value) => value,
            Err(_) => 0,
        };
        board.display();
        if board.check_win(Spot::Yellow, position){
            println!("yellow wins");
            break;
        }*/
    }
    
    
}
#[derive(Clone, Copy, PartialEq)]
enum Spot{
    Red,
    Yellow,
    Blank
}


struct Board{
    board: [Spot; (ROWS * COLUMNS) as usize],
}

impl Board{
    fn new() -> Self {
        Board {
            board: [Spot::Blank; (ROWS * COLUMNS) as usize],
        }
    }
    fn add_piece(&mut self, column: i32, color: Spot) -> Result<i32, &'static str> {
        if(column >= COLUMNS){
            return Err("column to high");
        }
        let mut row = 0;
        while row < ROWS {
            match self.board[(column as usize) + (COLUMNS * row) as usize ] {
                Spot::Blank => {
                    self.board[(column as usize) + (COLUMNS* row) as usize ] = color;
                    return Ok(column + (COLUMNS* row));
                }
                _ => (),
            }
            row += 1;
        }
        Err("Column is full")
    }
    fn display(&self) {
        for row in (0..ROWS).rev() {
            for col in 0..COLUMNS {
                match self.board[(col + (COLUMNS * row)) as usize] {
                    Spot::Red => print!("R "),
                    Spot::Yellow => print!("Y "),
                    Spot::Blank => print!("_ "),
                }
            }
            println!();
        }
        println!();
        println!("0 1 2 3 4 5 6 7");
    }
    fn check_win(&self, last_team: Spot, last_piece: i32) -> bool{
        let last_row = (last_piece % COLUMNS) as usize;
        //println!("last row {}", last_row);
        let last_col = (last_piece / COLUMNS) as usize;
        //println!("last column {}", last_col);

        // Check horizontally
        let mut count = 0;
        for col in 0..COLUMNS as usize {
            if self.board[col + last_col * COLUMNS as usize] == last_team {
                count += 1;
            } else {
                count = 0;
            }

            if count >= 4 {
                return true;
            }
        }

        // Check vertically
        count = 0;
        for row in 0..ROWS as usize {
            if self.board[last_row + row * COLUMNS as usize] == last_team {
                count += 1;
            } else {
                count = 0;
            }

            if count >= 4 {
                return true;
            }
        }

        // Check diagonally (from top-left to bottom-right)
        count = 0;
        let mut row = last_row as i32 - last_col as i32;
        let mut col = 0;
        while row < ROWS && col < COLUMNS {
            if row >= 0 && self.board[(col + row * COLUMNS) as usize] == last_team {
                count += 1;
            } else {
                count = 0;
            }

            if count >= 4 {
                return true;
            }

            row += 1;
            col += 1;
        }

        // Check diagonally (from top-right to bottom-left)
        count = 0;
        let mut row = last_row as i32 + last_col as i32;
        let mut col = COLUMNS - 1;
        while row >= 0 && row < ROWS && col >= 0 {
            if self.board[(col + row * COLUMNS ) as usize] == last_team {
                count += 1;
            } else {
                count = 0;
            }

            if count >= 4 {
                return true;
            }

            row -= 1;
            col -= 1;
        }

        false
    }
}
