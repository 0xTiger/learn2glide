mod utils;
mod aircraft;
mod hoops;

use std::collections::HashMap;
use macroquad::prelude::*;
use aircraft::Aircraft;
use hoops::{Hoop, HoopKind};


const FLOOR_HEIGHT: f32 = 0.0;
const GRAVITY: Vec2 = const_vec2!([0., -0.03]);


fn draw_mountain(x: f32, y : f32, w: f32) {
    draw_line(x, y, x + w, FLOOR_HEIGHT, 3.0, GREEN);
    draw_line(x, y, x - w, FLOOR_HEIGHT, 3.0, GREEN);
}


fn draw_text_centered(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let text_dims = measure_text(text, None, font_size as u16, 1.0);
    draw_text(text, x - text_dims.width / 2.0, 
                    y + text_dims.height / 2.0, 
                    font_size, color)
}


async fn death_screen(score: f32) {
    loop {
        if is_key_down(KeyCode::Enter) { break }
        set_default_camera();
        
        let x = screen_width() / 2.0;
        let y = screen_height() / 2.0;
        draw_text_centered("You flew a distance of:", x, y - 100.0, 50.0, WHITE);
        draw_text_centered(format!("{}", score.round()).as_str(), x, y, 100.0, WHITE);
        draw_text_centered("Press ENTER to play again.", x, y + 100.0, 50.0, WHITE);
        next_frame().await
    }
}


fn draw_hud(plane: &Aircraft, fps: f32) {
    draw_text(format!("rot:   {}", plane.rot).as_str(), 20.0, 15.0, 20.0, DARKGRAY);
    draw_text(format!("pos:   {}", plane.pos.round()).as_str(), 20.0, 30.0, 20.0, DARKGRAY);
    draw_text(format!("vel:   {}", plane.vel).as_str(), 20.0, 45.0, 20.0, DARKGRAY);
    draw_text(format!("accel: {}", plane.accel).as_str(), 20.0, 60.0, 20.0, DARKGRAY);
    draw_line(0.0, 75.0, plane.fuel * 300.0 / 100.0, 75.0, 5.0, RED);
    draw_text(format!("fps: {fps}").as_str(), screen_width() - 100.0, 15.0, 20.0, DARKGRAY);
}


fn setup_background() {
    // TODO Avoid recalculating cosmetic things like background
    rand::srand(0);
    clear_background(GRAY);
    for i in (0..3000).map(|x| rand::gen_range(0.0, 1000.0) *  x as f32) {
        draw_mountain(i, FLOOR_HEIGHT + rand::gen_range(0.0, 200.0), rand::gen_range(0.0, 100.0));
    }
    // Draw floor
    draw_rectangle(-1e10, FLOOR_HEIGHT, 2e10, -500., GREEN);
}


#[macroquad::main("learn2glide")]
async fn main() {
    let texture_names = vec![
        "aircraft", 
        "boost", 
        "hoop_fuel", 
        "hoop_score", 
        "hoop_boost", 
        "cloud"
    ];
    let mut textures = HashMap::new();
    for texture_name in texture_names {
        let texture = load_texture(format!("assets/{}.png", texture_name).as_str()).await.unwrap();
        texture.set_filter(FilterMode::Nearest);
        textures.insert(texture_name, texture);
    }

    let mut myplane = Aircraft::default();
    let mut fpss = Vec::new();
    let mut hoops = Vec::new();

    for i in 1..30 {
        hoops.push(Hoop { 
            pos: Vec2::new((i * 500) as f32, FLOOR_HEIGHT + 200.), 
            vel: Vec2::ZERO, //Vec2::new(3., 6.),
            accel: Vec2::ZERO,
            size: 100.,
            value: 20.,
            kind: HoopKind::random()
        });
    }

    loop {
        setup_background();
        let regionsize = 1000.;
        let current_region = (myplane.pos.x as i32 / regionsize as i32, myplane.pos.y as i32 / regionsize as i32);
        let rendered_regions = vec![
            (current_region.0 - 1, current_region.1 - 1),
            (current_region.0, current_region.1 - 1),
            (current_region.0 + 1, current_region.1 - 1),
            (current_region.0 - 1, current_region.1),
            (current_region.0, current_region.1),
            (current_region.0 + 1, current_region.1),
            (current_region.0 - 1, current_region.1 + 1),
            (current_region.0, current_region.1 + 1),
            (current_region.0 + 1, current_region.1 + 1),
        ];
        // TODO add collision detection & drawing only within region
        
        let texture = *(&textures).get("cloud").unwrap();
        for region in rendered_regions {
            let seed = (region.0 + region.1) * (region.0 + region.1 + 1) / 2 + region.0;
            rand::srand(seed as u64);
            for _ in 0..10 {
                let x = rand::gen_range(0., regionsize);
                let y = rand::gen_range(0., regionsize);
                
                let params = DrawTextureParams { 
                    dest_size: Some(60.*Vec2::ONE),
                    ..Default::default()
                };
                
                if 1000. * region.1 as f32 + y > FLOOR_HEIGHT {
                    draw_texture_ex(texture, 1000. * region.0 as f32 + x , 1000. * region.1 as f32 + y , WHITE, params);
                }
            }
        }
        for hoop in &mut hoops {
            hoop.draw(&textures);
            hoop.update_pos();
            if (myplane.pos - hoop.pos).length() < hoop.size {
                hoop.do_effect(&mut myplane);
                hoop.kind = HoopKind::Dead;
            } else if hoop.pos.y - hoop.size < FLOOR_HEIGHT {
                hoop.kind = HoopKind::Dead;
            }
        }
        hoops.retain(|hoop| hoop.kind != HoopKind::Dead);
        set_default_camera();
        fpss.push(get_fps() as i16);
        let fps = utils::avg_last_n(&fpss, 10);
        draw_hud(&myplane, fps);

        let cam = Camera2D {
            zoom: 0.002 * Vec2::ONE,
            target: myplane.pos,
            ..Default::default()
        };
        
        
        set_camera(&cam);
        myplane.check_input(&textures);
        
        // Forces
        let drag = -1e-4 * myplane.vel.powf(2.0);
        myplane.accel = myplane.lift() + GRAVITY + myplane.boost() + drag;
        myplane.draw(&textures);
        myplane.update_pos();

        if myplane.pos.y < FLOOR_HEIGHT {
            death_screen(myplane.pos.x).await;
            myplane = Aircraft::default();
        }
        next_frame().await
    }
}