

//////
////// pieces
//////

enum Direction {
    Forward,
    Backward,

    Left,
    Right,

    ForwardLeft,
    ForwardRight,

    BackwardLeft,
    BackwardRight,
}

type MovePath = Vec<Direction>;

enum Repeatable {
    Yes,
    No,
}
type AllowedMoves = Vec<(Repeatable, MovePath)>;

struct Piece {
    icon: String,
    allowed_moves_regular: AllowedMoves,
    // TODO must also add special moves like castle and passant
}

impl Piece {

    //////
    ////// constructors
    //////

    fn pawn() -> Self {
        Piece {
            icon:  "♟︎".to_string(),
            allowed_moves_regular: vec![
                (Repeatable::No, vec![Direction::Forward])
            ],
        }
    }

    fn knight() -> Self {
        Piece {
            icon:  "♞".to_string(),
            allowed_moves_regular: vec![
                (Repeatable::No, vec![Direction::Forward,  Direction::ForwardLeft]),
                (Repeatable::No, vec![Direction::Forward,  Direction::ForwardRight]),
                (Repeatable::No, vec![Direction::Backward, Direction::BackwardLeft]),
                (Repeatable::No, vec![Direction::Backward, Direction::BackwardRight]),
            ],
        }
    }

    fn bishop() -> Self {
        Piece {
            icon:  "♝".to_string(),
            allowed_moves_regular: vec![
                (Repeatable::Yes, vec![Direction::ForwardLeft]),
                (Repeatable::Yes, vec![Direction::ForwardRight]),
                (Repeatable::Yes, vec![Direction::BackwardLeft]),
                (Repeatable::Yes, vec![Direction::BackwardRight]),
            ],
        }
    }

    fn rook() -> Self {
        Piece {
            icon: "♜".to_string(),
            allowed_moves_regular: vec![
                (Repeatable::Yes, vec![Direction::Forward]),
                (Repeatable::Yes, vec![Direction::Backward]),
                (Repeatable::Yes, vec![Direction::Left]),
                (Repeatable::Yes, vec![Direction::Right]),
            ],
        }
    }

    fn queen() -> Self {
        Piece {
            icon: "♛".to_string(),
            allowed_moves_regular: vec![
                (Repeatable::Yes, vec![Direction::Forward]),
                (Repeatable::Yes, vec![Direction::Backward]),
                (Repeatable::Yes, vec![Direction::Left]),
                (Repeatable::Yes, vec![Direction::Right]),
                (Repeatable::Yes, vec![Direction::ForwardLeft]),
                (Repeatable::Yes, vec![Direction::ForwardRight]),
                (Repeatable::Yes, vec![Direction::BackwardLeft]),
                (Repeatable::Yes, vec![Direction::BackwardRight]),
            ],
        }
    }

    fn king() -> Self {
        Piece {
            icon: "♚".to_string(),
            allowed_moves_regular: vec![
                (Repeatable::No, vec![Direction::Forward]),
                (Repeatable::No, vec![Direction::Backward]),
                (Repeatable::No, vec![Direction::Left]),
                (Repeatable::No, vec![Direction::Right]),
                (Repeatable::No, vec![Direction::ForwardLeft]),
                (Repeatable::No, vec![Direction::ForwardRight]),
                (Repeatable::No, vec![Direction::BackwardLeft]),
                (Repeatable::No, vec![Direction::BackwardRight]),
            ],
        }
    }

    //////
    ////// methods
    //////

    fn draw(&self) {
        print!("{}", self.icon);
    }

}

//////
////// board
//////

enum Player {
    A,
    B,
}

struct BoardPiece {
    piece: Piece,
    owner: Player,
}

impl BoardPiece {
    // fn empty() -> Self {
    //     BoardPiece {
    //         empty: true,
    //         piece, 
    //     }
    // }
    // fn new() -> Self {
    //     BoardPiece { piece: (), owner: () }
    // }
}

struct Board {
    tiles: Vec<Vec<Option<BoardPiece>>>, // good enough for a simple game
}

impl Board {
    fn new() -> Self {
        Board {
            tiles: vec![
                vec![
                    Some(BoardPiece{piece: Piece::rook(),   owner: Player::B}),
                    Some(BoardPiece{piece: Piece::bishop(), owner: Player::B}),
                    Some(BoardPiece{piece: Piece::knight(), owner: Player::B}),
                    Some(BoardPiece{piece: Piece::king(),   owner: Player::B}),
                    Some(BoardPiece{piece: Piece::queen(),  owner: Player::B}),
                    Some(BoardPiece{piece: Piece::knight(), owner: Player::B}),
                    Some(BoardPiece{piece: Piece::bishop(), owner: Player::B}),
                    Some(BoardPiece{piece: Piece::rook(),   owner: Player::B}),
                ],

                vec![
                    Some(BoardPiece{piece: Piece::pawn(), owner: Player::B}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: Player::B}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: Player::B}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: Player::B}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: Player::B}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: Player::B}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: Player::B}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: Player::B}),
                ],

                vec![],
                vec![],
                vec![],
                vec![],

                vec![
                    Some(BoardPiece{piece: Piece::pawn(), owner: Player::A}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: Player::A}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: Player::A}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: Player::A}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: Player::A}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: Player::A}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: Player::A}),
                    Some(BoardPiece{piece: Piece::pawn(), owner: Player::A}),
                ],

                vec![
                    Some(BoardPiece{piece: Piece::rook(),   owner: Player::A}),
                    Some(BoardPiece{piece: Piece::bishop(), owner: Player::A}),
                    Some(BoardPiece{piece: Piece::knight(), owner: Player::A}),
                    Some(BoardPiece{piece: Piece::queen(),  owner: Player::A}),
                    Some(BoardPiece{piece: Piece::king(),   owner: Player::A}),
                    Some(BoardPiece{piece: Piece::knight(), owner: Player::A}),
                    Some(BoardPiece{piece: Piece::bishop(), owner: Player::A}),
                    Some(BoardPiece{piece: Piece::rook(),   owner: Player::A}),
                ],
            ],
        }
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

    let board = Board::new();
}
