use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{asset_loader::GameAssets, Axial, Vec3Extra};

#[derive(Event, Debug)]
pub struct CursorGridClickEvent {
    pub pos: Axial,
}

#[derive(Event, Debug)]
pub struct CursorWorldMoveEvent {
    pub pos: Vec3,
}

#[derive(Event, Debug)]
pub struct CursorGridMoveEvent {
    pub pos: Axial,
}

#[derive(Resource, Default, Debug)]
pub struct CursorGridPos {
    pub pos: Axial,
}

pub struct CursorPlugin;

#[derive(Component)]
struct HexCursor;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorGridPos>()
            .add_event::<CursorGridClickEvent>()
            .add_event::<CursorWorldMoveEvent>()
            .add_event::<CursorGridMoveEvent>()
            .add_systems(PostStartup, spawn_cursor)
            .add_systems(Update, (cursor_move, cursor_click));
    }
}

fn spawn_cursor(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(game_assets.hexagone.clone()),
            material: game_assets.cursor_color.clone(),
            transform: Transform {
                translation: Vec3::new(0., 0., 1.),
                ..default()
            },
            ..default()
        },
        HexCursor,
        Name::new("HexCursor"),
    ));
}

fn calc_cursor_pos(
    window: &Window,
    camera_transform: &Transform,
    cursor_moved_event: &CursorMoved,
) -> (Vec3, Axial) {
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let cursor_pos_adjusted_window =
        (cursor_moved_event.position - window_size / 2.) * Vec2::new(1., -1.);
    let cursor_pos_adjusted_camera =
        camera_transform.compute_matrix() * cursor_pos_adjusted_window.extend(0.).extend(1.);
    let cursor_adjusted_to_grid = Axial::from_pixel_vec3(cursor_pos_adjusted_camera.truncate());

    (
        cursor_pos_adjusted_camera.truncate(),
        cursor_adjusted_to_grid,
    )
}

fn cursor_move(
    mut cursor_grid_move_event_w: EventWriter<CursorGridMoveEvent>,
    mut cursor_world_move_event_w: EventWriter<CursorWorldMoveEvent>,
    mut cursor_grid_pos: ResMut<CursorGridPos>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_query: Query<&mut Transform, (With<HexCursor>, Without<Camera>)>,
    windows: Query<&Window>,
    cameras: Query<&Transform, With<Camera>>,
) {
    let Ok(window) = windows.get_single() else {
        return;
    };

    let Ok(camera_transform) = cameras.get_single() else {
        return;
    };

    let Ok(mut cursor) = cursor_query.get_single_mut() else {
        return;
    };

    for cursor_moved_event in cursor_moved_events.read() {
        let (cursor_pos_adjusted_camera, cursor_adjusted_to_grid) =
            calc_cursor_pos(window, camera_transform, cursor_moved_event);

        cursor_world_move_event_w.send(CursorWorldMoveEvent {
            pos: cursor_pos_adjusted_camera,
        });

        if cursor_grid_pos.pos != cursor_adjusted_to_grid {
            cursor_grid_pos.pos = cursor_adjusted_to_grid;
            // mouse position event in relation to the grid
            cursor_grid_move_event_w.send(CursorGridMoveEvent {
                pos: cursor_adjusted_to_grid,
            });

            cursor.translation = cursor_adjusted_to_grid.to_vec3_pixel().set_z(10.);
        }
    }
}

fn cursor_click(
    mut cursor_grid_click_event_w: EventWriter<CursorGridClickEvent>,
    cursor_grid_pos: Res<CursorGridPos>,
    buttons: Res<ButtonInput<MouseButton>>,
) {
    if buttons.just_released(MouseButton::Left) {
        cursor_grid_click_event_w.send(CursorGridClickEvent {
            pos: cursor_grid_pos.pos,
        });
    }
}
