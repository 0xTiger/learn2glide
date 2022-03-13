use std::iter::Sum;
use macroquad::math::Vec2;


pub fn vec2_from_polar(r: f32, theta: f32) -> Vec2 {
    Vec2::new(r * theta.cos(), r * theta.sin())
}


pub fn avg_last_n<'a, T: Into<f32> + Sum<&'a T>>(v: &'a Vec<T>, n: usize) -> f32 {
    let l = v.len().saturating_sub(n);
    let last_n = &v[l..];
    last_n.iter().sum::<T>().into() / last_n.len() as f32
}