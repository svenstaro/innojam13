use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::{
    game_state::AppState,
    input::get_world_cursor_pos,
    polishing_constants::{WATER_SIZE, WATER_STRENGTH},
    MainCamera, level::LevelComponent,
};

#[derive(Debug, Default)]
pub struct SpawnCannonGadgetEvent;

pub struct GadgetPlugin;

#[derive(Component, Default)]
pub struct Gadget {
    pub is_placed: bool,
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

const SNAP_ON_DIST: f32 = 300.0;

impl Plugin for GadgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnCannonGadgetEvent>()
            // .add_system(shoot_water_system)
            .add_system(on_gadget_placment_status_change)
            .add_system(handle_spawn_cannons)
            .add_system(update_gadget_placement);
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
    asset_server: Res<AssetServer>,
) {
    if spawn_cannon_events.is_empty() || *app_state.current() != AppState::Build {
        return;
    }
    spawn_cannon_events.clear();

    if let Some(position) = get_world_cursor_pos(windows, camera_q) {
        if let Some(position) = snap_to_surface(position) {
            let cannon_component = CannonGadget {
                emission_strength: 10.0,
                shots_per_second: 10.0,
            };

            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(1.0)),
                        ..default()
                    },
                    texture: asset_server.load("items/CanonWater.png"),
                    transform: Transform::from_scale(Vec3::new(379.0, 512.0, 1.0))
                        .with_translation(Vec3::ONE),
                    ..default()
                })
                .insert(Sprite {
                    custom_size: Some(Vec2::splat(0.5)),
                    ..default()
                })
                // .insert(cannon_component)
                .insert(Gadget { is_placed: false })
                .insert(GadgetPart { is_placed: false });
        }
    };
}

fn update_gadget_placement(
    mut commands: Commands,
    mut gadget_query: Query<(&mut Gadget, &mut Transform)>,
    windows: Res<Windows>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    if let Some(position) = get_world_cursor_pos(windows, camera_q) {
        for (mut gadget, mut gadget_transform) in &mut gadget_query {
            if mouse_buttons.just_released(MouseButton::Left) {
                gadget.is_placed = true;
            }
            if !gadget.is_placed {
                if let Some(position) = snap_to_surface(position) {
                    gadget_transform.translation = Vec3::new(position.x, position.y, 1.0);
                }
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

fn snap_to_surface(sample_point: Vec2) -> Option<Vec2> {
    struct Edge {
        left: Vec2,
        right: Vec2,
    }

    let snap_edges: Vec<Edge> = vec![
        Edge {
            left: Vec2::new(35.0, 185.0),
            right: Vec2::new(3165.0, 185.0),
        },
        Edge {
            left: Vec2::new(35.0, 835.0),
            right: Vec2::new(650.0, 835.0),
        },
        Edge {
            left: Vec2::new(400.0, 1430.0),
            right: Vec2::new(1100.0, 1430.0),
        },
        Edge {
            left: Vec2::new(1100.0, 1820.0),
            right: Vec2::new(2000.0, 1820.0),
        },
    ];

    for edge in snap_edges {
        if sample_point.x < edge.left.x || sample_point.x > edge.right.x {
            continue;
        }

        let t = (edge.right.x - sample_point.x) / (edge.right.x - edge.left.x);

        let snap_on_target = edge.right.lerp(edge.left, t);
        let dist = (sample_point.y - snap_on_target.y).abs();
        if dist < SNAP_ON_DIST {
            dbg!(t);
            return Some(snap_on_target);
        }
    }

    None
}

fn shoot_water_system(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    if buttons.pressed(MouseButton::Left) {
        if let Some(position) = get_world_cursor_pos(windows, camera_q) {
            shoot_water(
                Vec3::new(position.x, position.y, 0.0),
                Vec3::new(position.x, position.y, 0.0) + Vec3::NEG_ONE,
                &mut meshes,
                &mut materials,
                &mut commands,
            );
        }
    }
}

pub fn shoot_water(
    shoot_pos: Vec3,
    target_pos: Vec3,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
) {
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
            impulse: (target_pos - shoot_pos).truncate().normalize() * WATER_STRENGTH,
            torque_impulse: 0.0,
        })
        .insert(LevelComponent)
        .insert_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::from_xyz(shoot_pos.x, shoot_pos.y, 0.96)
                .with_scale(Vec3::splat(WATER_SIZE)),
            material: materials.add(ColorMaterial::from(Color::hex("27636E").unwrap())),
            ..default()
        });
}
