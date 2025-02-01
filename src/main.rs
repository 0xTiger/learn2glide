mod utils;
mod aircraft;
mod hoops;

use std::collections::HashMap;
use macroquad::prelude::*;
use aircraft::Aircraft;
use hoops::{Hoop, HoopKind};


const FLOOR_HEIGHT: f32 = 0.0;
const GRAVITY: Vec2 = Vec2::from_array([0., -0.04]);
const SKY_BLUE: Color = color_u8!(12, 92, 146, 255);
const LAND_GREEN: Color = color_u8!(13, 128, 36, 255);

fn draw_text_centered(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let text_dims = measure_text(text, None, font_size as u16, 1.0);
    draw_text(text, x - text_dims.width / 2.0, 
                    y + text_dims.height / 2.0, 
                    font_size, color);
}


async fn death_screen(score: u32) {
    loop {
        if is_key_down(KeyCode::Enter) { break }
        set_default_camera();
        
        let x = screen_width() / 2.0;
        let y = screen_height() / 2.0;
        draw_text_centered("You flew a distance of:", x, y - 100.0, 50.0, WHITE);
        draw_text_centered(&score.to_string(), x, y, 100.0, WHITE);
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


fn setup_background(textures: &HashMap<&str, Texture2D>) {
    // TODO Use dithered gradient for sky
    clear_background(SKY_BLUE);
    let texture = textures.get("sky").unwrap();
    let params = DrawTextureParams { 
        dest_size: Some(Vec2::new(2e10, -1e4)),
        ..Default::default()
    };
    // Draw floor
    draw_rectangle(-1e10, FLOOR_HEIGHT, 2e10, -500., LAND_GREEN);
    draw_texture_ex(&texture, -1e10, FLOOR_HEIGHT + 1e4, WHITE, params);
}


#[macroquad::main("learn2glide")]
async fn main() {
    let texture_names = vec![
        "aircraft", 
        "boost", 
        "hoop_fuel", 
        "hoop_score", 
        "hoop_boost", 
        "cloud",
        "mountain_1",
        "mountain_2",
        "mountain_3",
        "sky",
        "sky_sunset"
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
        setup_background(&textures);
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
        
        let cloud_texture = (&textures).get("cloud").unwrap();
        let mountain_textures = vec![
            (&textures).get("mountain_1").unwrap(),
            (&textures).get("mountain_2").unwrap(),
            (&textures).get("mountain_3").unwrap()
        ];
        for region in rendered_regions {
            let seed = (region.0 + region.1) * (region.0 + region.1 + 1) / 2 + region.0;
            rand::srand(seed as u64);
            for _ in 0..10 {
                let x = regionsize * region.0 as f32 + rand::gen_range(0., regionsize);
                let y = regionsize * region.1 as f32 + rand::gen_range(0., regionsize);
                
                let params = DrawTextureParams { 
                    dest_size: Some(60.*Vec2::ONE),
                    ..Default::default()
                };
                
                if y > FLOOR_HEIGHT + 300. {
                    draw_texture_ex(&cloud_texture, x, y, WHITE, params);
                }
            }
            if region.1 == 0 {
                for _ in 0..2 {
                    let x = regionsize * region.0 as f32 + rand::gen_range(0., regionsize);
                    let texture = rand::ChooseRandom::choose(&mountain_textures).unwrap();
                    let mountain_height = texture.height() * 10.;
                    let mountain_width = texture.width() * 10.;
                    let params = DrawTextureParams { 
                        dest_size: Some(Vec2::new(mountain_width, -mountain_height)),
                        ..Default::default()
                    };
                    draw_texture_ex(&texture, x, FLOOR_HEIGHT + mountain_height, WHITE, params)
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
            zoom: Vec2 { x: 0.002, y: -0.002 },
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
            death_screen(myplane.pos.x as u32 + myplane.score).await;
            myplane = Aircraft::default();
        }
        next_frame().await
    }
}