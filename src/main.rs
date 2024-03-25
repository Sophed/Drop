use draw::*;
use input::*;
use raylib::prelude::*;
mod draw;
mod input;

const TRACKS: i32 = 4;
const _INTERVAL_SECONDS: f64 = 1.0;
const ENEMY_SPEED: f32 = 0.1;

struct Enemy {
    track: i32,
    y: f32,
    id: i32,
}

fn main() {

    let (mut rl, thread) = raylib::init()
        .size(1100, 620)
        .title("Drop")
        .build();

    let mut score = 0;
    let mut player_track = 1;
    let mut enemies: Vec<Enemy> = Vec::new();

    enemies.push(Enemy {
        track: 0,
        y: 0.0,
        id: 0,
    });
     
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        draw_tracks(&mut d, TRACKS);
        player_track = input(&mut d, &mut player_track, TRACKS);

        let mut purge: Vec<i32> = Vec::new();
        for enemy in &mut enemies {
            enemy.y += ENEMY_SPEED;
            if enemy.y > d.get_screen_height() as f32 {
                purge.push(enemy.id);
            }
            if enemy.track == player_track &&
                ((enemy.y > 500.0 && enemy.y < 550.0) ||
                (enemy.y + 50.0 > 500.0 && enemy.y + 50.0 < 550.0)) {
                purge.push(enemy.id);
                score += 1;
            }
            d.draw_rectangle(
                160 + (enemy.track) * 250,
                enemy.y as i32,
                14,
                50,
                Color::RED
            );
        }
        for id in purge {
            enemies.retain(|e| e.id != id);
        }

        d.draw_text(&format!("Score: {}", score), 20, 20, 30, Color::WHITE);
        draw_player(&mut d, player_track);

    }
}