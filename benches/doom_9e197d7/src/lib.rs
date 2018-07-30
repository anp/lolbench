#[macro_use]
extern crate lolbench_support;

extern crate common;
extern crate env_logger;
extern crate game;
extern crate getopts;
extern crate gfx;
extern crate sdl2;
extern crate time;
extern crate wad;

use common::GeneralError;
use game::Level;
use gfx::SceneBuilder;
use gfx::Window;
use std::path::PathBuf;
use wad::{Archive, TextureDirectory};

fn check_wad(wad_file: &str) {
    let sdl = sdl2::init().map_err(|e| GeneralError(e.0)).unwrap();
    let win = Window::new(&sdl, 128, 128).unwrap();

    let wad = Archive::open(
        &wad_file,
        &concat!(env!("CARGO_MANIFEST_DIR"), "/doom.toml"),
    ).unwrap();
    let textures = TextureDirectory::from_archive(&wad).unwrap();
    for level_index in 0..wad.num_levels() {
        let mut scene = SceneBuilder::new(
            &win,
            PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/src/shaders")),
        );
        Level::new(&wad, &textures, level_index, &mut scene).unwrap();
        scene.build().unwrap();
    }
}

wrap_libtest! {
    fn freedoom1(b: &mut Bencher) {
        b.iter(|| check_wad(concat!(env!("CARGO_MANIFEST_DIR"), "/freedoom/freedoom1.wad")));
    }
}

wrap_libtest! {
    fn freedoom2(b: &mut Bencher) {
        b.iter(|| check_wad(concat!(env!("CARGO_MANIFEST_DIR"), "/freedoom/freedoom2.wad")))
    }
}
