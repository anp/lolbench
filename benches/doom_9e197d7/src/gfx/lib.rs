#[macro_use]
extern crate glium;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

extern crate common;
extern crate glium_sdl2;
extern crate libc;
extern crate math;
extern crate sdl2;
extern crate sdl2_ttf;
extern crate slab;

pub use error::{Error, Result};
pub use scene::{Scene, SceneBuilder};
pub use text::{Text, TextId, TextRenderer};
pub use vertex::{DecorBufferBuilder, FlatBufferBuilder, SkyBufferBuilder, WallBufferBuilder};
pub use vertex::{SkyBuffer, SkyVertex, SpriteBuffer, SpriteVertex, StaticBuffer, StaticVertex};
pub use window::Window;

mod error;
mod platform;
mod scene;
mod vertex;
mod window;
mod text;

use math::Vec2f;

#[derive(Copy, Clone, Debug)]
pub struct Bounds {
    pub pos: Vec2f,
    pub size: Vec2f,
    pub num_frames: usize,
    pub row_height: usize,
}
