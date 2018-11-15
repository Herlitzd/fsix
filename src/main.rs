//! The simplest possible example that does something.

extern crate ggez;

use ggez::conf;
use ggez::event::{self, EventHandler, Keycode, Mod};
use ggez::graphics::{self, DrawMode, Point2, Vector2};
use ggez::nalgebra as na;
use ggez::{Context, GameResult};

const g: f32 = 1.0;

struct MainState {
  actors: Vec<Actor>,
  screen_height: f32,
  screen_width: f32,
}
struct Actor {
  pos: Point2,
  vel: Vector2,
  mass: f32,
}

impl MainState {
  fn new(ctx: &mut Context) -> GameResult<MainState> {
    let s = MainState {
      actors: Vec::new(),
      screen_width: ctx.conf.window_mode.width as f32,
      screen_height: ctx.conf.window_mode.height as f32,
    };
    Ok(s)
  }
}

fn check_bounds(actor: &Actor, w: f32, h: f32) -> Vector2 {
  if actor.pos.y > h {
    Vector2::new(0.0, -1.0)
  } else if actor.pos.x > w || actor.pos.x < 0.0 {
    Vector2::new(-1.0, 0.0)
  } else {
    Vector2::new(1.0, 1.0)
  }
}

impl event::EventHandler for MainState {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    for actor in &mut self.actors {
      let dir = check_bounds(actor, self.screen_width, self.screen_height);
      println!("{}", dir);
      actor.vel = Vector2::new(actor.vel.x * dir.x, actor.vel.y * dir.y + (g * actor.mass));

      actor.pos += actor.vel;
    }
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx);
    for actor in &self.actors {
      graphics::circle(
        ctx,
        DrawMode::Fill,
        Point2::new(actor.pos[0], actor.pos[1]),
        30.0,
        2.0,
      )?;
    }

    graphics::present(ctx);
    Ok(())
  }
  fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
    match keycode {
      Keycode::Space => {
        self.actors.insert(
          0,
          Actor {
            mass: 0.7,
            pos: Point2::new(100.0, 100.0),
            vel: na::zero(),
          },
        );
      }
      _ => (), // Do nothing
    }
  }
}

pub fn main() {
  let c = conf::Conf::new();
  let ctx = &mut Context::load_from_conf("super_simple", "ggez", c).unwrap();
  let state = &mut MainState::new(ctx).unwrap();
  event::run(ctx, state).unwrap();
}
