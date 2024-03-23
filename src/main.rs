mod asset_loader;
mod cursor;
mod gui;
mod hex_pos;
mod map;
mod movement;
mod obstacle;
mod schedule;
mod unit;

use std::ops::Add;

use bevy::{
    log::LogPlugin,
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, EguiPlugin},
    egui,
    quick::WorldInspectorPlugin,
};
use map::Hex;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(
                        WINDOW.w * WINDOW.zoom,
                        WINDOW.h * WINDOW.zoom,
                    )
                    .with_scale_factor_override(WINDOW.zoom),
                    resizable: true,
                    ..default()
                }),
                ..default()
            }),
    )
    .add_systems(PostStartup, spawn_camera)
    .add_plugins(EguiPlugin)
    // .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin) // adds default options and `InspectorEguiImpl`s
    // .add_systems(Update, inspector_ui)
    .add_plugins(asset_loader::AssetLoaderPlugin)
    .add_plugins(map::MapPlugin)
    .add_plugins(obstacle::ObstaclePlugin)
    .add_plugins(hex_pos::PositionPlugin)
    .add_plugins(unit::UnitPlugin)
    .add_plugins(schedule::SchedulePlugin)
    .add_plugins(cursor::CursorPlugin)
    .add_plugins(movement::MovementPlugin)
    .run();
    // bevy_mod_debugdump::print_schedule_graph(&mut app, PostUpdate);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);

            egui::CollapsingHeader::new("Materials").show(ui, |ui| {
                bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            });

            ui.heading("Entities");
            bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);
        });
    });
}

// lib

struct WindowSize {
    w: f32,
    h: f32,
    zoom: f32,
}

const WINDOW: WindowSize = WindowSize {
    w: 1200.,
    h: 720.,
    zoom: 2.,
};

const HEX_RADIUS: f32 = 30.0;
const UNIT_RADIUS: f32 = 20.0;
const SQUART3: f32 = 1.7320508;

const AXIAL_TO_PIXEL_MAT: Mat3 =
    Mat3::from_cols_array(&[SQUART3, 0., 0., SQUART3 / 2., 3. / 2., 0., 0., 0., 0.]);

const PIXEL_TO_AXIAL_MAT: Mat3 =
    Mat3::from_cols_array(&[SQUART3 / 3., 0., 0., -1. / 3., 2. / 3., 0., 0., 0., 0.]);

trait Vec3Extra {
    fn set_z(&self, value: f32) -> Self;
    fn axial_to_cube_vec3(&self) -> Self;
}

impl Vec3Extra for Vec3 {
    fn set_z(&self, value: f32) -> Self {
        Vec3::new(self.x, self.y, value)
    }

    fn axial_to_cube_vec3(&self) -> Self {
        Vec3::new(self.x, self.y, -self.x - self.y)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Default)]
pub struct HexDistance {
    hex: Axial,
    distance: usize,
}

impl HexDistance {
    pub fn new(hex: Axial, distance: usize) -> Self {
        Self { distance, hex }
    }
}

#[derive(Clone)]
pub struct Cube {
    q: isize,
    r: isize,
    s: isize,
}

impl Cube {
    const ZERO: Self = Self { q: 0, r: 0, s: 0 };

    pub fn new(q: isize, r: isize, s: isize) -> Self {
        Self { q, r, s }
    }

    pub fn from_axial(axial: Axial) -> Self {
        Self {
            q: axial.q,
            r: axial.r,
            s: -axial.q - axial.r,
        }
    }

    pub fn from_cude_not_rounded(cube: Vec3) -> Self {
        let mut q = cube.x.round();
        let mut r = cube.y.round();
        let mut s = cube.z.round();

        let q_diff = (q - cube.x).abs();
        let r_diff = (r - cube.y).abs();
        let s_diff = (s - cube.z).abs();

        if (q_diff > r_diff) & (q_diff > s_diff) {
            q = -r - s
        } else if r_diff > s_diff {
            r = -q - s
        } else {
            s = -q - r
        }

        Self {
            q: q as isize,
            r: r as isize,
            s: s as isize,
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Default, Reflect)]
pub struct Axial {
    q: isize,
    r: isize,
}

impl Axial {
    const ZERO: Self = Self { q: 0, r: 0 };

    pub fn new(q: isize, r: isize) -> Self {
        Self { q, r }
    }

    pub fn from_pixel_vec3(pixel_pos: Vec3) -> Self {
        let axial_vec3 = PIXEL_TO_AXIAL_MAT * pixel_pos.set_z(0.) / HEX_RADIUS;
        let cube_vec3 = axial_vec3.axial_to_cube_vec3();
        let cube = Cube::from_cude_not_rounded(cube_vec3);
        Self::from_cube(cube)
    }

    pub fn from_cube(cube: Cube) -> Self {
        Self {
            q: cube.q,
            r: cube.r,
        }
    }

    pub fn distance(&self, other: &Self) -> usize {
        ((self.q - other.q).abs()
            + (self.q + self.r - other.q - other.r).abs()
            + (self.r - other.r).abs()) as usize
            / 2
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.q as f32, self.r as f32, 0.)
    }

    pub fn to_vec3_pixel(&self) -> Vec3 {
        HEX_RADIUS * AXIAL_TO_PIXEL_MAT * self.to_vec3()
    }

    pub fn get_neighbors(&self, obstacles: &Vec<Self>) -> Vec<Self> {
        let mut neighbors: Vec<Self> = Vec::new();
        let neighbor_hexs = [
            Self::new(1, 0),
            Self::new(0, 1),
            Self::new(-1, 1),
            Self::new(1, -1),
            Self::new(-1, 0),
            Self::new(0, -1),
        ];

        for tile in neighbor_hexs.iter() {
            let neighbor = *tile + *self;

            if !obstacles.contains(&neighbor) {
                neighbors.push(neighbor);
            }
        }

        neighbors
    }

    pub fn bfs(&self, obstacles: &Vec<Self>, max_distance: usize) -> Vec<Self> {
        if max_distance == 0 {
            return vec![];
        }
        let mut visited: Vec<Self> = vec![*self];
        let mut queue: Vec<Self> = vec![*self];
        let mut distance: usize = 0;

        while !queue.is_empty() {
            let mut next_queue: Vec<Self> = Vec::new();

            for tile in queue.iter() {
                let neighbors = tile.get_neighbors(obstacles);

                for neighbor in neighbors.iter() {
                    if !visited.contains(neighbor) {
                        visited.push(*neighbor);
                        next_queue.push(*neighbor);
                    }
                }
            }

            queue = next_queue;
            distance += 1;

            if distance == max_distance {
                break;
            }
        }

        visited.retain(|&x| x != *self);

        visited
    }

    pub fn new_bfs(&self, obstacles: &Vec<Self>, max_distance: usize) -> Vec<HexDistance> {
        if max_distance == 0 {
            return vec![];
        }
        let mut distance: usize = 1;
        let mut visited: Vec<HexDistance> = vec![HexDistance::new(*self, 0)];
        let mut queue: Vec<HexDistance> = vec![HexDistance::new(*self, 0)];

        while !queue.is_empty() {
            let mut next_queue: Vec<HexDistance> = Vec::new();

            for HexDistance { hex, distance: _ } in queue.iter() {
                let neighbors = hex.get_neighbors(obstacles);

                for neighbor in neighbors.iter() {
                    if !visited.contains(&HexDistance::new(*neighbor, distance)) {
                        visited.push(HexDistance::new(*neighbor, distance));
                        next_queue.push(HexDistance::new(*neighbor, distance));
                    }
                }
            }

            queue = next_queue;
            distance += 1;

            if distance == max_distance + 1 {
                break;
            }
        }

        visited.retain(|&x| x != HexDistance::new(*self, 0));

        visited
    }

    pub fn to_string(&self) -> String {
        format!("({},{})", self.q, self.r)
    }
}

impl Add for Axial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            q: self.q + other.q,
            r: self.r + other.r,
        }
    }
}
