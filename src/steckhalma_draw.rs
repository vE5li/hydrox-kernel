use steckhalma::*;
use graphics::Framebuffer;

//pub struct Framebuffer {}
//impl Framebuffer {
//    fn draw_pixel(&mut self, x_position: usize, y_position: usize, color: u32) {}
//    fn draw_rectangle(
//        &mut self,
//        x_position: usize,
//        y_position: usize,
//        width: usize,
//        height: usize,
//        fill_color: u32,
//        border_color: u32,
//    ) {
//    }
//}

pub struct DrawSettings {
    pub colour_background: u32,
    pub colour_border: u32,
    pub colour_nopeg: u32,
    pub colour_peg: u32,
    pub colour_highlight: u32,
    pub field_size: usize,
    pub margin: usize,
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
    cursor_pos: Pos,
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
                Position::Invalid => settings.colour_background,
                Position::NoPeg => settings.colour_nopeg,
                Position::Peg => settings.colour_peg,
            };
            let border = if (cursor_pos.x, cursor_pos.y) == (x, y) {
                settings.colour_highlight
            } else {
                colour
            };
            fb.draw_rectangle(
                offset.0 + x * (settings.field_size + settings.margin) + settings.margin,
                offset.1 + y * (settings.field_size + settings.margin) + settings.margin,
                settings.field_size,
                settings.field_size,
                colour,
                border,
            )
        }
    }
}
