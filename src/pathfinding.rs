use bevy::{prelude::*, ecs::system::Command};
use bevy_rapier2d::prelude::{ExternalImpulse, ExternalForce};

pub struct PathfindingPlugin;

#[derive(Component)]
pub struct PathfindingAgent {
    move_strength: f32
}

#[derive(Default)]
struct Navmesh {
    nodes: Vec<Vec2>,
}

impl PathfindingAgent {
    pub fn new(move_strength: f32) -> Self {
        PathfindingAgent { move_strength }
    }
}

impl Navmesh {
    fn generate(preset_nodes: Vec<Vec2>, density_npu: f32) -> Self {
        let mut nodes = vec![preset_nodes[0]];

        for i in 1..preset_nodes.len() {
            let last_node = preset_nodes[i - 1];
            let next_node = preset_nodes[i];

            let dist = last_node.distance(next_node);

            let steps = (dist * density_npu) as usize;

            for step_nr in 0..steps {
                let t = steps as f32 / step_nr as f32;
                let new_node = last_node.lerp(next_node, t);
                nodes.push(new_node);
            }
            nodes.push(next_node);
        }

        Navmesh { nodes }
    }
}

fn init_nav_mesh_debug(
    mut commands: Commands,
    navmesh: Res<Navmesh>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for node in navmesh.nodes.iter() {
        commands.spawn_bundle(ColorMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(node.x, node.y, 0.0)),
            ..default()
        });
    };
}

fn update_pathfinding_agent(mut commands: Commands, mut agent_query: Query<(&Transform, &PathfindingAgent, &mut ExternalForce), With<PathfindingAgent>>, navmesh: Res<Navmesh>) {
    for (agent_transform, agent, mut agent_move_force) in agent_query.iter_mut() {
        agent_move_force.force = get_force_from_navmesh(agent_transform.translation, &navmesh, agent.move_strength);
    }
}


fn get_force_from_navmesh(sample_position: Vec3, navmesh: &Navmesh, move_strength: f32) -> Vec2 {
    let sample_position = sample_position.truncate();
    let mut min_dist = f32::MAX;
    let mut force = Vec2::ZERO;
    for node in navmesh.nodes.iter() {
        let dist = sample_position.distance(*node);
        if dist < min_dist {
            min_dist = dist;
            force = (*node - sample_position).normalize() * move_strength;
        }
    }
    force
}

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Navmesh::generate(
            vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(10.0, 0.0),
                Vec2::new(20.0, 0.0),
                Vec2::new(30.0, 0.0),
                Vec2::new(40.0, 10.0),
                Vec2::new(30.0, 10.0),
            ],
            0.1,
        ));
        app.add_startup_system(init_nav_mesh_debug);
        app.add_system(update_pathfinding_agent);
    }
}