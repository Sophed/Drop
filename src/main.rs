use rand::Rng;

use draw::*;
use input::*;
use raylib::prelude::*;
mod draw;
mod input;

const TRACKS: i32 = 4;
const INTERVAL_SECONDS: f64 = 0.52;
const ENEMY_SPEED: f32 = 0.16;

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
    let mut missed = 0;
    let mut player_track = 1;
    let mut last_spawn = 0.0;
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

        if d.get_time() - last_spawn > INTERVAL_SECONDS {
            last_spawn = d.get_time();
            let track = rand::thread_rng().gen_range(0..TRACKS);
            enemies.push(Enemy {
                track: track,
                y: 0.0,
                id: d.get_time() as i32,
            });
        }

        let mut purge: Vec<i32> = Vec::new();
        for enemy in &mut enemies {
            enemy.y += ENEMY_SPEED;
            if enemy.y > d.get_screen_height() as f32 {
                purge.push(enemy.id);
                missed += 1;
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
            let e = enemies.iter().position(|x| x.id == id).unwrap();
            enemies.remove(e);
        }

        d.draw_text(&format!("Score: {}", score), 20, 20, 30, Color::WHITE);
        d.draw_text(&format!("Missed: {}", missed), 20, 60, 30, Color::WHITE);
        draw_player(&mut d, player_track);

    }
}