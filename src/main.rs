
use std::io;
use std::io::Write;

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

    // `player_num` starts at 1 and ends at `max_players`
    fn new(player_num: i32, max_players: i32) -> Self {
        let max = 255 * 255 * 255;
        let step = max / (max_players + 1);
        let current = step * player_num;

        let r = current / (255 * 255);
        let current = current - r * 255 * 255;

        let g = current / 255;
        let current = current - g * 255;

        let b = current;

        // TODO would be better (visually) if instead we had a couple of base colors (r, g, b, r+g, g+b, r+b)
        // we picked one of them, then we changed the brightness
        // what we have here is awesome for a lot of players, the problem is that with 1/2 and 2/2 we get bright red and dark red

        Player {
            color: (r.try_into().unwrap(), g.try_into().unwrap(), b.try_into().unwrap()),
        }
    }

    fn color_on(&self) {
        print!("\x1b[38;2;{};{};{}m", self.color.0, self.color.1, self.color.2);
    }

    fn color_off(&self) {
        print!("\x1b[0;0m");
    }

    fn same_as(&self, other: &Player) -> bool {
        return self.color == other.color;
        // using the color as an identificator is now awesome
        // also, there should be a better way to do this
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
        self.owner.color_on();
        self.piece.draw(); // TODO needs to also be colored in the player's color
        self.owner.color_off();
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
    ////// get piece
    //////

    fn get_piece_at(&self, x:usize, y:usize) -> &Option<BoardPiece> {
        return &self.tiles[y][x];
    }

    //////
    ////// draw
    //////

    fn draw_line(&self, line_idx: usize, show_idx: bool) -> usize {
        if show_idx {
            print!(" ");
            for idx in 0 .. self.tiles[line_idx].len() {
                print!("{} ", idx);
            }
            println!();
        }

        print!("|");
        for piece in &self.tiles[line_idx] {
            match piece {
                Some(piece) => piece.draw(),
                None => print!(" "),
            }
            print!("|");
        }
        println!();

        self.tiles[line_idx].len()
    }

    fn draw(&self) -> usize {
        for (line_idx, _line) in self.tiles.iter().enumerate() {
            print!("{}", line_idx);
            self.draw_line(line_idx, false);
        }

        self.tiles.len()
    }

}

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

        println!();
        let num_lines = board.draw();

        let pos_y = get_num("select line", 0, num_lines-1);
        println!();
        let num_tiles = board.draw_line(pos_y, true);

        let pos_x = get_num("select piece", 0, num_tiles-1);

        let piece = board.get_piece_at(pos_x, pos_y);

        let piece = match piece {
            None => {
                println!("there is no piece at that position");
                // std::process::exit(1);
                continue;
            },
            Some(val) => val,
        };

        if !piece.owner.same_as(&player_a) {
            println!("you don't own that piece");
            continue;
        }

        println!();
        print!("yay you will move the ");
        piece.draw();
        println!();

        break;

    }

}
