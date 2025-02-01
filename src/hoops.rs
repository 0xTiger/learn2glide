use super::aircraft::Aircraft;
use std::collections::HashMap;
use macroquad::{
    math::Vec2,
    color::colors::*, 
    texture::*,
    rand,
};


#[derive(PartialEq)]
pub enum HoopKind {
    Fuel,
    Score,
    Boost,
    Dead
}

pub struct Hoop {
    pub kind: HoopKind,
    pub pos: Vec2,
    pub vel: Vec2,
    pub accel: Vec2,
    pub size: f32,
    pub value: f32,
}


impl HoopKind {
    pub fn random() -> HoopKind {
        let i = rand::gen_range(0, 3);
        match i {
            0 => HoopKind::Fuel,
            1 => HoopKind::Score,
            2 => HoopKind::Boost,
            _ => HoopKind::Dead
        }
    }
}


impl Hoop {
    // TODO Only read files once
    pub fn draw(&self, textures: &HashMap<&str, Texture2D>) {
        let texture = match self.kind {
            HoopKind::Fuel => textures.get("hoop_fuel"),
            HoopKind::Score => textures.get("hoop_score"),
            HoopKind::Boost => textures.get("hoop_boost"),
            _ => textures.get("hoop_fuel")
        }.unwrap();
        texture.set_filter(FilterMode::Nearest);
        let shape = self.size*Vec2::ONE;
        let params = DrawTextureParams { 
            dest_size: Some(shape),
            ..Default::default()
        };
        draw_texture_ex(&texture, self.pos.x - shape.x, self.pos.y - shape.y, WHITE, params);
    }

    pub fn update_pos(&mut self) {
        self.vel += self.accel;
        self.pos += self.vel;
    }

    pub fn do_effect(&mut self, plane: &mut Aircraft) {
        match self.kind {
            HoopKind::Fuel => {plane.fuel += self.value}
            HoopKind::Boost => {plane.vel *= self.value}
            HoopKind::Score => {plane.score += self.value as u32}
            _ => {}
        }
    }
}