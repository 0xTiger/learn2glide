use super::aircraft::Aircraft;
use macroquad::{
    math::Vec2,
    color::Color,
    color::colors::*, 
    shapes::*,
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
    pub fn draw(&self) {
        let hoop_color = match self.kind {
            HoopKind::Fuel => RED,
            HoopKind::Score => GOLD,
            HoopKind::Boost => BLUE,
            _ => Color::from_rgba(0, 0, 0, 0)
        };
        draw_circle(self.pos.x, self.pos.y, self.size, hoop_color);
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