use bevy::{
  prelude::*,
  sprite::collide_aabb::{collide, Collision},
};

mod consts;

mod platforming;
use consts::FRICTION_COEFFICIENT;
use platforming::PlatformingPlugin;

mod input;
use input::InputPlugin;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_system(devcaders::close_on_menu_buttons)
    .insert_resource(FixedTime::new_from_secs(1.0 / 60.0))
    .add_startup_system(setup)
    .add_system(update_gravity)
    .add_system(update_drag)
    .add_system(update_position)
    .add_plugin(PlatformingPlugin)
    .add_plugin(InputPlugin)
    .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Platform;

impl Platform {
  fn new(size: Vec2, translation: Vec3) -> (Platform, SpriteBundle) {
    (
      Platform,
      SpriteBundle {
        sprite: Sprite {
          color: Color::rgb(0.9, 0.7, 0.6),
          custom_size: Some(size),
          ..default()
        },
        transform: Transform::from_translation(translation),
        ..default()
      },
    )
  }
}

#[derive(Component, Deref, DerefMut)]
struct OnGround(bool);

#[derive(Component, Deref, DerefMut)]
struct Gravity(f32);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn setup(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
  commands.spawn(Platform::new(
    Vec2::new(100.0, 10.0),
    Vec3::new(0.0, -190.0, 0.0),
  ));
  commands.spawn(Platform::new(
    Vec2::new(50.0, 50.0),
    Vec3::new(75.0, -190.0, 0.0),
  ));
  commands.spawn((
    SpriteBundle {
      sprite: Sprite {
        color: Color::rgb(0.3, 0.9, 0.6),
        custom_size: Some(Vec2::new(30.0, 60.0)),
        ..default()
      },
      transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
      ..default()
    },
    Velocity(Vec2::ZERO),
    Player,
    Gravity(9.81),
    OnGround(false),
  ));
}

fn update_position(mut query: Query<(&mut Transform, &Velocity)>) {
  for (mut transform, velocity) in &mut query {
    transform.translation.x += velocity.x;
    transform.translation.y += velocity.y;
  }
}

fn update_gravity(mut query: Query<(&mut Velocity, &Gravity, Option<&OnGround>)>, time: Res<Time>) {
  for (mut velocity, gravity, on_ground) in &mut query {
    if let Some(on_ground) = on_ground {
      if **on_ground {
        continue;
      }
    }
    velocity.y -= **gravity * time.delta_seconds();
  }
}

fn update_drag(mut query: Query<&mut Velocity>, time: Res<Time>) {
  for mut velocity in &mut query {
    velocity.x *= FRICTION_COEFFICIENT.powf(-time.delta_seconds());
    // velocity.x -= velocity.x.signum() * consts::FRICTION_COEFFICIENT * time.delta_seconds();
  }
}
