use macroquad::prelude::*;
use std::f32::consts::PI;

const FLOOR_HEIGHT: f32 = 1000.0;

struct Aircraft {
    pos: Vec2,
    vel: Vec2,
    accel: Vec2,
    rot: f32, // pi/2 to - pi/2
    fuel: f32
}


impl Default for Aircraft {
    fn default() -> Aircraft {
        Aircraft { 
            pos: Vec2::new(screen_width() / 2.0, screen_height() / 1.1),
            vel: -Vec2::Y,
            accel: Vec2::ZERO,
            rot: 0.0,
            fuel: 100.0,
        }
    }
}


impl Aircraft {
    
    fn draw(&self) {
        let glider_size = 10.0;
        let offset = vec2_from_polar(glider_size, self.rot);

        let lower = self.pos - offset;
        let upper = self.pos + offset;

        draw_line(lower.x, screen_height() - lower.y, upper.x, screen_height() - upper.y, 3.0, GREEN);
    }

    fn draw_boost(&self) {
        let glider_size = 10.0;
        let offset = vec2_from_polar(glider_size, self.rot);

        let lower = self.pos - 2.0*offset;
        let upper = self.pos - 1.5*offset;

        draw_line(lower.x, screen_height() - lower.y, 
                    upper.x, screen_height() - upper.y, 3.0, RED);
    }

    fn rotate(&mut self, amount: f32) {
        self.rot = (self.rot + amount).clamp(-PI / 2.0, PI / 2.0)
    }

    fn update_pos(&mut self) {
        self.vel += self.accel;
        self.pos += self.vel;
        
    }

    fn lift(&mut self) -> Vec2{
        let down = vec2_from_polar(1.0, self.rot - PI / 2.0);

        let lift_dir = self.rot + PI / 2.0;
        let eps = Vec2::new(f32::EPSILON, f32::EPSILON);
        let mag_in_down_dir = self.vel.length() * (self.vel + eps).angle_between(down).cos();
        let lift_accel = vec2_from_polar(mag_in_down_dir * 0.1, lift_dir);

        return lift_accel;
    }
        
}


fn vec2_from_polar(r: f32, theta: f32) -> Vec2{
    Vec2::new(r * theta.cos(), r * theta.sin())
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


fn get_avg_fps(fpss: &Vec<i32>) -> f32{
    let l = fpss.len().saturating_sub(10);
    let fps_window = &fpss[l..];
    fps_window.iter().sum::<i32>() as f32 / fps_window.len() as f32
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
        draw_mountain(i, FLOOR_HEIGHT - rand::gen_range(0.0, 200.0), rand::gen_range(0.0, 100.0));
    }
    // Draw floor
    draw_line(-1e10, FLOOR_HEIGHT, 1e10, FLOOR_HEIGHT, 5.0, GREEN);
}


#[macroquad::main("learn2glide")]
async fn main() {

    let mut myplane = Aircraft { ..Default::default() };
    let mut fpss = Vec::new();
    loop {
        setup_background();
        set_default_camera();
        draw_text(format!("rot:   {}", myplane.rot).as_str(), 20.0, 15.0, 20.0, DARKGRAY);
        draw_text(format!("pos:   {}", myplane.pos.round()).as_str(), 20.0, 30.0, 20.0, DARKGRAY);
        draw_text(format!("vel:   {}", myplane.vel).as_str(), 20.0, 45.0, 20.0, DARKGRAY);
        draw_text(format!("accel: {}", myplane.accel).as_str(), 20.0, 60.0, 20.0, DARKGRAY);
        draw_line(0.0, 75.0, myplane.fuel * 300.0 / 100.0, 75.0, 5.0, RED);

        fpss.push(get_fps());
        let fps = get_avg_fps(&fpss);
        draw_text(format!("fps: {fps}").as_str(), screen_width() - 100.0, 15.0, 20.0, DARKGRAY);

        let cam = Camera2D {
            zoom: 0.002 * Vec2::new(1.0, -1.0),
            target: Vec2::new(myplane.pos.x, screen_height() - myplane.pos.y),
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
            boost = vec2_from_polar(0.1, myplane.rot);
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

        if screen_height() - myplane.pos.y > FLOOR_HEIGHT {
            death_screen(myplane.pos.x).await;

            myplane = Aircraft { ..Default::default() };

        }
        next_frame().await
    }
}