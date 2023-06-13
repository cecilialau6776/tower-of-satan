use crate::{OnGround, Platform, Player, Velocity};
use bevy::{
  prelude::*,
  sprite::collide_aabb::{collide, Collision},
};

pub struct PlatformingPlugin;
impl Plugin for PlatformingPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(ground_collision);
  }
}

fn ground_collision(
  mut player_query: Query<(
    (&mut Velocity, &mut Transform, &mut OnGround, &Sprite),
    With<Player>,
  )>,
  platform_query: Query<(&Transform, &Sprite), (With<Platform>, Without<Player>)>,
) {
  let ((mut player_velocity, mut player_transform, mut player_on_ground, player_sprite), ()) =
    player_query.single_mut();
  let player_size = player_sprite.custom_size.unwrap();
  for (platform_transform, platform_sprite) in &platform_query {
    while let Some(collision) = collide(
      player_transform.translation,
      player_size,
      platform_transform.translation,
      platform_sprite.custom_size.unwrap(),
    ) {
      if collision != Collision::Top {
        break;
      }
      player_on_ground.0 = true;
      player_velocity.y = 0.0;
      player_transform.translation.y += 0.1;
    }
  }
}
