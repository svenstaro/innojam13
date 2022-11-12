use std::collections::HashMap;

use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

static WORLD_SIZE: UVec2 = UVec2::new(50, 30);

#[derive(Debug, Clone)]
struct Atom {
    velocity: Vec2,
    pressure: u32,
}

#[derive(Debug, Clone)]
struct Cell {
    water: Atom,
    lava: Atom,
    sand: Atom,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            water: Atom {
                velocity: vec2(0.0, 1.0),
                pressure: 0,
            },
            lava: Atom {
                velocity: vec2(0.0, 0.0),
                pressure: 0,
            },
            sand: Atom {
                velocity: vec2(0.0, 0.0),
                pressure: 0,
            },
        }
    }
}

fn get_index(x: u32, y: u32) -> usize {
    (y * WORLD_SIZE.x + x) as usize
}

fn calculate_new_position(x: u32, y: u32, world_display: &Vec<Cell>) -> usize {
    let current_cell = &world_display[get_index(x, y)];

    let current_pos = vec2(x as f32, y as f32);
    let new_cell_pos = current_pos + current_cell.water.velocity;
    let new_cell_pos = new_cell_pos.clamp(vec2(0.0, 0.0), WORLD_SIZE.as_vec2() - Vec2::ONE);
    let new_index = get_index(new_cell_pos.x as u32, new_cell_pos.y as u32);

    // Check if target cell is occupied.
    if world_display[new_index].water.pressure > 0 {
        get_index(x, y)
    } else {
        new_index
    }
}

#[macroquad::main("Innojam13")]
async fn main() {
    let mut world_display = vec![Cell::default(); (WORLD_SIZE.x * WORLD_SIZE.y) as usize];
    let mut world_pressure = vec![Cell::default(); (WORLD_SIZE.x * WORLD_SIZE.y) as usize];

    let mut image = Image::gen_image_color(WORLD_SIZE.x as u16, WORLD_SIZE.y as u16, WHITE);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let zoom_factor = 16.0;

    world_display[get_index(10, 10)].water.pressure = 2;

    loop {
        clear_background(LIGHTGRAY);

        let (mx, my) = mouse_position();
        if is_mouse_button_down(MouseButton::Left) {
            let world_pos = vec2(mx / zoom_factor, my / zoom_factor);
            let cell = &mut world_display[get_index(world_pos.x as u32, world_pos.y as u32)];
            cell.water.pressure += 5;
        }

        let mut total_pressure = 0;
        for y in 0..WORLD_SIZE.y as u32 {
            for x in 0..WORLD_SIZE.x as u32 {
                let current_cell = &world_display[get_index(x, y)];
                total_pressure += current_cell.water.pressure;
            }
        }

        // Move from world_display to world_pressure.
        for y in 0..WORLD_SIZE.y as u32 {
            for x in 0..WORLD_SIZE.x as u32 {
                let current_cell = &mut world_display[get_index(x, y)];
                let new_cell = &mut world_pressure[get_index(x, y)];

                new_cell.water.pressure = current_cell.water.pressure;
                current_cell.water.pressure = 0;
            }
        }

        // let mut after_gravity_total_pressure = 0;
        // for y in 0..WORLD_SIZE.y as u32 {
        //     for x in 0..WORLD_SIZE.x as u32 {
        //         let current_cell = &world_pressure[get_index(x, y)];
        //         after_gravity_total_pressure += current_cell.water.pressure;
        //     }
        // }
        //
        // let mut world_display_pressure = 0;
        // for y in 0..WORLD_SIZE.y as u32 {
        //     for x in 0..WORLD_SIZE.x as u32 {
        //         let current_cell = &world_display[get_index(x, y)];
        //         world_display_pressure += current_cell.water.pressure;
        //     }
        // }
        // assert_eq!(total_pressure, after_gravity_total_pressure, "fucky gravy");
        // assert_eq!(world_display_pressure, 0);

        // Resolve pressure.
        for y in 0..WORLD_SIZE.y as u32 {
            for x in 0..WORLD_SIZE.x as u32 {
                let mut current_cell = world_pressure[get_index(x, y)].clone();

                if current_cell.water.pressure == 0 {
                    continue;
                }

                let gravity = ivec2(0, 1);

                let mut directions = vec![ivec2(0, -1), ivec2(-1, 0), ivec2(1, 0), ivec2(0, 1)];
                directions.shuffle();
                for direction in directions {
                    // world_pressure[get_index(x, y)]
                    // TODO Calculate proper pressure differences
                    // Check if we have enough pressure to give.
                    // if current_cell.water.pressure > 0 {
                    //     let pos = ivec2(x as i32, y as i32) + direction + gravity;
                    //     let pos = pos
                    //         .clamp(ivec2(0, 0), WORLD_SIZE.as_ivec2() - IVec2::ONE)
                    //         .as_uvec2();
                    //     world_display[get_index(pos.x, pos.y)].water.pressure += 1;
                    //     current_cell.water.pressure -= 1;
                    // } else {
                    //     break;
                    // }
                }
                world_display[get_index(x, y)].water.pressure += current_cell.water.pressure;
                current_cell.water.pressure = 0;
            }
        }

        // let mut after_resolve_total_pressure = 0;
        // for y in 0..WORLD_SIZE.y as u32 {
        //     for x in 0..WORLD_SIZE.x as u32 {
        //         let current_cell = &world_display[get_index(x, y)];
        //         after_resolve_total_pressure += current_cell.water.pressure;
        //     }
        // }
        // assert_eq!(total_pressure, after_resolve_total_pressure, "fucky wucky");

        for i in 0..world_pressure.len() {
            // TODO Display depending on prevalent atom pressure.
            let current_cell = &world_display[i as usize];
            let cell_color = if current_cell.water.pressure > 0 {
                let water_color = current_cell.water.pressure as f32 / 10.0;
                Color::new(0.2, 0.2, 0.9, water_color)
            } else {
                WHITE
            };

            // Also set the texture.
            image.set_pixel(i as u32 % WORLD_SIZE.x, i as u32 / WORLD_SIZE.x, cell_color);
        }
        world_pressure.fill(Cell::default());

        texture.update(&image);

        let texture_params = DrawTextureParams {
            dest_size: Some(WORLD_SIZE.as_vec2() * zoom_factor),
            ..Default::default()
        };
        draw_texture_ex(texture, 0., 0., WHITE, texture_params);

        next_frame().await
    }
}
