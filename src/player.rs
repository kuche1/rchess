
type Color = (u8, u8, u8);

pub struct Player {
    color: Color,
}

impl Player {

    // `player_num` starts at 1 and ends at `max_players`
    pub fn new(player_num: i32, max_players: i32) -> Self {
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

    pub fn color_on(&self) {
        print!("\x1b[38;2;{};{};{}m", self.color.0, self.color.1, self.color.2);
    }

    pub fn color_off(&self) {
        print!("\x1b[0;0m");
    }

    pub fn same_as(&self, other: &Player) -> bool {
        return self.color == other.color;
        // using the color as an identificator is now awesome (however it will be OK as long as we keep using the "spectrum" color generator, or anything that doesn't repear colors)
        // also, there should be a better way to do this
    }

}
