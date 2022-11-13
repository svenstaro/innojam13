use bevy::{math::vec3, prelude::*};

pub struct AttackSystemPlugin;

impl Plugin for AttackSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(attack_system);
    }
}

fn attack_system(mut commands: Commands) {}
