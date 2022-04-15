use macroquad::{
    math::Vec2, 
    color::colors::*, 
    shapes::*,
};

use std::f32::consts::PI;
use crate::utils::vec2_from_polar;

pub struct Aircraft {
    pub pos: Vec2,
    pub vel: Vec2,
    pub accel: Vec2,
    pub rot: f32, // pi/2 to - pi/2
    pub fuel: f32
}


impl Default for Aircraft {
    fn default() -> Aircraft {
        Aircraft { 
            pos: Vec2::new(0., 500.),
            vel: -Vec2::Y,
            accel: Vec2::ZERO,
            rot: 0.0,
            fuel: 100.0,
        }
    }
}


impl Aircraft {
    pub fn draw(&self) {
        let glider_size = 10.0;
        let offset = vec2_from_polar(glider_size, self.rot);

        let lower = self.pos - offset;
        let upper = self.pos + offset;

        draw_line(lower.x, lower.y, upper.x, upper.y, 3.0, GREEN);
    }

    pub fn draw_boost(&self) {
        let glider_size = 10.0;
        let offset = vec2_from_polar(glider_size, self.rot);

        let lower = self.pos - 2.0*offset;
        let upper = self.pos - 1.5*offset;

        draw_line(lower.x, lower.y, 
                    upper.x, upper.y, 3.0, RED);
    }

    pub fn rotate(&mut self, amount: f32) {
        self.rot = (self.rot + amount).clamp(-PI / 2.0, PI / 2.0)
    }

    pub fn update_pos(&mut self) {
        self.vel += self.accel;
        self.pos += self.vel;
        
    }

    pub fn lift(&mut self) -> Vec2{
        let down = vec2_from_polar(1.0, self.rot - PI / 2.0);

        let lift_dir = self.rot + PI / 2.0;
        let eps = Vec2::new(f32::EPSILON, f32::EPSILON);
        let mag_in_down_dir = self.vel.length() * (self.vel + eps).angle_between(down).cos();
        let lift_accel = vec2_from_polar(mag_in_down_dir * 0.1, lift_dir);

        return lift_accel;
    }
        
}