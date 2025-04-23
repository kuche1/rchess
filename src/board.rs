
// this is how you import a `mod` from another `mod`
// make sure `mod piece` is in `main.rs`
use super::piece::Piece;

use super::Player;
// wtf, this is a typo and yet it works
// perhaps `super` means "get it from `main.rs`"

use super::input;

use std::io;
use std::io::Write;

const BOARD_WIDTH: i32 = 8;
const BOARD_HEIGHT: i32 = 8;

struct BoardPiece<'a> { // the BoardPiece's lifetime is the same as the owner's -> we cannot use BoardPiece if owner's memory has been freed
    piece: Piece,
    owner: &'a Player,
}

impl BoardPiece<'_> { // or: impl<'a> BoardPiece<'a>
    fn draw(&self) {
        self.owner.color_on();
        self.piece.draw();
        self.owner.color_off();
    }
}

pub struct Board<'a> {
    tiles: Vec<Vec<Option<BoardPiece<'a>>>>, // good enough for a simple game
}

impl<'a> Board<'a> {

    //////
    ////// constructors
    //////

    pub fn standard(you: &'a Player, opponent: &'a Player) -> Self { // take a look at the lifetime 'a -> Board, you, opponent
        Board {
            tiles: vec![
                vec![
                    Some(BoardPiece{piece: Piece::rook(),   owner: opponent}),
                    Some(BoardPiece{piece: Piece::knight(), owner: opponent}),
                    Some(BoardPiece{piece: Piece::bishop(), owner: opponent}),
                    Some(BoardPiece{piece: Piece::king(),   owner: opponent}),
                    Some(BoardPiece{piece: Piece::queen(),  owner: opponent}),
                    Some(BoardPiece{piece: Piece::bishop(), owner: opponent}),
                    Some(BoardPiece{piece: Piece::knight(), owner: opponent}),
                    Some(BoardPiece{piece: Piece::rook(),   owner: opponent}),
                ],

                vec![
                    Some(BoardPiece{piece: Piece::pawn(), owner: opponent}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: opponent}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: opponent}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: opponent}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: opponent}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: opponent}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: opponent}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: opponent}),
                ],

                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],

                vec![
                    Some(BoardPiece{piece: Piece::pawn(), owner: you}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: you}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: you}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: you}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: you}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: you}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: you}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: you}),
                ],

                vec![
                    Some(BoardPiece{piece: Piece::rook(),   owner: you}),
                    Some(BoardPiece{piece: Piece::knight(), owner: you}),
                    Some(BoardPiece{piece: Piece::bishop(), owner: you}),
                    Some(BoardPiece{piece: Piece::king(),   owner: you}),
                    Some(BoardPiece{piece: Piece::queen(),  owner: you}),
                    Some(BoardPiece{piece: Piece::bishop(), owner: you}),
                    Some(BoardPiece{piece: Piece::knight(), owner: you}),
                    Some(BoardPiece{piece: Piece::rook(),   owner: you}),
                ],
            ],
        }
    }

    //////
    ////// user input
    //////

    pub fn select_friendly_piece(&self, player: &Player) {
        loop {
            player.color_on();
            print!("player");
            player.color_off();
            print!(", select a piece (example d2) > ");
            io::stdout().flush().unwrap();

            let (pos_x, pos_y) = match input::letter_bound_normalised__number_bound_normalised(
                'a', ('a' as i32 + BOARD_WIDTH - 1) as u8 as char, 1, BOARD_HEIGHT
            ) {
                None => {
                    println!("invalid input");
                    continue;
                }
                Some(val) => val,
            };

            let pos_y = BOARD_HEIGHT as usize - 1 - pos_y;

            // println!("input: y={} x={}", pos_x, pos_y);

            let piece = match &self.tiles[pos_y][pos_x] {
                None => {
                    println!("there is no piece on that position");
                    continue;
                }
                Some(v) => v,
            };

            if !piece.owner.same_as(player) {
                println!("you don't own that piece");
                continue;
            }

            print!("piece: ");
            piece.draw();
            println!();
        }
    }

    //////
    ////// draw
    //////

    fn draw_line(&self, line_idx: usize, pad:bool) -> usize { // TODO this needs to go
        if pad {
            print!(" ");
        }
        print!("|");
        for piece in &self.tiles[line_idx] {
            match piece {
                Some(piece) => piece.draw(),
                None => print!(" "),
            }
            print!("|");
        }

        self.tiles[line_idx].len()
    }

    pub fn draw(&self) {
        print!(" |");
        for idx in 0..BOARD_WIDTH {
            print!("{}|", char::from_u32( ('a' as i32 + idx) as u32 ).unwrap());
        }
        println!();

        let len = self.tiles.len();
        for (line_idx, _line) in self.tiles.iter().enumerate() {
            let line_ui_num = len - line_idx;
            print!("{}", line_ui_num);
            self.draw_line(line_idx, false);
            println!("{}", line_ui_num);
        }

        print!(" |");
        for idx in 0..BOARD_WIDTH {
            print!("{}|", char::from_u32( ('a' as i32 + idx) as u32 ).unwrap());
        }
        println!();
    }

}
