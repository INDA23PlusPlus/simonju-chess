use api;
use std::io;

fn main() {
    let mut board = api::board();
    println!("CHESS");
    loop {
        board.print_tiles();
        println!("Enter ply (or q to quit): ");

        let mut str = String::new();
        io::stdin().read_line(&mut str)
            .expect("Error reading line");
        
        if str == "q\n" {
            break;
        }

        let mut nums: Vec<_> = str.split_whitespace()
            .filter_map(|n| Some(n.parse::<i32>().ok().unwrap())).collect();

        board.print_tile(api::Pos { 
            x: nums[0],
            y: nums[1], 
        });
    }
}
