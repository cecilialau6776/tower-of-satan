use crate::{consts, OnGround, Player, Velocity};
use bevy::prelude::*;
use devcaders::DevcadeControls;

pub struct InputPlugin;
impl Plugin for InputPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(input_handler);
  }
}

fn input_handler(
  controls: DevcadeControls,
  mut player_query: Query<(&mut Velocity, &mut OnGround), With<Player>>,
  time: Res<Time>,
) {
  let (mut player_velocity, mut on_ground) = player_query.single_mut();
  if on_ground.0 && controls.just_pressed(devcaders::Player::P1, devcaders::Button::A1) {
    player_velocity.y += 6.0;
    on_ground.0 = false;
  }
  if controls.pressed(devcaders::Player::P1, devcaders::Button::StickLeft) {
    player_velocity.x -= consts::PLAYER_SPEED * time.delta_seconds();
  }
  if controls.pressed(devcaders::Player::P1, devcaders::Button::StickRight) {
    player_velocity.x += consts::PLAYER_SPEED * time.delta_seconds();
  }
}
