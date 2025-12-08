use crate::game::Game;

mod game;

#[tokio::main]
async fn main() {
    let mut game: Game = Game::new(24, 28);
    game.set_target_fps(2);

    game.start().await;
}
