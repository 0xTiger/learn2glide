use super::utils;
use macroquad::{
    math::Vec2, 
    color::colors::*, 
    input::*,
    texture::*
};
use std::{f32::consts::PI, collections::HashMap};
use crate::utils::vec2_from_polar;

pub struct Aircraft {
    pub pos: Vec2,
    pub vel: Vec2,
    pub accel: Vec2,
    pub rot: f32, // pi/2 to - pi/2
    pub fuel: f32,
    pub score: u32
}


impl Default for Aircraft {
    fn default() -> Aircraft {
        Aircraft { 
            pos: Vec2::new(0., 500.),
            vel: -Vec2::Y,
            accel: Vec2::ZERO,
            rot: 0.0,
            fuel: 100.0,
            score: 0
        }
    }
}


impl Aircraft {
    pub fn draw(&self, textures: &HashMap<&str, Texture2D>) {
        let glider_size = Vec2::new(80.0, -80.0);
        let texture = textures.get("aircraft").unwrap();
        let params = DrawTextureParams { 
            dest_size: Some(glider_size),
            rotation: self.rot,
            ..Default::default()
        };

        draw_texture_ex(&texture, self.pos.x - glider_size.x, self.pos.y - glider_size.y, WHITE, params);
    }

    pub fn draw_boost(&self, textures: &HashMap<&str, Texture2D>) {
        let glider_size = Vec2::new(80.0, -80.0);
        let texture = textures.get("boost").unwrap();
        texture.set_filter(FilterMode::Nearest);
        let params = DrawTextureParams { 
            dest_size: Some(glider_size),
            rotation: self.rot,
            ..Default::default()
        };

        draw_texture_ex(&texture, self.pos.x - glider_size.x, self.pos.y - glider_size.y, WHITE, params);
    }

    pub fn rotate(&mut self, amount: f32) {
        self.rot = (self.rot + amount).clamp(-PI / 2.0, PI / 2.0)
    }

    pub fn update_pos(&mut self) {
        self.vel += self.accel;
        self.pos += self.vel;
    }

    pub fn lift(&mut self) -> Vec2 {
        let down = vec2_from_polar(1.0, self.rot - PI / 2.0);
        let lift_dir = self.rot + PI / 2.0;
        let eps = Vec2::new(f32::EPSILON, f32::EPSILON);
        let mag_in_down_dir = self.vel.length() * (self.vel + eps).angle_between(down).cos();
        
        vec2_from_polar(mag_in_down_dir * 0.1, lift_dir)
    }
    
    pub fn boost(&self) -> Vec2 {
        if is_key_down(KeyCode::Space) && self.fuel > 0.0 { 
            utils::vec2_from_polar(0.1, self.rot) 
        } else {
            Vec2::ZERO
        }
    }

    pub fn check_input(&mut self, textures: &HashMap<&str, Texture2D>) {
        if is_key_down(KeyCode::Left) {
            self.rotate(0.05);
        }
        if is_key_down(KeyCode::Right) {
            self.rotate(-0.05);
        }
        if is_key_down(KeyCode::Space) && self.fuel > 0.0 {
            self.fuel -= 0.2;
            self.draw_boost(textures);
        }
    }    
}