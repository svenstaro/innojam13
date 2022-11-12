
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::{MainCamera, input::get_world_cursor_pos};


pub struct GadgetPlugin;

impl Plugin for GadgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(shoot_water);
    }
}

fn shoot_water(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    if buttons.pressed(MouseButton::Left) {
        if let Some(position) = get_world_cursor_pos(windows, camera_q) {
            commands
                .spawn()
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(0.5))
                .insert(Restitution::coefficient(0.1))
                .insert(ExternalImpulse {
                    impulse: Vec2::new(5.0, -5.0),
                    torque_impulse: 0.0,
                })
                .insert_bundle(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
                    transform: Transform::from_xyz(position.x, position.y, 0.0)
                        .with_scale(Vec3::splat(10.)),
                    material: materials.add(ColorMaterial::from(Color::BLUE)),
                    ..default()
                });
        } else {
            // cursor is not inside the window
        }
    }
}