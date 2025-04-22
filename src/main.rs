

//////
////// piece
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
////// player
//////

type Color = (u8, u8, u8);

struct Player {
    color: Color,
}

impl Player {
    fn new() -> Self {
        Player {
            color: (255, 255, 255),
        }
    }
}

//////
////// board
//////

struct BoardPiece<'a> { // the BoardPiece's lifetime is the same as the owner's -> we cannot use BoardPiece if owner's memory has been freed
    piece: Piece,
    owner: &'a Player,
}

impl BoardPiece<'_> { // or: impl<'a> BoardPiece<'a>
    fn draw(&self) {
        // self.owner.color_on();
        self.piece.draw(); // TODO needs to also be colored in the player's color
        // self.owner.color_off();
    }
}

struct Board<'a> {
    tiles: Vec<Vec<Option<BoardPiece<'a>>>>, // good enough for a simple game
}

impl<'a> Board<'a> {

    //////
    ////// constructors
    //////

    fn standard(you: &'a Player, opponent: &'a Player) -> Self { // take a look at the lifetime 'a -> Board, you, opponent
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
    ////// methods
    //////
    
    fn draw(&self) {
        for line in &self.tiles {

            // for _ in 0 .. line.len()*2+1 {
            //     print!("-");
            // }
            // println!();

            print!("|");

            for piece in line {
                match piece {
                    Some(piece) => piece.draw(),
                    None => print!(" "),
                }
                print!("|");
            }
            println!();
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

    let player_a = Player::new();
    let player_b = Player::new();

    let board = Board::standard(&player_a, &player_b);
    board.draw();
}
