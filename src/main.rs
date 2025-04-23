
// globally disable warnings about non-snake names
#![allow(non_snake_case)]

// if this is not here we will not be able to import it from other `mod`s
mod piece;
// use piece::Piece;

mod player;
use player::Player;

mod board;
use board::Board;

mod input;

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

        let piece = board.select_friendly_piece(&player_a);

        print!("piece: ");
        piece.draw();
        println!();

        break;

    }

}
