use rltk::{VirtualKeyCode, Rltk};
use specs::prelude::*;
use std::cmp::{min, max};
use super::{Position, Player, TileType, State, Map, Viewshed};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
  let mut positions = ecs.write_storage::<Position>();
  let mut players = ecs.write_storage::<Player>();
  let mut viewshed = ecs.write_storage::<Viewshed>();
  let map = ecs.fetch::<Map>();

  for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewshed).join() {
    let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);
    if map.tiles[destination_idx] != TileType::Wall {
      pos.x = min(79, max(0, pos.x + delta_x));
      pos.y = min(49, max(0, pos.y + delta_y));

      viewshed.dirty = true;
    }
  }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
  // PLayer movement
  match ctx.key {
    None => {},
    Some(key) => match key {
      // Left
      VirtualKeyCode::Left |
        VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),

      // Right
      VirtualKeyCode::Right |
        VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),

      // Up
      VirtualKeyCode::Up |
        VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),

      // Down
      VirtualKeyCode::Down |
        VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs,),
      _ => {}
    }
  }
}