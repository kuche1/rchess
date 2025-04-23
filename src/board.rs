
// this is how you import a `mod` from another `mod`
// make sure `mod piece` is in `main.rs`
use super::piece;
use super::piece::Piece;

use super::Player;
// wtf, this is a typo and yet it works
// perhaps `super` means "get it from `main.rs`"

use super::input;

use std::io;
use std::io::Write;

const BOARD_WIDTH: i32 = 8;
const BOARD_HEIGHT: i32 = 8;

pub struct BoardPiece<'a> { // the BoardPiece's lifetime is the same as the owner's -> we cannot use BoardPiece if owner's memory has been freed
    piece: Piece,
    owner: &'a Player,
}

impl BoardPiece<'_> { // or: impl<'a> BoardPiece<'a>
    pub fn draw(&self) {
        self.owner.color_on();
        self.piece.draw();
        self.owner.color_off();
    }
}

type PiecePosition = (usize, usize);

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

    pub fn select_friendly_piece(&self, player: &Player) -> (&BoardPiece, PiecePosition) {
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

            return (piece, (pos_x, pos_y));
        }
    }

    //////
    ////// get piece
    //////

    // fn get_piece_at(&self, pos: PiecePosition) -> &BoardPiece {
    //     (&self.tiles[pos.1][pos.0].unwrap()).as_ref()
    // }

    //////
    ////// piece moves
    //////

    pub fn get_piece_available_moves(&self, pos: PiecePosition) -> Vec<PiecePosition> {
        let (pos_x, pos_y) = pos;

        // THIS IS FUCKING CANCER
        // let pic = &(self.tiles[pos_y][pos_x]).unwrap();
        let brummmm_brummmmmmmm = &self.tiles[pos_y];
        let trikitrakatrikitraki = &brummmm_brummmmmmmm[pos_x];
        let yoyoyoyoyoyoyo = &trikitrakatrikitraki.as_ref().unwrap();
        let pic = &yoyoyoyoyoyoyo;

        let mut available_positions:Vec<PiecePosition> = vec![];

        // TODO ignoring `repeatable` for now
        for (repeatable, direction) in &pic.piece.allowed_moves_regular {
            let mut cur_pos_x: isize = pos_x.try_into().unwrap();
            let mut cur_pos_y: isize = pos_y.try_into().unwrap();

            for dir in direction {
                match dir {
                    piece::Direction::Forward => cur_pos_y += pic.owner.forward_y as isize,
                    piece::Direction::Backward => cur_pos_y += pic.owner.backward_y as isize,
                    piece::Direction::Left => cur_pos_x -= 1,
                    piece::Direction::Right => cur_pos_x += 1,

                    piece::Direction::ForwardLeft => {
                        cur_pos_y += pic.owner.forward_y as isize;
                        cur_pos_x -= 1;
                    },
                    piece::Direction::ForwardRight => {
                        cur_pos_y += pic.owner.forward_y as isize;
                        cur_pos_x += 1;
                    },

                    piece::Direction::BackwardLeft => {
                        cur_pos_y += pic.owner.backward_y as isize;
                        cur_pos_x -= 1;
                    },
                    piece::Direction::BackwardRight => {
                        cur_pos_y += pic.owner.backward_y as isize;
                        cur_pos_x += 1;
                    },
                }
            }

            // I fucking hate this
            // I spent so much time on this and it still looks fucky
            let idx_y:Option<usize> = cur_pos_y.try_into().ok();
            let idx_x:Option<usize> = cur_pos_x.try_into().ok();

            let idx_y = match idx_y {
                None => continue,
                Some(v) => v,
            };
            let idx_x = match idx_x {
                None => continue,
                Some(v) => v,
            };

            if let Some(pic_in_the_way) = &self.tiles[idx_y][idx_x] {
                if pic_in_the_way.owner.same_as(pic.owner) {
                    continue;
                }
            }

            available_positions.push((idx_x, idx_y));
        }

        available_positions
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
