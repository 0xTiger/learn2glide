use macroquad::prelude::*;
use std::f32::consts::PI;


const FLOOR_HEIGHT: f32 = 1000.0;

struct Aircraft {
    pos: Vec2,
    vel: Vec2,
    rot: f32 // pi/2 to - pi/2
}


impl Aircraft {
    
    fn draw(&self) {
        let glider_size = 10.0;
        let offset = vec2_from_polar(glider_size, self.rot);

        let x1 = self.pos[0] - offset[0];
        let y1 = self.pos[1] - offset[1];
        let x2 = self.pos[0] + offset[0];
        let y2 = self.pos[1] + offset[1];

        draw_line(x1, screen_height() - y1, 
                    x2, screen_height() - y2, 3.0, GREEN);
    }

    fn rotate(&mut self, amount: f32) {
        self.rot = (self.rot + amount).clamp(-PI / 2.0, PI / 2.0)
    }

    fn update_pos(&mut self) {
        self.pos += self.vel;
        
    }

    fn do_lift(&mut self) {
        let down = vec2_from_polar(1.0, self.rot - PI / 2.0);

        let lift_dir = self.rot + PI / 2.0;
        let mag_in_down_dir = self.vel.length() * self.vel.angle_between(down).cos();
        let lift_accel = vec2_from_polar(mag_in_down_dir * 0.1, lift_dir);

        draw_text(("lift_x_accel: ".to_owned() + lift_accel[0].to_string().as_str()).as_str(), 20.0, 30.0, 20.0, DARKGRAY);
        draw_text(("lift_y_accel: ".to_owned() + lift_accel[1].to_string().as_str()).as_str(), 20.0, 45.0, 20.0, DARKGRAY);
        self.vel += lift_accel;
    }
        
}

fn dir(v: Vec2) -> f32 {
    v.angle_between(Vec2::X)
}

fn vec2_from_polar(r: f32, theta: f32) -> Vec2{
    Vec2::new(r * theta.cos(), r * theta.sin())
}

fn draw_mountain(x: f32, y : f32) {
    draw_line(x, y, x + 10.0, FLOOR_HEIGHT, 3.0, GREEN);
    draw_line(x, y, x - 10.0, FLOOR_HEIGHT, 3.0, GREEN);
}
fn setup_background() {
    clear_background(BLACK);
    for i in (0..30).map(|x| 2i32.pow(x)) {
        draw_mountain(i as f32, FLOOR_HEIGHT - 100.0);
    }
    // Draw floor
    draw_line(0.0, FLOOR_HEIGHT, 1e10, FLOOR_HEIGHT, 5.0, ORANGE);
}
#[macroquad::main("L2F")]
async fn main() {

    let mut myplane = Aircraft { pos: Vec2::new(screen_width() / 2.0, screen_height() / 1.1),
                                rot: 0.0,
                                vel: Vec2::ZERO};
    loop {

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        setup_background();
        if is_key_down(KeyCode::Left) {
            myplane.rotate(0.05);
        }

        if is_key_down(KeyCode::Right) {
            myplane.rotate(-0.05);
        }
        
        if is_key_down(KeyCode::Space) {
            myplane.vel += vec2_from_polar(0.1, myplane.rot)
        }

        if is_key_down(KeyCode::Enter) {
            myplane.pos = Vec2::new(screen_width() / 2.0, screen_height() / 1.1);
            myplane.vel = Vec2::ZERO;
        }

        set_default_camera();

        // Gravity
        myplane.vel[1] -= 0.01;
        // Air resistance
        // myplane.y_vel *= 0.98;
        // myplane.x_vel *= 0.98;
        myplane.do_lift();
        
        let cam = Camera2D {
            zoom: 0.002 * Vec2::new(1.0, -1.0),
            target: Vec2::new(myplane.pos[0], screen_height() - myplane.pos[1]),
            ..Default::default()
        };
        // draw_text(("x_vel: ".to_owned() + myplane.vel[0].to_string().as_str()).as_str(), 20.0, 60.0, 20.0, DARKGRAY);
        // draw_text(("y_vel: ".to_owned() + myplane.vel[1].to_string().as_str()).as_str(), 20.0, 75.0, 20.0, DARKGRAY);
        draw_text(format!("pos: {:#.2}", myplane.pos.round()).as_str(), 20.0, 60.0, 20.0, DARKGRAY);
        draw_text(format!("vel: {:#.2}", myplane.vel).as_str(), 20.0, 75.0, 20.0, DARKGRAY);
        draw_text(myplane.rot.to_string().as_str(), 20.0, 15.0, 20.0, DARKGRAY);
        
        set_camera(&cam);
        
        
        myplane.update_pos();
        myplane.draw();
        
        
        next_frame().await
    }
}