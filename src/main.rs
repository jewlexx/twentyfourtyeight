use bevy::prelude::*;

mod board;
mod game;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    App::new().add_plugins(DefaultPlugins).run();

    // let game = game::Game::new()?;

    Ok(())
}
