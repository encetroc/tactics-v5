use bevy::prelude::*;

use crate::{HEX_RADIUS, UNIT_RADIUS};

#[derive(Resource, Default)]
pub struct GameAssets {
    pub hexagone: Handle<Mesh>,
    pub circle: Handle<Mesh>,
    pub white: Handle<ColorMaterial>,
    pub black: Handle<ColorMaterial>,
    pub gray: Handle<ColorMaterial>,
    pub red: Handle<ColorMaterial>,
    pub blue: Handle<ColorMaterial>,
    pub movement_hex_color: Handle<ColorMaterial>,
    pub cursor_color: Handle<ColorMaterial>,
    pub font: Handle<Font>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(
    mut scene_assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    *scene_assets = GameAssets {
        hexagone: meshes.add(RegularPolygon::new(HEX_RADIUS, 6)),
        circle: meshes.add(Circle::new(UNIT_RADIUS)),
        white: materials.add(Color::WHITE),
        black: materials.add(Color::BLACK),
        gray: materials.add(Color::GRAY),
        red: materials.add(Color::RED),
        blue: materials.add(Color::BLUE),
        movement_hex_color: materials.add(Color::rgba(0., 1., 0., 0.3)),
        cursor_color: materials.add(Color::rgba(0., 1., 1., 0.5)),
        font: asset_server.load("Kenney Pixel.ttf"),
    }
}
