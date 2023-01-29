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

    //Por cada X en la FILA -->
    show_array_visually(maze);   

    //TEST
    //Funcional, muestra todo el array y lo deja visualmente aceptable
    show_array_simple(maze);

    //TEST
    //Funcional, comprueba que la posición donde se quiera mover la entidad pueda
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

fn show_array_visually(array: [[&str; 10]; 9]) {
    for (x, row) in array.iter().enumerate() {
        print!("{},", x);
        //Por cada Y dentro de la FILA -->
        for (y, col) in row.iter().enumerate() {
            print!(" {} : {} ||", y, col);
        }
        print!("\n");
    }
}

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

//Comprueba si la posición dada dentro del array es válida o si es el Theseus o la salida
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

//comprueba los espacios de alrededor hacia los que el personaje puede moverse
fn check_around(array: [[&str; 10];9], position: Position) -> VecDeque<Position> {
    println!("These are the positions where you can move from: [{}][{}]", position.posX, position.posY);
    let Position{posX, posY} = position;
    //VecDeque permite el uso de Queues y sacar y meter objetos al final y al principio de estas
    let mut states = VecDeque::new();

    for (dx, dy) in [(1,0),(0,1),(-1,0),(0,-1)] {
        let nx = posX + dx;
        let ny = posY + dy;

         if(is_position_valid(array, nx as usize, ny as usize)) {
            //muestra la posición hacia la que puede avanzar
            println!("position valid at -> [{}][{}]", nx, ny);
            states.push_back(Position{posX: nx, posY: ny});
         }
    }

    let state_pop = states.pop_front().expect("Error");
    println!("{:?}", state_pop);

    return states
}