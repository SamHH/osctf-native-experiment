extern crate find_folder;

use piston_window::{Glyphs, PistonWindow, TextureSettings};

pub fn get(window: &PistonWindow) -> Glyphs {
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    println!("{:?}", assets);

    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();

    return Glyphs::new(font, factory, TextureSettings::new()).unwrap();
}
