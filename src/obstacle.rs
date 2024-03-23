use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{asset_loader::GameAssets, hex_pos::HexPos, schedule::InGameSet, Axial, Vec3Extra};

pub struct ObstaclePlugin;

#[derive(Component)]
pub struct Obstacle;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_obstacles.in_set(InGameSet::InitEntities));
    }
}

fn spawn_obstacles(mut commands: Commands, game_assets: Res<GameAssets>) {
    let obstacles = vec![
        Axial::new(0, 1),
        Axial::new(1, 1),
        Axial::new(2, 1),
        Axial::new(3, 1),
        Axial::new(-3, 1),
        Axial::new(-3, 0),
        Axial::new(-3, -1),
    ];

    for pos in obstacles {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(game_assets.hexagone.clone()),
                material: game_assets.gray.clone(),
                transform: Transform {
                    translation: pos.to_vec3_pixel().set_z(1.),
                    ..default()
                },
                ..default()
            },
            Obstacle,
            HexPos { pos },
            Name::new("Obstacle"),
        ));
    }
}
