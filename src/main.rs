use std::result::Result; use std::time::SystemTime;
//number of rows
const ROWS: i32 = 6;
//number of columns
const COLUMNS: i32 = 7;
// MUST BE ODD
const DEPTH: usize = 7;

 
fn main() {
    println!("Hello, world!");
    let mut board = Board::new();
    /*
    let now = SystemTime::now();
    let mut column = 0;
    loop{
        match board.add_piece(column, Spot::Red){
            Ok(_) => (),
            Err(_) => column += 1
        }
        if column > 7{
            break;
        }
        
        board.check_win(Spot::Red, 0,);
        
    }
    println!("{}", now.elapsed().unwrap().as_nanos());
    */
    loop {
        //player
        println!("you are red");
        println!("enter your position");
        let mut user_input = String::new();
        let _ = std::io::stdin().read_line(&mut user_input);
        user_input.pop();
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
        if DEPTH %2 == 0{
            panic!("DEPTH MUST BE ODD");
        }
        board.display();
        if board.check_win(Spot::Red, position){
            println!("red wins");
            break;
        }
        println!("AI going");
        board.add_piece(board.find_next_move(Spot::Yellow), Spot::Yellow).unwrap();
        board.display();
        /*let position = match  board.add_piece(0, Spot::Yellow){
            Ok(value) => value,
            Err(_) => 0,
        };
        board.display();
        if board.check_win(Spot::Yellow, position){
            println!("yellow wins");
            break;
        }
    }
    */
    }
    
}
#[derive(Clone, Copy, PartialEq)]
enum Spot{
    Red,
    Yellow,
    Blank
}

#[derive(Clone, Copy)]
struct Board{
    board: [Spot; (ROWS * COLUMNS) as usize],
}

fn add_one(array: &mut [i32] ){
    
    array[0] += 1;
    for i in 0..array.len(){
        if array[i] == COLUMNS{
            array[i] = 0;
            if i < array.len() -1{
                array[i+1]+=1;
            }
        }
    }
}
impl Board{
    fn new() -> Self {
        Board {
            board: [Spot::Blank; (ROWS * COLUMNS) as usize],
        }
    }
    fn add_piece(&mut self, column: i32, color: Spot) -> Result<i32, &'static str> {
        if column >= COLUMNS  || column < 0{
            return Err("column to high or low");
        }
        let mut row = 0;
        while row < ROWS {
            if self.board[(column as usize) + (COLUMNS * row) as usize] == Spot::Blank{
                self.board[(column as usize) + (COLUMNS* row) as usize ] = color;
                return Ok(column + (COLUMNS* row));
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
    
    fn find_next_move(&self, team: Spot) -> i32{
        //an array of all the moves the ai will play, the first item is not used 
        let mut ai_array: (i32, [i32; (DEPTH/2) + 1]) = (0, [0; (DEPTH/2) + 1]);
        //first is score, second is position
        let mut best_position: (i32, i32) = (-1, -1);
        //looping over all possible ai moves
        loop{

            //increase array by 1
            add_one(&mut ai_array.1);
            //break if all values are full
            if ai_array.1[(DEPTH/2) -0] == COLUMNS - 1{
                break;
            }
            let mut user_array: (i32, [i32; (DEPTH/2) ]) = (0, [0; DEPTH/2]);
            // worst result records the worst the ai does with this set of moves
            let mut worst_result: (i32, i32) = (5, 0);
            // looping over all possible ai moves
            loop{
                add_one(&mut user_array.1);
                //break if all values are full
                if user_array.1[DEPTH/2 -1] == COLUMNS -1{
                    break;
                }
                //create a copy of the board
                let mut board = self.to_owned();
                //add all the moves to the board
                for i in 0..DEPTH{
                
                    //the team playing this move
                    let local_team;
                    if i % 2 == 0{
                        local_team = team;
                    }
                    else{
                        local_team =  match team{
                            Spot::Red => Spot::Yellow,
                            Spot::Yellow => Spot::Red,
                            _ => panic!("not a valid team")
                        };
                    }
                    //the position in the board where the piece landed
                    let position = match local_team{
                        Spot::Red => board.add_piece(user_array.1[i/2], Spot::Red),
                        Spot::Yellow => board.add_piece(ai_array.1[i/2], Spot::Yellow),
                        _ => panic!("not a valid team")
                    };
                    
                    match position{
                        Err(_) =>break,
                        Ok(value) => {
                            //if it is the human's turn and they won
                            if i % 2 == 1 && board.count_score(Spot::Red, value) == 4{
                                //if the human wins it is a bad strat
                                worst_result.0 = 0;
                                break;
                            }
                                
                            //if the ai does worse this round, aka the human does better
                            if i == DEPTH -1 && self.count_score(Spot::Yellow, value) < worst_result.0{
                                worst_result.1 = ai_array.1[0];
                                worst_result.0 = self.count_score(Spot::Yellow, value);
                                //println!("worse solution found: {} at {}", worst_result.0, worst_result.1 );
                            }
                        }
                    }
                }
            }
            // if playing against the best human is still better than the current best move,
            // replace it
            if worst_result.0 > best_position.0{
            //if worst_result.0 ==4{
                println!("new solution found");
                println!("better solution found: {} at {}", worst_result.0, worst_result.1 );
                best_position = worst_result;
                //best_position.0 = worst_result.0;
                //best_position.1 = worst_result.1;
            }
        }

        return best_position.1;
        
    }
    fn check_win(&self, last_team: Spot, last_piece: i32) -> bool{
        return self.count_score(last_team, last_piece) >= 4;
    }
    fn count_score(&self, last_team: Spot, last_piece: i32) -> i32{
        let last_row = (last_piece % COLUMNS) as usize;
        //println!("last row {}", last_row);
        let last_col = (last_piece / COLUMNS) as usize;
        //println!("last column {}", last_col);

        // Check horizontally
        let mut max_count = 0;
        let mut count = 0;
        let mut local_max = 0;
        for col in 0..COLUMNS as usize {
            if self.board[col + last_col * COLUMNS as usize] == last_team {
                count += 1;
            } else {
                count = 0;
            }
            if count > local_max {local_max = count;}

        }
        if local_max > max_count {max_count = local_max;}
        count = 0;
        local_max = 0;
        // Check vertically
        for row in 0..ROWS as usize {
            if self.board[last_row + row * COLUMNS as usize] == last_team {
                count += 1;
            } else {
                count = 0;
            }
            if count > local_max {local_max = count;}

        }

        // Check diagonally (from top-left to bottom-right)
        count = 0;
        if local_max > max_count {max_count = local_max;}
        local_max = 0;
        let mut row = last_row as i32 - last_col as i32;
        let mut col = 0;
        while row < ROWS && col < COLUMNS {
            if row >= 0 && self.board[(col + row * COLUMNS) as usize] == last_team {
                count += 1;
            } else {
                count = 0;
            }


            if count > local_max {local_max = count;}
            row += 1;
            col += 1;
        }

        // Check diagonally (from top-right to bottom-left)
        count = 0;
        if local_max > max_count {max_count = local_max;}
        local_max = 0;
        let mut row = last_row as i32 + last_col as i32;
        let mut col = COLUMNS - 1;
        while (0..ROWS).contains(&row) && col >= 0 {
            if self.board[(col + row * COLUMNS ) as usize] == last_team {
                count += 1;
            } else {
                count = 0;
            }


            if count > local_max {local_max = count;}
            row -= 1;
            col -= 1;
        }
        if local_max > max_count {max_count = local_max;}
        if count > max_count {max_count = count;}
        return max_count;

    }
}
