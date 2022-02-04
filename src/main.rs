use macroquad::prelude::*;
use std::f32::consts::PI;

struct Aircraft {
    x: f32,
    y: f32,
    x_vel: f32,
    y_vel: f32,
    rot: f32 // pi/2 to - pi/2
}


impl Aircraft {
    
    fn draw(&self) {
        // sin(rot) = y_offset / 1
        // cos(rot) = x_offset
        let x_offset = self.rot.cos() * 10.0;
        let y_offset = self.rot.sin() * 10.0;

        let x1 = self.x - x_offset;
        let y1 = self.y - y_offset;
        let x2 = self.x + x_offset;
        let y2 = self.y + y_offset;

        draw_line(x1, screen_height() - y1, 
                    x2, screen_height() - y2, 3.0, YELLOW);
    }

    fn rotate(&mut self, amount: f32) {
        self.rot = (self.rot + amount).clamp(-PI / 2.0, PI / 2.0)
    }

    fn update_pos(&mut self) {
        self.x += self.x_vel;
        self.y += self.y_vel;
        
    }

    fn do_lift(&mut self) {
        let lift_dir = self.rot + PI / 2.0;
        let down_dir = self.rot - PI / 2.0;
        let vel_dir = dir(self.y_vel, self.x_vel);

        let mag_in_down_dir = mag(self.x_vel, self.y_vel) * (vel_dir - down_dir).abs().cos();
        let lift_x_accel = lift_dir.cos() * mag_in_down_dir * 0.1;
        let lift_y_accel = lift_dir.sin() * mag_in_down_dir * 0.1;


        draw_text(("lift_x_accel: ".to_owned() + lift_x_accel.to_string().as_str()).as_str(), 20.0, 30.0, 20.0, DARKGRAY);
        draw_text(("lift_y_accel: ".to_owned() + lift_y_accel.to_string().as_str()).as_str(), 20.0, 45.0, 20.0, DARKGRAY);
        self.x_vel += lift_x_accel;
        self.y_vel += lift_y_accel;
    }
        
}

fn dir(x: f32, y: f32) -> f32 {
    (x / y).atan()
}

fn mag(x: f32, y: f32) -> f32 {
    (x.powi(2) + y.powi(2)).sqrt()
}
#[macroquad::main("L2F")]
async fn main() {

    let mut myplane = Aircraft { x: screen_width() / 2.0, 
                                y: screen_height() / 1.1,
                                rot: 0.0,
                                x_vel: 0.0,
                                y_vel: -0.0};
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

        if is_key_down(KeyCode::Enter) {
            myplane.x = screen_width() / 2.0;
            myplane.y = screen_height() / 2.0;
            myplane.x_vel = 0.0;
            myplane.y_vel = 0.0;
        }

        myplane.draw();
        // Gravity
        myplane.y_vel -= 0.01;
        // Air resistance
        // myplane.y_vel *= 0.98;
        // myplane.x_vel *= 0.98;
        myplane.do_lift();
        
        myplane.update_pos();
        draw_text(("x_vel: ".to_owned() + myplane.x_vel.to_string().as_str()).as_str(), 20.0, 60.0, 20.0, DARKGRAY);
        draw_text(("y_vel: ".to_owned() + myplane.y_vel.to_string().as_str()).as_str(), 20.0, 75.0, 20.0, DARKGRAY);
        draw_text(myplane.rot.to_string().as_str(), 20.0, 15.0, 20.0, DARKGRAY);
        next_frame().await
    }
}