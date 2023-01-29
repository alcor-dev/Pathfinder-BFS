use std::collections::VecDeque;

fn main() {
    let maze = [
        ["#","#","#","#","#","#","#","#","#","#"],
        ["#"," ","#"," "," "," "," "," ","E","#"],
        ["#"," ","#"," ","#"," ","#","#","#","#"],
        ["#"," "," "," ","#"," "," "," "," ","#"],
        ["#","#","#"," ","#"," ","#","#"," ","#"],
        ["#"," ","T"," ","#"," "," "," "," ","#"],
        ["#","#"," ","#","#"," ","#"," ","#","#"],
        ["#"," "," ","#"," "," ","#"," "," ","#"],
        ["#","#","#","#","#","#","#","#","#","#"],
    ];

    let lines = "hello\nworld".lines();

    println!("{}", maze.len());
    for (numberline, line) in lines.enumerate() {
        println!("{}: {}", numberline, line)
    }

    let theseus = check_position_entity(maze, "T");
    let exit = check_position_entity(maze, "E");

    println!("{:?} || {:?}", theseus, exit);

    show_array_visually(maze);   

    //TEST
    //Functional, shows all the array and keeps it visually acceptable
    show_array_simple(maze);

    //TEST
    //Functional, checks if the entity can move towards a new position (up, down, left, right)
    let bool: bool = is_position_valid(maze, 3, 2);

    let pos: Position = Position {posX: 3, posY: 2};
    let position_test = check_around(maze, theseus);
    println!("{:?}", position_test);

}

#[derive(Debug)]
struct Position {
    posX : i32,
    posY : i32
}

impl Position {
    fn change_position (&mut self, x: i32, y: i32) {
        self.posX = x;
        self.posY = y;
    }
}

//Little test to know how to check every space
//Mostly inspired by the Java way
fn show_array_simple(array: [[&str; 10]; 9]) {
    let mut counter: u32 = 0;

    println!("\n");
    for row in array.iter() {
        for column in row.iter() {
            print!("{} ", column);
        }
        counter += 1;
        println!("");
    }
}

//Shows a little "map" about how the array is arranged
//think of it like looking at a dungeon map from above
fn show_array_visually(array: [[&str; 10]; 9]) {
    for (x, row) in array.iter().enumerate() {
        print!("{},", x);
        //For every Y inside the ROW
        for (y, col) in row.iter().enumerate() {
            print!(" {} : {} ||", y, col);
        }
        print!("\n");
    }
}

//Given the char of the entity to seek, it will return the position (if there's one, of course)
fn check_position_entity(array: [[&str; 10]; 9], string: &str) -> Position {
    let mut coordinates = Position{ posX: 0, posY: 0};

    for (x, row) in array.iter().enumerate() {
        for (y, column) in row.iter().enumerate() {
            if column.contains(string) {
                coordinates.change_position(x as i32, y as i32);    
            }
        }
    }

    return coordinates
}

//Checks if every position given is valid, and even if the new position is either Theseus or the exit
fn is_position_valid(array: [[&str; 10];9], x: usize, y: usize) -> bool {
    if array[x][y] == "#" && x < array.len() && y < array[0].len() {
        println!("Yes, its not valid -> {}" , array[x as usize][y as usize]);
        return false 
    } else {
        if array[x as usize][y as usize] == "E" {
            println!("It is valid and this is the exit!");
            return true
        } else if array[x as usize][y as usize] == "T" {
            println!("It is valid and this is Theseus!");
            return true
        } else {
            println!("It is valid, there's nothing here!");
            return true
        }
    }
}

//Checks the spaces around Theseus to see if he can move towards that point
fn check_around(array: [[&str; 10];9], position: Position) -> VecDeque<Position> {
    println!("These are the positions where you can move from: [{}][{}]", position.posX, position.posY);
    let Position{posX, posY} = position;
    //VecDeque allows us to use queues and have some operations that can push and pop from the front or the back
    let mut states = VecDeque::new();

    for (dx, dy) in [(1,0),(0,1),(-1,0),(0,-1)] {
        let nx = posX + dx;
        let ny = posY + dy;

         if(is_position_valid(array, nx as usize, ny as usize)) {
            println!("position valid at -> [{}][{}]", nx, ny);
            //Pushes the position (if it is valid) into the back of the list
            states.push_back(Position{posX: nx, posY: ny});
         }
    }

    let state_pop = states.pop_front().expect("Error");
    println!("{:?}", state_pop);

    return states
}