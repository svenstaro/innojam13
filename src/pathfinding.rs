use bevy::prelude::*;
use bevy_rapier2d::prelude::ExternalForce;

pub struct PathfindingPlugin;

#[derive(Component)]
pub struct PathfindingAgent {
    move_strength: f32,
    current_idx: usize,
}

#[derive(Default)]
struct Navmesh {
    nodes: Vec<Vec2>,
}

impl PathfindingAgent {
    pub fn new(move_strength: f32) -> Self {
        PathfindingAgent {
            move_strength,
            current_idx: 0,
        }
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
                let t = step_nr as f32 / steps as f32;
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
            transform: Transform::from_translation(Vec3::new(node.x, node.y, 1.9)),
            ..default()
        });
    }
}

fn update_pathfinding_agent(
    mut agent_query: Query<
        (&Transform, &mut PathfindingAgent, &mut ExternalForce),
        With<PathfindingAgent>,
    >,
    navmesh: Res<Navmesh>,
) {
    for (agent_transform, mut agent, mut agent_move_force) in agent_query.iter_mut() {
        agent_move_force.force =
            get_force_from_navmesh(agent_transform.translation, &navmesh, &mut agent);
    }
}

fn get_force_from_navmesh(
    sample_position: Vec3,
    navmesh: &Navmesh,
    agent: &mut PathfindingAgent,
) -> Vec2 {
    let sample_position = sample_position.truncate();

    loop {
        if agent.current_idx >= (navmesh.nodes.len() - 1) {
            return Vec2::ZERO;
         }
        let current_node = navmesh.nodes[agent.current_idx];
        let dir = current_node - sample_position;
        let dist = current_node.distance(sample_position);

        if dist < 50.0 {
            agent.current_idx = agent.current_idx + 1;
            
            continue;
        }
        return dir.normalize() * agent.move_strength;
    }
}

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Navmesh::generate(
            vec![
                Vec2::new(180.0, 135.0), 
                Vec2::new(1160.0, 135.0),
                Vec2::new(1160.0, 460.0),
                Vec2::new(640.0, 800.0),
                Vec2::new(100.0, 800.0),
                Vec2::new(100.0, 1150.0),
                Vec2::new(330.0, 1400.0),
                Vec2::new(1110.0, 1400.0),
                Vec2::new(1110.0, 1800.0),
                Vec2::new(1500.0, 1800.0),
            ],
            0.1,
        ));
        app.add_startup_system(init_nav_mesh_debug);
        app.add_system(update_pathfinding_agent);
    }
}

