use super::aircraft::Aircraft;
use std::fs::read;
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
    pub fn draw(&self) {
        let file = match self.kind {
            HoopKind::Fuel => read("assets/hoop_fuel.png").unwrap(),
            HoopKind::Score => read("assets/hoop_score.png").unwrap(),
            HoopKind::Boost => read("assets/hoop_boost.png").unwrap(),
            _ => read("hoop_fuel.png").unwrap()
        };
        let texture = Texture2D::from_file_with_format(&file, None);
        texture.set_filter(FilterMode::Nearest);
        let shape = self.size*Vec2::ONE;
        let params = DrawTextureParams { 
            dest_size: Some(shape),
            ..Default::default()
        };
        draw_texture_ex(texture, self.pos.x - shape.x, self.pos.y - shape.y, WHITE, params);
    }

    pub fn update_pos(&mut self) {
        self.vel += self.accel;
        self.pos += self.vel;
    }

    pub fn do_effect(&mut self, plane: &mut Aircraft) {
        match self.kind {
            HoopKind::Fuel => {plane.fuel += self.value}
            HoopKind::Boost => {plane.vel *= self.value}
            _ => {}
        }
    }
}