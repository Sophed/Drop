use raylib::prelude::*;

pub fn input(d: &mut RaylibDrawHandle, player_track: &mut i32, max: i32) -> i32 {

    if d.is_key_pressed(KeyboardKey::KEY_RIGHT) && player_track < &mut (max - 1) {
        *player_track += 1;
        return *player_track;
    }
    if d.is_key_pressed(KeyboardKey::KEY_LEFT) && player_track > &mut 0 {
        *player_track -= 1;
        return *player_track;
    }

    if d.is_key_pressed(KeyboardKey::KEY_D) {
        return 0;
    }
    if d.is_key_pressed(KeyboardKey::KEY_F) {
        return 1;
    }
    if d.is_key_pressed(KeyboardKey::KEY_J) {
        return 2;
    }
    if d.is_key_pressed(KeyboardKey::KEY_K) {
        return 3;
    }
    return *player_track;
}