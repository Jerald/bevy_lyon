use bevy::{
    prelude::*,
};

use bevy_lyon::{
    shapes,
    LyonMeshBuilder,
    math
};

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(setup_system.system())
        .run();
}

fn setup_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let red = materials.add(Color::RED.into());
    let green = materials.add(Color::GREEN.into());
    let blue = materials.add(Color::BLUE.into());

    let fill_circle = meshes.add(LyonMeshBuilder::with_only(
        shapes::FillCircle {
            center: math::point(-100.0, 0.0),
            ..Default::default()
        }
    ));

    let stroke_circle = meshes.add(LyonMeshBuilder::with_only(
        shapes::StrokeCircle {
            center: math::point(-100.0, 0.0),
            radius: 35.0,
            ..Default::default()
        }
    ));

    let ellipse = meshes.add(LyonMeshBuilder::with_only(
        shapes::StrokeEllipse {
            center: math::point(50.0, 25.0),
            ..Default::default()
        }
    ));

    let convex_polyline = meshes.add(LyonMeshBuilder::with_only(
        shapes::FillConvexPolyline {
            points: vec![
                math::point(0.0, 0.0),
                math::point(25.0, 50.0),
                math::point(50.0, 0.0),
                math::point(50.0, -100.0),
                math::point(25.0, -150.0),
                math::point(0.0, -100.0)
            ],
            ..Default::default()
        }
    ));

    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            mesh: fill_circle,
            material: red,
            sprite: Sprite { size: Vec2::new(1.0, 1.0) },
            ..Default::default()
        })
        .spawn(SpriteComponents {
            mesh: stroke_circle,
            material: green,
            sprite: Sprite { size: Vec2::new(1.0, 1.0) },
            ..Default::default()
        })
        .spawn(SpriteComponents {
            mesh: ellipse,
            material: red,
            sprite: Sprite { size: Vec2::new(1.0, 1.0) },
            ..Default::default()
        })
        .spawn(SpriteComponents {
            mesh: convex_polyline,
            material: blue,
            sprite: Sprite { size: Vec2::new(1.0, 1.0) },
            translation: Translation::new(25.0, 75.0, 0.0),
            ..Default::default()
        });

}