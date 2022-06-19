mod board;
mod game;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let game = game::Game::new()?;

    Ok(())
}
