use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    asset_loader::GameAssets,
    cursor::CursorGridClickEvent,
    hex_pos::HexPos,
    obstacle::Obstacle,
    schedule::UpdateSet,
    unit::{CurrentActiveUnit, Unit},
    Axial, HexDistance, Vec3Extra,
};

#[derive(Component, Default)]
pub struct MovementRange {
    pub hexes: Vec<HexDistance>,
}

#[derive(Component)]
pub struct Mp {
    pub base: usize,
    pub current: usize,
}

impl Mp {
    pub fn new(base: usize) -> Self {
        Self {
            base,
            current: base,
        }
    }
}

#[derive(Event, Debug)]
pub struct UnitMovedEvent {
    pub entity: Entity,
}

#[derive(Component)]
pub struct MovementHex {
    pub distance: usize,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UnitMovedEvent>().add_systems(
            Update,
            (
                movement_hex_clicked.in_set(UpdateSet::UserInput),
                recalc_movement_range.in_set(UpdateSet::StateCalc),
                despawn_update_movement_range.in_set(UpdateSet::DespawnObjects),
                respawn_movement_range.in_set(UpdateSet::RespawnObjects),
            ),
        );
    }
}

fn movement_hex_clicked(
    mut cursor_grid_click_event_r: EventReader<CursorGridClickEvent>,
    mut unit_query: Query<(&mut HexPos, &mut Mp, &mut Transform), With<Unit>>,
    movement_hex_query: Query<(&HexPos, &MovementHex), Without<Unit>>,
    selected_unit: ResMut<CurrentActiveUnit>,
    mut unit_moved_event_w: EventWriter<UnitMovedEvent>,
) {
    for event in cursor_grid_click_event_r.read() {
        let maybe_hex = movement_hex_query
            .iter()
            .find(|(pos, _)| pos.pos == event.pos);

        match maybe_hex {
            Some((hex, MovementHex { distance })) => match selected_unit.entity {
                Some(unit_entity) => {
                    if let Ok((mut unit_hex_pos, mut mp, mut unit_transform)) =
                        unit_query.get_mut(unit_entity)
                    {
                        // let distance = unit_hex_pos.pos.distance(&hex.pos);

                        println!("before mp: {}, distance: {}", mp.current, distance);

                        mp.current -= distance;
                        unit_hex_pos.pos = hex.pos;
                        unit_transform.translation = hex.pos.to_vec3_pixel().set_z(2.);

                        println!("after mp: {}, distance: {}", mp.current, distance);

                        unit_moved_event_w.send(UnitMovedEvent {
                            entity: unit_entity,
                        });
                    }
                }
                None => {}
            },
            None => {}
        }
    }
}

fn recalc_movement_range(
    mut unit_moved_event_r: EventReader<UnitMovedEvent>,
    obstacle_query: Query<&HexPos, With<Obstacle>>,
    mut movement_query: Query<(&HexPos, &Mp, &mut MovementRange)>,
) {
    for _ in unit_moved_event_r.read() {
        for (hex_pos, mp, mut movement_range) in movement_query.iter_mut() {
            let obstacles = obstacle_query
                .iter()
                .map(|pos| pos.pos)
                .collect::<Vec<Axial>>();

            movement_range.hexes = hex_pos.pos.new_bfs(&obstacles, mp.current);
        }
    }
}

fn respawn_movement_range(
    mp_query: Query<&MovementRange>,
    mut unit_moved_event_r: EventReader<UnitMovedEvent>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    for unit_moved_e in unit_moved_event_r.read() {
        let movement_range = mp_query.get(unit_moved_e.entity).unwrap();

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

fn despawn_update_movement_range(
    mut unit_moved_event_r: EventReader<UnitMovedEvent>,
    mut commands: Commands,
    movement_hex_query: Query<Entity, With<MovementHex>>,
) {
    for _ in unit_moved_event_r.read() {
        for entity in movement_hex_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
