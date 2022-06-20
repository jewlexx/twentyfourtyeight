use bevy::prelude::*;

mod board;
mod game;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup_system)
        .add_system(update_board)
        .run();

    // let game = game::Game::new()?;

    Ok(())
}

fn update_board(mut game: Query<&mut game::Game>) {
    game.iter_mut().for_each(|game| {
        if !game.is_changed() {
            return;
        }

        let board = game.get_board();

        for cell in board.get_cells() {
            println!("{:?}", cell);
        }
    });
}

fn startup_system(mut commands: Commands) {
    let board = match game::Game::new() {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    commands.spawn().insert(board);
}
