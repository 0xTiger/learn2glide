mod utils;
mod aircraft;

use macroquad::prelude::*;
use aircraft::Aircraft;


const FLOOR_HEIGHT: f32 = 0.0;


struct Meteor {
    pos: Vec2,
    vel: Vec2
}


impl Meteor {
    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 10., BLACK);
    }

    pub fn update_pos(&mut self) {
        self.pos += self.vel;
    }
}


fn draw_mountain(x: f32, y : f32, w: f32) {
    draw_line(x, y, x + w, FLOOR_HEIGHT, 3.0, GREEN);
    draw_line(x, y, x - w, FLOOR_HEIGHT, 3.0, GREEN);
}


fn draw_text_centered(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let text_dims = measure_text(text, None, font_size as u16, 1.0);
    draw_text(text, x - text_dims.width / 2.0, 
                    y + text_dims.height / 2.0, 
                    font_size, color)
}


async fn death_screen(score: f32) {
    loop {
        if is_key_down(KeyCode::Enter) { break }
        set_default_camera();
        
        let x = screen_width() / 2.0;
        let y = screen_height() / 2.0;
        draw_text_centered("You flew a distance of:", x, y - 100.0, 50.0, WHITE);
        draw_text_centered(format!("{}", score.round()).as_str(), x, y, 100.0, WHITE);
        draw_text_centered("Press ENTER to play again.", x, y + 100.0, 50.0, WHITE);
        next_frame().await
    }
}

fn setup_background() {
    // TODO Avoid recalculating cosmetic things like background
    rand::srand(0);
    clear_background(GRAY);
    for i in (0..3000).map(|x| rand::gen_range(0.0, 1000.0) *  x as f32) {
        draw_mountain(i, FLOOR_HEIGHT + rand::gen_range(0.0, 200.0), rand::gen_range(0.0, 100.0));
    }
    // Draw floor
    draw_line(-1e10, FLOOR_HEIGHT, 1e10, FLOOR_HEIGHT, 5.0, GREEN);
}


#[macroquad::main("learn2glide")]
async fn main() {

    let mut myplane = Aircraft { ..Default::default() };
    let mut fpss = Vec::new();

    let mut meteors = Vec::new();
    for i in 1..30 {
        meteors.push(Meteor { pos: Vec2::new((i * 1000) as f32, FLOOR_HEIGHT + 1000.), 
                            vel: - Vec2::X - Vec2::Y});
    }

    loop {
        setup_background();
        for meteor in &mut meteors {
            if meteor.pos.y > FLOOR_HEIGHT {
                meteor.draw();
                meteor.update_pos();
                if (myplane.pos - meteor.pos).length() < 10.0 {
                    death_screen(myplane.pos.x).await;
                    myplane = Aircraft { ..Default::default() };
                }

            }
        }
        set_default_camera();
        draw_text(format!("rot:   {}", myplane.rot).as_str(), 20.0, 15.0, 20.0, DARKGRAY);
        draw_text(format!("pos:   {}", myplane.pos.round()).as_str(), 20.0, 30.0, 20.0, DARKGRAY);
        draw_text(format!("vel:   {}", myplane.vel).as_str(), 20.0, 45.0, 20.0, DARKGRAY);
        draw_text(format!("accel: {}", myplane.accel).as_str(), 20.0, 60.0, 20.0, DARKGRAY);
        draw_line(0.0, 75.0, myplane.fuel * 300.0 / 100.0, 75.0, 5.0, RED);

        fpss.push(get_fps() as i16);
        let fps = utils::avg_last_n(&fpss, 10);
        draw_text(format!("fps: {fps}").as_str(), screen_width() - 100.0, 15.0, 20.0, DARKGRAY);

        let cam = Camera2D {
            zoom: 0.002 * Vec2::ONE,
            target: myplane.pos,
            ..Default::default()
        };
        
        
        set_camera(&cam);

        if is_key_down(KeyCode::Left) {
            myplane.rotate(0.05);
        }

        if is_key_down(KeyCode::Right) {
            myplane.rotate(-0.05);
        }
        let mut boost = Vec2::ZERO;
        if is_key_down(KeyCode::Space) && myplane.fuel > 0.0 {
            boost = utils::vec2_from_polar(0.1, myplane.rot);
            myplane.fuel -= 0.2;
            myplane.draw_boost();
        }
        
        // Forces
        let lift = myplane.lift();
        let gravity = -0.03 * Vec2::Y;
        let drag = -1e-4 * myplane.vel.powf(2.0);

        myplane.accel = lift + gravity + boost + drag;

        myplane.draw();
        myplane.update_pos();

        if myplane.pos.y < FLOOR_HEIGHT {
            death_screen(myplane.pos.x).await;
            myplane = Aircraft { ..Default::default() };
        }
        next_frame().await
    }
}