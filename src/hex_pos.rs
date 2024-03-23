use bevy::prelude::*;

use crate::Axial;

pub struct PositionPlugin;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HexPos {
    pub pos: Axial,
}

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HexPos>();
    }
}
