use bevy::{math::vec3, prelude::*};

use crate::{
    gadget::{shoot_water, Gadget},
    polishing_constants::{GADGET_MIN_DISTANCE, SHOOTING_CHANCE},
};

pub struct AttackSystemPlugin;

impl Plugin for AttackSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(attack_system);
    }
}

fn attack_system(
    gadgets: Query<(&Transform, &Gadget)>,
    enemies: Query<&Transform, With<crate::enemy::Enemy>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for (transform, gadget) in gadgets.iter() {
        if !gadget.is_placed {
            continue;
        }

        //new variable random value between 0 and 1
        let random_value = rand::random::<f32>();

        if random_value < SHOOTING_CHANCE {
            continue;
        }

        let mut min_distance = f32::INFINITY;
        let mut enemy_position = None;
        for enemy in enemies.iter() {
            let gadget_pos = transform.translation;
            let loc_enemy_pos = enemy.translation;
            let distance = (gadget_pos - loc_enemy_pos).length();

            if distance < min_distance {
                min_distance = distance;
                enemy_position = Some(loc_enemy_pos);
            }
        }

        if min_distance > GADGET_MIN_DISTANCE {
            continue;
        }

        //now we have the nearest enemy
        // shoot wa'er
        if let Some(pos) = enemy_position {
            if !is_shooting_down(transform.translation.truncate(), pos.truncate()) {
                shoot_water(
                    transform.translation,
                    pos,
                    &mut meshes,
                    &mut materials,
                    &mut commands,
                );
            } 
        }
    }

    fn is_shooting_down(cannon_pos: Vec2, enemy_pos: Vec2) -> bool {
        let direction_vec = (enemy_pos - cannon_pos).normalize();

        if direction_vec.y > 0.0 {
            return false;
        }

        let a = Vec2::X;
        let b = direction_vec;

        let angle = (a.dot(b) / (a.length() * b.length())).acos().to_degrees();

        dbg!(angle);

        angle > 20.0 && angle < 160.0


    }
}
