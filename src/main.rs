use macroquad::prelude::*;
use std::f32::consts::PI;

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
                    x2, screen_height() - y2, 3.0, YELLOW);
    }

    fn rotate(&mut self, amount: f32) {
        self.rot = (self.rot + amount).clamp(-PI / 2.0, PI / 2.0)
    }

    fn update_pos(&mut self) {
        self.pos += self.vel;
        
    }

    fn do_lift(&mut self) {
        let down_dir = self.rot - PI / 2.0;
        let vel_dir = dir(self.vel);
        

        let lift_dir = self.rot + PI / 2.0;
        let mag_in_down_dir = self.vel.length() * (vel_dir - down_dir).abs().cos();
        let lift_accel = vec2_from_polar(mag_in_down_dir * 0.1, lift_dir);

        draw_text(("lift_x_accel: ".to_owned() + lift_accel[0].to_string().as_str()).as_str(), 20.0, 30.0, 20.0, DARKGRAY);
        draw_text(("lift_y_accel: ".to_owned() + lift_accel[1].to_string().as_str()).as_str(), 20.0, 45.0, 20.0, DARKGRAY);
        self.vel += lift_accel;
    }
        
}

fn dir(v: Vec2) -> f32 {
    (v[1] / v[0]).atan()
}

fn vec2_from_polar(r: f32, theta: f32) -> Vec2{
    Vec2::new(r * theta.cos(), r * theta.sin())
}

#[macroquad::main("L2F")]
async fn main() {

    let mut myplane = Aircraft { pos: Vec2::new(screen_width() / 2.0, screen_height() / 1.1),
                                rot: 0.0,
                                vel: Vec2::ZERO};
    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
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


        myplane.draw();
        // Gravity
        myplane.vel[1] -= 0.01;
        // Air resistance
        // myplane.y_vel *= 0.98;
        // myplane.x_vel *= 0.98;
        myplane.do_lift();
        
        myplane.update_pos();
        draw_text(("x_vel: ".to_owned() + myplane.vel[0].to_string().as_str()).as_str(), 20.0, 60.0, 20.0, DARKGRAY);
        draw_text(("y_vel: ".to_owned() + myplane.vel[1].to_string().as_str()).as_str(), 20.0, 75.0, 20.0, DARKGRAY);
        draw_text(myplane.rot.to_string().as_str(), 20.0, 15.0, 20.0, DARKGRAY);
        next_frame().await
    }
}