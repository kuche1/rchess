

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

    fn bighop() -> Self {
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

fn main() {
    print!("Hello, rook ");
    let rook = Piece::rook();
    rook.draw();
    println!();
}
