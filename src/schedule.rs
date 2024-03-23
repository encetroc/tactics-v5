use bevy::prelude::*;

pub struct SchedulePlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
    InitEntities,
    InitMovementRange,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum UpdateSet {
    UserInput,
    StateCalc,
    DespawnObjects,
    RespawnObjects,
}

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PostStartup,
            (InGameSet::InitEntities, InGameSet::InitMovementRange).chain(),
        )
        .add_systems(
            PostStartup,
            apply_deferred
                .after(InGameSet::InitEntities)
                .before(InGameSet::InitMovementRange),
        )
        .configure_sets(
            Update,
            (
                UpdateSet::UserInput,
                UpdateSet::StateCalc,
                UpdateSet::DespawnObjects,
                UpdateSet::RespawnObjects,
            )
                .chain(),
        )
        .add_systems(
            PostStartup,
            apply_deferred
                .after(UpdateSet::DespawnObjects)
                .before(UpdateSet::RespawnObjects),
        );
    }
}
