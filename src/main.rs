use std::io;

fn main() {
    println!("== Welcome to the Resistance game! ==");

    // Get the number of player
    let max_player = 10;
    let min_player = 6;
    
    loop {
        let mut input = String::new();

        println!("Please input the number of player: ");
        io::stdin()
            .read_line(&mut input)
            .expect("The input type was incorrect!");
        println!("Current input: {input}");
        let num_player : u32 = input.trim().parse().expect("Please input a number!");

        if num_player >= min_player && num_player <= max_player {
            println!("Great! We have {num_player} player!");
            break;
        } else {
            println!("Please input num player between {min_player} and {max_player}.");
        }
    }
    
    
}
