use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{asset_loader::GameAssets, Axial};
use crate::{hex_pos::HexPos, Vec3Extra};

#[derive(Component)]
pub struct Hex;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, generate_map);
    }
}

fn generate_map(mut commands: Commands, game_assets: Res<GameAssets>) {
    let text_style = TextStyle {
        font: game_assets.font.clone(),
        font_size: 12.0,
        color: Color::BLACK,
    };

    for i in -5..6 {
        for j in -6..7 {
            let hex = Axial::new(i, j);
            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(hex.to_string(), text_style.clone())
                        .with_justify(JustifyText::Center),
                    transform: Transform {
                        translation: hex.clone().to_vec3_pixel().set_z(10.),
                        ..default()
                    },
                    ..default()
                },
                Name::new("HexText"),
            ));
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(game_assets.hexagone.clone()),
                    material: game_assets.white.clone(),
                    transform: Transform {
                        translation: hex.to_vec3_pixel(),
                        ..default()
                    },
                    ..default()
                },
                Hex,
                HexPos { pos: hex },
                Name::new("Hex"),
            ));
        }
    }
}
