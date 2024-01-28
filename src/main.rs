use std::collections::HashMap;
use std::result::Result;
use std::time::SystemTime;
//number of rows
const ROWS: i32 = 7;
//number of columns
const COLUMNS: i32 = 7;

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
        let position = match user_input.parse::<i32>() {
            Ok(value) => match board.add_piece(value, Spot::Red) {
                Ok(position) => position,
                Err(..) => {
                    println!("you bad (not a valid number)");
                    continue;
                }
            },
            _ => {
                println!("you bad! (not a number)");
                continue;
            }
        };
        board.display();
        if board.check_win(Spot::Red, position.0, position.1) {
            println!("red wins");
            break;
        }
        println!("AI going");
        let now = SystemTime::now();
        let position = board
            .add_piece(board.find_next_move(Spot::Yellow), Spot::Yellow)
            .unwrap();
        println!("the ai took {} seconds", now.elapsed().unwrap().as_secs());
        if board.check_win(Spot::Yellow, position.0, position.1) {
            println!("yellow wins");
            break;
        }
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
enum Spot {
    Red,
    Yellow,
    Blank,
}

#[derive(Clone, Copy)]
struct Board {
    board: [Spot; (ROWS * COLUMNS) as usize],
}

fn add_one(array: &mut [i32], start_at_zero: bool) {
    if start_at_zero {
        array[0] += 1;
        let mut i = 0;
        loop {
            if array[i] == COLUMNS {
                array[i] = 0;
                if i < array.len() - 1 {
                    array[i + 2] += 1;
                }
            }
            i += 2;
            if i >= array.len() {
                break;
            }
        }
    } else {
        array[1] += 1;
        let mut i = 1;
        loop {
            if array[i] == COLUMNS {
                array[i] = 0;
                if i < array.len() - 1 {
                    array[i + 2] += 1;
                }
            }
            i += 2;
            if i >= array.len() - 1 {
                break;
            }
        }
    }
}
impl Board {
    fn new() -> Self {
        Board {
            board: [Spot::Blank; (ROWS * COLUMNS) as usize],
        }
    }
    fn add_piece(&mut self, column: i32, color: Spot) -> Result<(i32, i32), &'static str> {
        if column >= COLUMNS || column < 0 {
            return Err("column to high or low");
        }
        let mut row = 0;
        while row < ROWS {
            if self.board[(column as usize) + (COLUMNS * row) as usize] == Spot::Blank {
                self.board[(column as usize) + (COLUMNS * row) as usize] = color;
                //return Ok(column + (COLUMNS * row));
                return Ok((row, column));
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

    fn find_next_move(&self, team: Spot) -> i32 {
        let client = reqwest::blocking::Client::new();

        //first is position, second is score
        let mut best_option = (-1000, -1000);
        let mut options = vec![0, 1, 2, 3, 4, 5, 6];
        let mut map = HashMap::new();
        map.insert("board_data".to_string(), self.to_string());
        map.insert("player".to_string(), "2".to_string());

        println!("the board is: {}", self.to_string());
        let mut result =
            client.post(format!("https://kevinalbs.com/connect4/back-end/index.php/getMoves?board_data={}&player=2", self.to_string()))
            //client.post("https://kevinalbs.com/connect4/back-end/index.php/getMoves")
                //.json(&map)
                .send()
                .unwrap()
                .json::<HashMap<String, i32>>();
        for i in result.as_mut().unwrap(){
            //options.remove(i.0.parse::<usize>().unwrap());
            options.remove(options.iter().position(|x| *x == i.0.parse::<usize>().unwrap()).unwrap());
            println!("position: {}, score: {}", i.0, i.1);
            if i.1 >= &mut best_option.1{
                best_option.0 = i.0.parse::<i32>().unwrap() ;
                best_option.1 = *i.1;
            }
        }
        println!("{:#?}", result);
        println!("the best column is: {}", best_option.0);
        println!("the missing option is {:?}", options);
        return best_option.0;
        //return options[0].try_into().unwrap();
    }
    fn to_string(&self) -> String {
        let mut string = String::new();
        /*for i in self.board {
            string.push_str(match i {
                Spot::Red => "1",
                Spot::Yellow => "2",
                Spot::Blank => "0",
            })
        }*/
        for row in (0..ROWS).rev() {
            for col in (0..COLUMNS){
                string.push_str(match self.board[((row*7) + col) as usize] {
                Spot::Red => "1",
                Spot::Yellow => "2",
                Spot::Blank => "0",
            })
            }
        }
        return string;
    }
    fn check_win(&self, last_team: Spot, row: i32, column: i32) -> bool {
        let client = reqwest::blocking::Client::new();
        let last_team_number = match last_team {Spot::Red => 1, Spot::Yellow => 2, _ => panic!()};
        let mut result =
            client.post(format!("https://kevinalbs.com/connect4/back-end/index.php/hasWon?board_data={}&player={}&i={}&j={}", self.to_string(),last_team_number, row, column))
            //client.post("https://kevinalbs.com/connect4/back-end/index.php/getMoves")
                //.json(&map)
                .send()
                .unwrap().text().unwrap().parse::<bool>();
        return result.unwrap();

    }
}
