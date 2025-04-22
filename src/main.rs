

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
enum CanJump {
    Yes,
    No,
}
type AllowedMoves = Vec<(Repeatable, CanJump, MovePath)>;

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
                (Repeatable::No, CanJump::No, vec![Direction::Forward])
            ],
        }
    }

    fn knight() -> Self {
        Piece {
            icon:  "♞".to_string(),
            allowed_moves_regular: vec![
                (Repeatable::No, CanJump::Yes, vec![Direction::Forward,  Direction::ForwardLeft]),
                (Repeatable::No, CanJump::Yes, vec![Direction::Forward,  Direction::ForwardRight]),
                (Repeatable::No, CanJump::Yes, vec![Direction::Backward, Direction::BackwardLeft]),
                (Repeatable::No, CanJump::Yes, vec![Direction::Backward, Direction::BackwardRight]),
            ],
        }
    }

    fn bighop() -> Self {
        Piece {
            icon:  "♝".to_string(),
            allowed_moves_regular: vec![
                (Repeatable::Yes, CanJump::No, vec![Direction::ForwardLeft]),
                (Repeatable::Yes, CanJump::No, vec![Direction::ForwardRight]),
                (Repeatable::Yes, CanJump::No, vec![Direction::BackwardLeft]),
                (Repeatable::Yes, CanJump::No, vec![Direction::BackwardRight]),
            ],
        }
    }

    fn rook() -> Self {
        Piece {
            icon: "♜".to_string(),
            allowed_moves_regular: vec![
                (Repeatable::Yes, CanJump::No, vec![Direction::Forward]),
                (Repeatable::Yes, CanJump::No, vec![Direction::Backward]),
                (Repeatable::Yes, CanJump::No, vec![Direction::Left]),
                (Repeatable::Yes, CanJump::No, vec![Direction::Right]),
            ],
        }
    }

    fn queen() -> Self {
        Piece {
            icon: "♛".to_string(),
            allowed_moves_regular: vec![
                (Repeatable::Yes, CanJump::No, vec![Direction::Forward]),
                (Repeatable::Yes, CanJump::No, vec![Direction::Backward]),
                (Repeatable::Yes, CanJump::No, vec![Direction::Left]),
                (Repeatable::Yes, CanJump::No, vec![Direction::Right]),
                (Repeatable::Yes, CanJump::No, vec![Direction::ForwardLeft]),
                (Repeatable::Yes, CanJump::No, vec![Direction::ForwardRight]),
                (Repeatable::Yes, CanJump::No, vec![Direction::BackwardLeft]),
                (Repeatable::Yes, CanJump::No, vec![Direction::BackwardRight]),
            ],
        }
    }

    fn king() -> Self {
        Piece {
            icon: "♚".to_string(),
            allowed_moves_regular: vec![
                (Repeatable::No, CanJump::No, vec![Direction::Forward]),
                (Repeatable::No, CanJump::No, vec![Direction::Backward]),
                (Repeatable::No, CanJump::No, vec![Direction::Left]),
                (Repeatable::No, CanJump::No, vec![Direction::Right]),
                (Repeatable::No, CanJump::No, vec![Direction::ForwardLeft]),
                (Repeatable::No, CanJump::No, vec![Direction::ForwardRight]),
                (Repeatable::No, CanJump::No, vec![Direction::BackwardLeft]),
                (Repeatable::No, CanJump::No, vec![Direction::BackwardRight]),
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
