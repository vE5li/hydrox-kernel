use steckhalma::*;
use graphics::Framebuffer;

pub struct DrawSettings {
    pub colour_background: u32,
    pub colour_border: u32,
    pub colour_nopeg: u32,
    pub colour_peg: u32,
    pub colour_text: u32,
    pub colour_cursor_moving: u32,
    pub colour_cursor_selected: u32,
    pub field_size: usize,
    pub margin: usize,
}

impl DrawSettings {
    fn total_size(&self) -> usize {
        self.field_size * 7 + self.margin * 9
    }
}

pub fn draw_tile(
    fb: &mut Framebuffer,
    settings: &DrawSettings,
    offset: (usize, usize),
    board: &Board,
    game_state: &GameState,
    tile_pos: &Pos,
    highlighted: bool,
) {
    let colour = match board.at(*tile_pos) {
        Position::Invalid => settings.colour_background,
        Position::NoPeg => settings.colour_nopeg,
        Position::Peg => settings.colour_peg,
    };

    let border = match highlighted {
        true => match game_state {
            GameState::MovingPeg => settings.colour_cursor_selected,
            _other => settings.colour_cursor_moving,
        },
        false => colour,
    };

    fb.draw_rectangle(
        offset.0 + tile_pos.x * (settings.field_size + settings.margin) + settings.margin,
        offset.1 + tile_pos.y * (settings.field_size + settings.margin) + settings.margin,
        settings.field_size,
        settings.field_size,
        colour,
        border,
    )
}

pub fn draw(
    framebuffer: &mut Framebuffer,
    settings: &DrawSettings,
    offset: (usize, usize),
    board: &Board,
    game_state: &GameState,
    cursor_pos: Pos,
) {
    framebuffer.draw_rectangle(
        offset.0,
        offset.1,
        settings.total_size(),
        settings.total_size(),
        settings.colour_background,
        settings.colour_border,
    );

    for x in 0..7 {
        for y in 0..7 {
            let highlighted = (cursor_pos.x, cursor_pos.y) == (x, y);
            draw_tile(framebuffer, settings, offset, board, game_state, &Pos::new(x, y), highlighted);
        }
    }
}

pub fn draw_end_screen(
    framebuffer: &mut Framebuffer,
    settings: &DrawSettings,
    offset: (usize, usize),
    end_game_state: EndGameState,
) {
    framebuffer.draw_rectangle(
        offset.0,
        offset.1,
        settings.total_size(),
        settings.total_size(),
        settings.colour_background,
        settings.colour_border,
    );

    let text = match end_game_state {
        EndGameState::PlayerWon => "You won!",
        EndGameState::PlayerLost => "Better luck next time",
    };

    framebuffer.draw_text(offset.0 + 30, offset.1 + 40, text, settings.colour_text, settings.colour_background);
}
