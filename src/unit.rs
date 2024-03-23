use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    transform,
};

use rand::Rng;

use crate::{
    asset_loader::GameAssets,
    cursor::CursorGridClickEvent,
    hex_pos::HexPos,
    movement::{MovementHex, MovementRange, Mp},
    obstacle::Obstacle,
    schedule::InGameSet,
    Axial, Vec3Extra,
};

#[derive(Component)]
pub struct Unit;

#[derive(Resource, Default)]
pub struct CurrentActiveUnit {
    pub entity: Option<Entity>,
}

pub struct UnitPlugin;

impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentActiveUnit>()
            .add_systems(PostStartup, spawn_units.in_set(InGameSet::InitEntities))
            .add_systems(
                PostStartup,
                init_movement_range.in_set(InGameSet::InitMovementRange),
            )
            .add_systems(
                Update,
                (unit_clicked, despawn_movement_range, spawn_movement_range).chain(),
            );
    }
}

fn spawn_units(mut commands: Commands, game_assets: Res<GameAssets>) {
    let units = vec![Axial::new(0, 0), Axial::new(1, -1)];

    for pos in units {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(game_assets.circle.clone()),
                material: game_assets.red.clone(),
                transform: Transform {
                    translation: pos.to_vec3_pixel().set_z(2.),
                    ..default()
                },
                ..default()
            },
            HexPos { pos },
            Unit,
            Obstacle,
            MovementRange::default(),
            Mp::new(4),
            Name::new("Unit"),
        ));
    }
}

fn init_movement_range(
    obstacle_query: Query<&HexPos, With<Obstacle>>,
    mut movement_query: Query<(&HexPos, &Mp, &mut MovementRange)>,
) {
    for (hex_pos, mp, mut movement_range) in movement_query.iter_mut() {
        let obstacles = obstacle_query
            .iter()
            .map(|pos| pos.pos)
            .collect::<Vec<Axial>>();

        movement_range.hexes = hex_pos.pos.new_bfs(&obstacles, mp.current);
    }
}

fn unit_clicked(
    mut cursor_grid_click_event_r: EventReader<CursorGridClickEvent>,
    unit_query: Query<(&HexPos, Entity), With<Unit>>,
    mut selected_unit: ResMut<CurrentActiveUnit>,
) {
    for event in cursor_grid_click_event_r.read() {
        let units = unit_query.iter().collect::<Vec<(&HexPos, Entity)>>();
        let unit = units
            .iter()
            .find(|(pos, _)| pos.pos == event.pos)
            .map(|(_, entity)| *entity);

        match unit {
            Some(entity) => {
                selected_unit.entity = Some(entity);
            }
            None => {}
        }
    }
}

fn spawn_movement_range(
    current_active_unit: Res<CurrentActiveUnit>,

    mut commands: Commands,
    game_assets: Res<GameAssets>,
    movement_range_query: Query<&MovementRange>,
) {
    if current_active_unit.is_changed() {
        match current_active_unit.entity {
            Some(entity) => {
                if let Ok(movement_range) = movement_range_query.get(entity) {
                    for hex in &movement_range.hexes {
                        commands.spawn((
                            MaterialMesh2dBundle {
                                mesh: Mesh2dHandle(game_assets.hexagone.clone()),
                                material: game_assets.movement_hex_color.clone(),
                                transform: Transform {
                                    translation: hex.hex.to_vec3_pixel().set_z(1.),
                                    ..default()
                                },
                                ..default()
                            },
                            MovementHex {
                                distance: hex.distance,
                            },
                            HexPos { pos: hex.hex },
                            Name::new("MovementHex"),
                        ));
                    }
                }
            }
            None => {}
        }
    }
}

fn despawn_movement_range(
    current_active_unit: Res<CurrentActiveUnit>,
    mut commands: Commands,
    movement_hex_query: Query<Entity, With<MovementHex>>,
) {
    if current_active_unit.is_changed() {
        for entity in movement_hex_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
