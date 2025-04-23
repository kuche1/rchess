
use std::io;
use std::io::Write;

// if this is not here we will not be able to import it from other `mod`s
mod piece;
// use piece::Piece;

mod player;
use player::Player;

mod board;
use board::Board;

//////
////// user input
//////

fn get_num(prompt: &str, from:usize, to:usize) -> usize {
    loop {
        print!("{} [{}:{}] > ", prompt, from, to);
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let number = input.trim().parse::<usize>();

        if let Err(_err) = number {
            println!("not a number or invaid");
            continue;
        }

        let number = number.unwrap();

        if number < from {
            println!("number too small");
            continue;
        }

        if number > to {
            println!("number too big");
            continue;
        }

        return number;
    }
}

//////
////// main
//////

fn main() {
    // print!("Hello, rook ");
    // let rook = Piece::rook();
    // rook.draw();
    // println!();

    let player_a = Player::new(1, 2);
    let player_b = Player::new(2, 2);

    let board = Board::standard(&player_a, &player_b);

    loop {

        // println!();

        // let num_lines = board.draw();
        // let pos_y = get_num("select line", 0, num_lines-1);

        // println!();
        // let num_tiles = board.draw_line(pos_y, true);
        // println!();
        // let pos_x = get_num("select piece", 0, num_tiles-1);

        // let piece = board.get_piece_at(pos_x, pos_y);

        // let piece = match piece {
        //     None => {
        //         println!("there is no piece at that position");
        //         // std::process::exit(1);
        //         continue;
        //     },
        //     Some(val) => val,
        // };

        // if !piece.owner.same_as(&player_a) {
        //     println!("you don't own that piece");
        //     continue;
        // }

        // println!();
        // print!("yay you will move the ");
        // piece.draw();
        // println!();

        board.draw();

        break;

    }

}
