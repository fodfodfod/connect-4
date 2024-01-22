use std::result::Result; use std::time::SystemTime;
//number of rows
const ROWS: i32 = 6;
//number of columns
const COLUMNS: i32 = 7;
// is half of actual depth
const DEPTH: usize = 3;

 
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
    for i in 0..DEPTH{
        if array[i] == COLUMNS{
            array[i] = 0;
            if i < DEPTH -1{
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
        if column >= COLUMNS {
            return Err("column to high");
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
        let mut best_array: (i32, [i32; DEPTH]) = (0, [0; DEPTH]);
        let mut array: (i32, [i32; DEPTH]) = (0, [0; DEPTH]);
        loop{
            //println!("{}", array.0);
            //println!("{}, {}, {}, {}, {}", array.1[0], array.1[1], array.1[2], array.1[3], array.1[4]);

            //increase array by 1
            add_one(&mut array.1);
            if array.1[DEPTH -1] == COLUMNS - 1{
                break;
            }
            let mut board = self.to_owned();
            for i in 0..DEPTH{
                let mut board = self.to_owned();
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
                let position = board.add_piece(array.1[i], local_team);
                match position{
                    Err(_) =>break,
                    Ok(value) => {
                        //if a better solution is found
                        if i == DEPTH -1 && self.count_score(local_team, value) > best_array.0{
                            println!("better solution found");
                            best_array.1=array.1;
                        }
                    }
                }
            }
        }

        //return board.clone().add_piece(best_array.1[0], team).unwrap();
        return best_array.1[0];
        

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
