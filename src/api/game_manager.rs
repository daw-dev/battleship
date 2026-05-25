use rocket::{State, get, put, response::status::NotFound, serde::json::Json};
use serde::{Deserialize, Serialize};

use crate::{
    api::state::{GameState, GamesRef},
    game::{Player, boat::Boat},
};

#[derive(Serialize)]
struct ConnectResponse {
    game_id: usize,
    role: Player,
}

#[get("/connect")]
pub fn connect_to_game(games: &State<GamesRef>) -> Json<ConnectResponse> {
    let mut games = games.write().unwrap();
    match games.next_game.take() {
        Some(id) => {
            games.running_games.insert(
                id,
                crate::api::state::GameState::Setup {
                    challenger_boats: Vec::new(),
                    challenged_boats: Vec::new(),
                },
            );
            Json(ConnectResponse {
                game_id: id,
                role: Player::Challenged,
            })
        }
        None => {
            let next_game = games
                .running_games
                .keys()
                .last()
                .map(|game| game + 1)
                .unwrap_or(0);
            games.next_game = Some(next_game);
            Json(ConnectResponse {
                game_id: next_game,
                role: Player::Challenger,
            })
        }
    }
}

#[put("/game/<id>/<player>/setup", data = "<boats>")]
pub fn setup(
    games: &State<GamesRef>,
    id: usize,
    player: Player,
    boats: Json<Vec<Boat>>,
) -> Result<Json<&'static str>, NotFound<Json<&'static str>>> {
    let mut games = games.write().unwrap();
    let this_game = games.running_games.get_mut(&id);
    match this_game {
        Some(GameState::Setup { challenger_boats, challenged_boats }) => {
            let setup_done = match player {
                Player::Challenger => {
                    *challenger_boats = boats.into_inner();
                    !challenged_boats.is_empty()
                }
                Player::Challenged => {
                    *challenged_boats = boats.into_inner();
                    !challenger_boats.is_empty()
                }
            };
            if setup_done {
                Ok(Json("done"))
            } else {
                Ok(Json("waiting"))
            }
        }
        Some(GameState::Playing(_)) => {
            Err(NotFound(Json("that game is already going")))
        }
        None => {
            Err(NotFound(Json("that game does not exist")))
        }
    }
}
