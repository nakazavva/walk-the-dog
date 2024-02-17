#[macro_use]
mod browser;
mod engine;
mod game;

use engine::GameLoop;
use game::WalkTheDog;
use wasm_bindgen::JsValue;

pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    browser::spawn_local(async move {
        let game = WalkTheDog::new();

        GameLoop::start(game)
            .await
            .expect("Could not start game loop");
    });

    Ok(())
}
