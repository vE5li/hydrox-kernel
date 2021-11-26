use crate::*;

pub struct Framebuffer {}
impl Framebuffer {
    fn draw_pixel(&mut self, x_position: usize, y_position: usize, color: u32) {}
    fn draw_rectangle(
        &mut self,
        x_position: usize,
        y_position: usize,
        width: usize,
        height: usize,
        fill_color: u32,
        border_color: u32,
    ) {
    }
}

pub struct DrawSettings {
    colour_background: u32,
    colour_border: u32,
    colour_invalid: u32,
    colour_nopeg: u32,
    colour_peg: u32,
    colour_highlight: u32,
    field_size: usize,
    margin: usize,
}

impl DrawSettings {
    fn total_size(&self) -> usize {
        self.field_size * 7 + self.margin * 9
    }
}

pub fn draw(
    fb: &mut Framebuffer,
    settings: &DrawSettings,
    offset: (usize, usize),
    board: &Board,
    highlight_pos: Pos,
) {
    fb.draw_rectangle(
        offset.0,
        offset.1,
        settings.total_size(),
        settings.total_size(),
        settings.colour_background,
        settings.colour_border,
    );
    for x in 0..7 {
        for y in 0..7 {
            let colour = match board.at(Pos::new(x, y)) {
                Position::Invalid => settings.colour_invalid,
                Position::NoPeg => settings.colour_nopeg,
                Position::Peg => settings.colour_peg,
            };
            let border = if (highlight_pos.x, highlight_pos.y) == (x, y) {
                settings.colour_highlight
            } else {
                colour
            };
            fb.draw_rectangle(
                x * (settings.field_size + settings.margin) + settings.margin,
                y * (settings.field_size + settings.margin) + settings.margin,
                settings.field_size,
                settings.field_size,
                colour,
                border,
            )
        }
    }
}
