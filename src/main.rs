use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_graphics)
        .add_startup_system(setup_physics)
        .add_system(print_ball_altitude)
        .add_system(shoot_water)
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

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
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
