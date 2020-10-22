use rltk::{Rltk, GameState, RGB};
use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;
pub use rect::Rect;

pub struct State {
  ecs: World,
}

impl State {
  fn run_system(&mut self) {
    self.ecs.maintain();
  }
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut Rltk) {
    ctx.cls();
    player_input(self, ctx);
    self.run_system();

    let map = self.ecs.fetch::<Vec<TileType>>();
    draw_map(&map, ctx);

    let positions = self.ecs.read_storage::<Position>();
    let renderables = self.ecs.read_storage::<Renderable>();

    for (pos, render) in (&positions, &renderables).join() {
      ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
    }
  }
}

fn main() {
  use rltk::RltkBuilder;
  let context = RltkBuilder::simple80x50()
    .with_tile_dimensions(12, 12)
    .with_title("Roguelike Tutorial")
    .build().unwrap();
  let mut gs = State {
    ecs: World::new(),
  };

  gs.ecs.register::<Position>();
  gs.ecs.register::<Renderable>();
  gs.ecs.register::<Player>();

  let (map, rooms) = new_map_rooms_and_corridors();

  gs.ecs.insert(map);

  let (px, py) = rooms[0].center();

  gs.ecs
    .create_entity()
    .with(Position { x: px, y: py })
    .with(Renderable {
      glyph: rltk::to_cp437('@'),
      fg: RGB::named(rltk::YELLOW),
      bg: RGB::named(rltk::RED),
    })
    .with(Player {})
    .build();

  rltk::main_loop(context, gs).unwrap();
}
