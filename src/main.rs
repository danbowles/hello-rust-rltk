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
mod visibility_system;
pub use visibility_system::VisibilitySystem;


pub struct State {
  ecs: World,
}

impl State {
  fn run_system(&mut self) {
    let mut vis = VisibilitySystem{};
    vis.run_now(&self.ecs);
    self.ecs.maintain();
  }
}

impl GameState for State {
  fn tick(&mut self, ctx: &mut Rltk) {
    ctx.cls();
    player_input(self, ctx);
    self.run_system();

    draw_map(&self.ecs, ctx);

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
  gs.ecs.register::<Viewshed>();

  let map: Map = Map::new_map_rooms_and_corridors();
  let (px, py) = map.rooms[0].center();
  gs.ecs.insert(map);


  gs.ecs
    .create_entity()
    .with(Position { x: px, y: py })
    .with(Renderable {
      glyph: rltk::to_cp437('@'),
      fg: RGB::named(rltk::YELLOW),
      bg: RGB::named(rltk::RED),
    })
    .with(Player {})
    .with(Viewshed { visible_tiles: Vec::new(), range: 8, dirty: true })
    .build();

  rltk::main_loop(context, gs).unwrap();
}
