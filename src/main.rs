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
        let inv_rot = -self.rot;
        let x_offset = inv_rot.cos() * 10.0;
        let y_offset = inv_rot.sin() * 10.0;
        draw_line(self.x - x_offset, self.y - y_offset, 
                    self.x + x_offset, self.y + y_offset, 3.0, YELLOW);
    }

    fn rotate(&mut self, amount: f32) {
        self.rot = (self.rot + amount).clamp(-PI / 2.0, PI / 2.0)
    }

    fn update_pos(&mut self) {
        self.x += self.x_vel;
        self.y += self.y_vel;
    }

    fn do_lift(&mut self) {
        let lift_angle = self.rot + PI / 2.0;
        let lift_x_vel = lift_angle.cos() * self.x_vel * 0.5;
        let lift_y_vel = lift_angle.sin() * self.y_vel * 0.5;
        draw_text(lift_x_vel.to_string().as_str(), 20.0, 40.0, 30.0, DARKGRAY);
        draw_text(lift_y_vel.to_string().as_str(), 20.0, 60.0, 30.0, DARKGRAY);
        self.x_vel += lift_x_vel;
        self.y_vel += lift_y_vel;
    }
        
}

#[macroquad::main("L2F")]
async fn main() {

    let mut myplane = Aircraft { x: screen_width() / 2.0, 
                                y: screen_height() / 2.0,
                                rot: 0.0,
                                x_vel: 0.0,
                                y_vel: 0.0};
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

        myplane.draw();
        myplane.update_pos();
        myplane.y_vel += 0.01;
        myplane.do_lift();

        draw_text(myplane.rot.to_string().as_str(), 20.0, 20.0, 30.0, DARKGRAY);
        next_frame().await
    }
}