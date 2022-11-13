use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::{input::get_world_cursor_pos, MainCamera, game_state::AppState};

#[derive(Debug, Default)]
pub struct SpawnCannonGadgetEvent;

pub struct GadgetPlugin;

#[derive(Component, Default)]
pub struct Gadget {
    is_placed: bool,
}

#[derive(Component, Default)]
pub struct GadgetPart {
    is_placed: bool,
}

#[derive(Component)]
pub struct CannonGadget {
    emission_strength: f32,
    shots_per_second: f32,
}

impl Plugin for GadgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnCannonGadgetEvent>();
        app.add_system(shoot_water);
        app.add_system(on_gadget_placment_status_change);
        app.add_system(handle_spawn_cannons);
        app.add_system(update_gadget_placement);
    }
}

fn handle_spawn_cannons(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    spawn_cannon_events: EventReader<SpawnCannonGadgetEvent>,
    app_state: Res<State<AppState>>,
    windows: Res<Windows>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if spawn_cannon_events.is_empty() || *app_state.current() != AppState::Build {
        return;
    }
    spawn_cannon_events.clear();

    if let Some(position) = get_world_cursor_pos(windows, camera_q) {
        //TODO: grid for gadget placement?
        let cannon_component = CannonGadget {
            emission_strength: 10.0,
            shots_per_second: 10.0,
        };

        let barrel_entity = commands
            .spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(shape::Quad::new(Vec2::new(0.3, 1.0))))
                    .into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform::from_xyz(position.x, position.y, 1.0)
                    .with_scale(Vec3::splat(200.0)),
                ..default()
            })
            .insert(cannon_component)
            .insert(Gadget { is_placed: false })
            .insert(GadgetPart { is_placed: false })
            .id();
    };
}

fn update_gadget_placement(
    mut commands: Commands,
    mut gadget_query: Query<(&mut Gadget, &mut Transform) >,
    windows: Res<Windows>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_buttons: Res<Input<MouseButton>>
) {
   

    if let Some(position) = get_world_cursor_pos(windows, camera_q) {
        for (mut gadget, mut gadget_transform) in &mut gadget_query {
            if mouse_buttons.just_released(MouseButton::Left){
                gadget.is_placed = true;
            }
            if !gadget.is_placed {
                gadget_transform.translation = Vec3::new(position.x, position.y, 1.0);
            }
        }
    }
}

fn on_gadget_placment_status_change(
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    changed_gadget_part_query: Query<(&Handle<ColorMaterial>, &GadgetPart), Changed<GadgetPart>>,
) {
    for (material_handle, gadget_part) in changed_gadget_part_query.iter() {
        if let Some(material) = color_materials.get_mut(material_handle) {
            let alpha = if gadget_part.is_placed { 1.0 } else { 0.7 };
            material.color.set_a(alpha);
        }
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
                .insert(CollisionGroups::new(
                    Group::GROUP_2,
                    Group::GROUP_1 | Group::GROUP_2,
                ))
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
