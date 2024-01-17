use std::result::Result;
 
fn main() {
    println!("Hello, world!");
    let mut board = Board::new();
    loop {
        //player
        println!("you are red");
        println!("enter your position");
        let mut user_input = String::new();
        let _ = std::io::stdin().read_line(&mut user_input);
        println!("{}", user_input.pop().unwrap());
        let _ = match user_input.parse::<u8>(){
            Ok(value) => board.add_piece(value, Spot::RED),
            _ => {
                println!("you bad");
                continue;
            }
        };
        board.add_piece(0, Spot::YELLOW);
        board.display();
    }
    
}
#[derive(Clone, Copy)]
enum Spot{
    RED,
    YELLOW,
    BLANK
}


struct Board{
    board: [Spot; 64],
}

impl Board{
    fn new() -> Self {
        Board {
            board: [Spot::BLANK; 64],
        }
    }
    fn add_piece(&mut self, column: u8, color: Spot) -> Result<(), &'static str> {
        let mut row = 0;
        while row < 8 {
            match self.board[(column as usize) + (8 * row) as usize] {
                Spot::BLANK => {
                    self.board[(column as usize) + (8 * row) as usize] = color;
                    return Ok(());
                }
                _ => (),
            }
            row += 1;
        }
        Err("Column is full")
    }
    fn display(&self) {
        for row in (0..8).rev() {
            for col in 0..8 {
                match self.board[col + (8 * row) as usize] {
                    Spot::RED => print!("R "),
                    Spot::YELLOW => print!("Y "),
                    Spot::BLANK => print!("_ "),
                }
            }
            println!();
        }
        println!();
        println!("0 1 2 3 4 5 6 7");
    }
}
