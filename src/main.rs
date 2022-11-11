use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

static WORLD_SIZE: UVec2 = UVec2::new(30, 20);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Material {
    Air,
    Water,
    Lava,
    Sand,
}

impl Default for Material {
    fn default() -> Self {
        Material::Air
    }
}

#[derive(Debug, Default, Clone)]
struct Cell {
    velocity: Vec2,
    material: Material,
}

fn get_index(x: u32, y: u32) -> usize {
    (y * WORLD_SIZE.x + x) as usize
}

fn calculate_new_position(
    x: u32,
    y: u32,
    world_front: &Vec<Cell>,
    world_back: &Vec<Cell>,
) -> Option<usize> {
    let current_cell = &world_front[get_index(x, y)];
    match current_cell.material {
        Material::Air => None,
        Material::Water => {
            let current_pos = Vec2::new(x as f32, y as f32);

            let new_cell_pos = current_pos + current_cell.velocity;

            let new_index = get_index(new_cell_pos.x as u32, new_cell_pos.y as u32);

            // Check if pixel below is free.
            if new_cell_pos.y > (WORLD_SIZE.y - 1) as f32
                || world_back[new_index].material == Material::Water
            {
                // Try to flow left or right.
                let directions = vec![vec2(-1.0, 0.0), vec2(1.0, 0.0)];
                for direction in directions.choose() {
                    let new_cell_pos = current_pos + *direction;
                    if new_cell_pos.x < 0.0 || new_cell_pos.x > (WORLD_SIZE.x - 1) as f32 {
                        continue;
                    }
                    let new_index = get_index(new_cell_pos.x as u32, new_cell_pos.y as u32);
                    if world_back[new_index].material == Material::Air {
                        return Some(new_index);
                    }
                }
                None
            } else {
                // It's free.

                // If the new position is the same as the old, don't return an updated position.
                // if new_cell_pos == current_pos {
                //     return None;
                // }
                Some(new_index)
            }
        }
        Material::Lava => Some(get_index(x, y)),
        Material::Sand => Some(get_index(x, y)),
    }
}

#[macroquad::main("Innojam13")]
async fn main() {
    let mut world_front = vec![Cell::default(); (WORLD_SIZE.x * WORLD_SIZE.y) as usize];
    let mut world_back = vec![Cell::default(); (WORLD_SIZE.x * WORLD_SIZE.y) as usize];

    let mut image = Image::gen_image_color(WORLD_SIZE.x as u16, WORLD_SIZE.y as u16, WHITE);
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    let zoom_factor = 16.0;

    loop {
        clear_background(LIGHTGRAY);

        let (mx, my) = mouse_position();
        if is_mouse_button_down(MouseButton::Left) {
            let world_pos = vec2(mx / zoom_factor, my / zoom_factor);
            world_front[(WORLD_SIZE.x * world_pos.y as u32) as usize + world_pos.x as usize] =
                Cell {
                    velocity: Vec2::new(0.0, 1.0),
                    material: Material::Water,
                }
        }

        for y in (0..WORLD_SIZE.y as u32).rev() {
            for x in 0..WORLD_SIZE.x as u32 {
                // Update stuff
                let current_cell = &world_front[get_index(x, y)];
                if let Some(new_pos) = calculate_new_position(x, y, &world_front, &world_back) {
                    world_back[new_pos as usize] = current_cell.clone();
                    // world_back[get_index(x, y)].material = Material::Air;
                } else {
                    world_back[get_index(x, y)] = current_cell.clone();
                }
            }
        }

        // Flip backbuffer to frontbuffer.
        std::mem::swap(&mut world_front, &mut world_back);

        for i in 0..world_back.len() {
            // world_front[i] = world_back[i].clone();

            // Also set the texture.
            image.set_pixel(
                i as u32 % WORLD_SIZE.x,
                i as u32 / WORLD_SIZE.x,
                match world_front[i as usize].material {
                    Material::Air => WHITE,
                    Material::Water => BLUE,
                    Material::Lava => RED,
                    Material::Sand => YELLOW,
                },
            );
        }
        world_back.fill(Cell::default());

        texture.update(&image);

        // set_camera(&Camera2D {
        //     zoom: vec2(1., 1.),
        //     ..Default::default()
        // });

        let texture_params = DrawTextureParams {
            dest_size: Some(WORLD_SIZE.as_vec2() * zoom_factor),
            ..Default::default()
        };
        draw_texture_ex(texture, 0., 0., WHITE, texture_params);

        // set_default_camera();

        next_frame().await
    }
}
