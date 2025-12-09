use crate::game::Game;

mod game;

#[tokio::main]
async fn main() {
    let mut game: Game = Game::new(16, 24);
    game.set_target_fps(4);

    game.start().await;
}
