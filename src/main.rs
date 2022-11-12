use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;

#[derive(Debug, Default)]
struct SpawnWaveEvent;

#[derive(Component, Debug, Default)]
enum EnemyType {
    #[default]
    Grunt,
    Swimmer,
    Digger,
    Tank,
}

#[derive(Component, Debug, Default)]
struct Enemy;

fn main() {
    App::new()
        .add_event::<SpawnWaveEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(print_ball_altitude)
        .add_system(shoot_water)
        .add_system(spawn_new_wave_on_event)
        .add_system(enemy_pathfinding)
        .add_system(debug_keymap)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn()
        .insert(Collider::cuboid(500.0, 50.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));
    commands
        .spawn()
        .insert(Collider::cuboid(20.0, 100.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(100.0, 0.0, 0.0)));
    commands
        .spawn()
        .insert(Collider::cuboid(20.0, 100.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(300.0, 0.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
}

fn print_ball_altitude(_positions: Query<&Transform, With<RigidBody>>) {
    // for transform in positions.iter() {
    //     println!("Ball altitude: {}", transform.translation.y);
    // }
}

fn shoot_water(buttons: Res<Input<MouseButton>>, windows: Res<Windows>, mut commands: Commands) {
    if buttons.pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();

        if let Some(position) = window.cursor_position() {
            commands
                .spawn()
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(5.0))
                .insert(Restitution::coefficient(0.1))
                .insert(ExternalImpulse {
                    impulse: Vec2::new(5.0, -5.0),
                    torque_impulse: 0.0,
                })
                .insert_bundle(TransformBundle::from(Transform::from_xyz(
                    position.x - window.width() / 2.0,
                    position.y - window.height() / 2.0,
                    0.0,
                )));
        } else {
            // cursor is not inside the window
        }
    }
}

fn debug_keymap(keys: Res<Input<KeyCode>>, mut spawn_wave_events: EventWriter<SpawnWaveEvent>) {
    // Spawn next wave.
    if keys.pressed(KeyCode::N) {
        spawn_wave_events.send_default();
    }
}

fn rand_f32(l: f32, u: f32) -> f32 {
    rand::thread_rng().gen_range(l..u)
}

fn spawn_new_wave_on_event(
    spawn_wave_events: EventReader<SpawnWaveEvent>,
    windows: Res<Windows>,
    mut commands: Commands,
) {
    // Play a sound once per frame if a collision occurred.
    if spawn_wave_events.is_empty() {
        return;
    }

    // This prevents events staying active on the next frame.
    spawn_wave_events.clear();

    let _window = windows.get_primary().unwrap();

    let wave_size = 10;

    for _ in 0..wave_size {
        let base_transform = Transform::from_xyz(0.0, 0.0, 0.0);
        let offset = Vec3::new(rand_f32(-50.0, 50.0), rand_f32(-50.0, 50.0), 0.0);
        let transform = base_transform.with_translation(base_transform.translation + offset);

        commands
            .spawn()
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(10.0))
            .insert(Damping {
                linear_damping: 0.90,
                angular_damping: 0.5,
            })
            .insert(ExternalForce {
                force: Vec2::new(0.0, 0.0),
                torque: 0.0,
            })
            .insert_bundle(TransformBundle::from(transform))
            .insert(Enemy)
            .insert(EnemyType::Grunt);
    }
}

fn enemy_pathfinding(
    mut enemy_query: Query<(&EnemyType, &mut ExternalForce, &Transform), With<Enemy>>,
) {
    // TODO get target and/or map (to compute a* or something....)
    let target = Vec2::new(500.0, 500.0);
    for (_enemy_type, mut ext_force, transform) in enemy_query.iter_mut() {
        let direction = target - transform.translation.truncate();

        // TODO switch by enemy type
        ext_force.force = direction.normalize() * 10.0;
    }
}
