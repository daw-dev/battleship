use rocket::{launch, routes};

use crate::api::game_manager::*;

mod api;
mod game;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(api::state::Games::new())
        .mount("/", routes![connect_to_game, setup])
}
