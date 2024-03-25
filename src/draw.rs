use raylib::prelude::*;

pub fn draw_tracks(d: &mut RaylibDrawHandle, tracks: i32) {
    for i in 0..tracks {
        d.draw_rectangle(
            160 + (i) * 250,
            0,
            14,
            d.get_screen_height(),
            Color::new(10, 10, 10, 255)
        )
    }
}

pub fn draw_player(d: &mut RaylibDrawHandle, track: i32) {
    d.draw_rectangle(
        160 + (track) * 250,
        500,
        14,
        50,
        Color::GREEN
    )
}