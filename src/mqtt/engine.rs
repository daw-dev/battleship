use std::collections::HashMap;
use itertools::Itertools;
use rumqttc::{AsyncClient, ClientError, Event, EventLoop, MqttOptions, Packet, Publish, QoS};
use serde::{Deserialize, Serialize};
use crate::game::{Game, Role, board::Board, boat::Boat, hit_result::HitResult};

#[derive(Deserialize, Debug)]
struct Register {
    id: String,
}

#[derive(Deserialize, Debug)]
enum Action {
    Shoot(usize, usize),
    Setup(Vec<Boat>),
}

#[derive(Serialize)]
struct TurnInfo {
    turn: Role,
}

struct WaitingGame {
    host_boats: Option<Vec<Boat>>,
    guest_boats: Option<Vec<Boat>>,
}

pub struct Engine {
    client: AsyncClient,
    eventloop: EventLoop,
    register_id: usize,
    active_games: HashMap<usize, Game>,
    waiting_games: HashMap<usize, WaitingGame>,
}

impl Engine {
    pub fn new(mqtt_options: MqttOptions, cap: usize) -> Self {
        let (client, eventloop) = AsyncClient::new(mqtt_options, cap);
        Self {
            client,
            eventloop,
            register_id: 0,
            active_games: HashMap::new(),
            waiting_games: HashMap::new(),
        }
    }

    pub async fn subscribe(&self) -> Result<(), ClientError> {
        self.client
            .subscribe("battleship/register", QoS::AtLeastOnce)
            .await?;
        self.client
            .subscribe("battleship/game/+/+/action", QoS::AtLeastOnce)
            .await?;

        Ok(())
    }

    async fn handle_register(&mut self, register_payload: Register) {
        #[derive(Serialize)]
        struct Register {
            role: Role,
            game_id: usize,
        }

        println!("new register request: {register_payload:?}");
        let role = if self.register_id % 2 == 0 {
            Role::Host
        } else {
            Role::Guest
        };

        let game_id = self.register_id / 2;

        let res = self
            .client
            .publish(
                format!("battleship/{}/assign", register_payload.id),
                QoS::AtLeastOnce,
                false,
                serde_json::to_string(&Register { role, game_id }).unwrap(),
            )
            .await;

        self.waiting_games.entry(game_id).or_insert(WaitingGame {
            host_boats: None,
            guest_boats: None,
        });

        self.register_id += 1;

        if let Err(err) = res {
            eprintln!("couldn't register {}: {}", register_payload.id, err);
        }
    }

    async fn handle_shoot(&mut self, game_id: usize, attacker: Role, position: (usize, usize)) {
        #[derive(Serialize)]
        struct HitInfo {
            attacker: Role,
            hit: HitResult,
            position: (usize, usize),
        }

        let Some(game) = self.active_games.get_mut(&game_id) else {
            eprintln!("no such active game");
            return;
        };

        let hit = game.shoot(position);

        let hit_info_payload = HitInfo {
            attacker,
            hit,
            position,
        };

        let res = self
            .client
            .publish(
                format!("battleship/game/{game_id}/{attacker}/event"),
                QoS::AtLeastOnce,
                false,
                serde_json::to_string(&hit_info_payload).unwrap(),
            )
            .await;

        if let Err(err) = res {
            eprintln!("couldn't send info to attacker: {}", err);
        }

        let res = self
            .client
            .publish(
                format!("battleship/game/{game_id}/{}/event", !attacker),
                QoS::AtLeastOnce,
                false,
                serde_json::to_string(&hit_info_payload).unwrap(),
            )
            .await;

        if let Err(err) = res {
            eprintln!("couldn't send info to defender: {}", err);
        }

        let res = self
            .client
            .publish(
                format!("battleship/game/{game_id}/state"),
                QoS::AtLeastOnce,
                false,
                serde_json::to_string(&TurnInfo { turn: game.turn() }).unwrap(),
            )
            .await;

        if let Err(err) = res {
            eprintln!("couldn't send turn info: {}", err);
        }
    }

    async fn handle_setup(&mut self, game_id: usize, role: Role, boats: Vec<Boat>) {
        let Some(game) = self.waiting_games.get_mut(&game_id) else {
            eprintln!("no such waiting game");
            return;
        };

        let start_game = match role {
            Role::Host => {
                game.host_boats = Some(boats);
                game.guest_boats.is_some()
            }
            Role::Guest => {
                game.guest_boats = Some(boats);
                game.host_boats.is_some()
            }
        };

        if start_game {
            let game = self.waiting_games.remove(&game_id).unwrap();

            let new_game = Game::new(
                Board::new(game.host_boats.unwrap()),
                Board::new(game.guest_boats.unwrap()),
            );

            let first_turn = new_game.turn();

            self.active_games.insert(game_id, new_game);

            let res = self
                .client
                .publish(
                    format!("battleship/games/{game_id}/state"),
                    QoS::AtLeastOnce,
                    false,
                    serde_json::to_string(&TurnInfo { turn: first_turn }).unwrap(),
                )
                .await;

            if let Err(err) = res {
                eprintln!("couldn't send turn info: {err}");
            }
        }
    }

    async fn handle_action(&mut self, game_id: usize, role: Role, action_payload: Action) {
        match action_payload {
            Action::Shoot(x, y) => self.handle_shoot(game_id, role, (x, y)).await,
            Action::Setup(boats) => self.handle_setup(game_id, role, boats).await,
        }
    }

    async fn handle_packet(&mut self, packet: Publish) {
        match packet.topic.split("/").collect_vec().as_slice() {
            ["battleship", "register"] => {
                let payload = match String::from_utf8(packet.payload.to_vec()) {
                    Ok(payload) => payload,
                    Err(err) => {
                        eprintln!("broken payload: {err}");
                        return;
                    }
                };

                let register_payload: Register = match serde_json::from_str(&payload) {
                    Ok(parsed) => parsed,
                    Err(err) => {
                        eprintln!("wrong payload: {err}");
                        return;
                    }
                };

                self.handle_register(register_payload).await;
            }
            ["battleship", "game", game_id, role, "action"] => {
                let payload = match String::from_utf8(packet.payload.to_vec()) {
                    Ok(payload) => payload,
                    Err(err) => {
                        eprintln!("broken payload: {err}");
                        return;
                    }
                };

                let action_payload: Action = match serde_json::from_str(&payload) {
                    Ok(parsed) => parsed,
                    Err(err) => {
                        eprintln!("wrong payload: {err}");
                        return;
                    }
                };

                let game_id = match game_id.parse() {
                    Ok(parsed) => parsed,
                    Err(err) => {
                        eprintln!("wrong game_id: {err}");
                        return;
                    }
                };

                let role = match role.parse() {
                    Ok(parsed) => parsed,
                    Err(err) => {
                        eprintln!("wrong role: {err}");
                        return;
                    }
                };

                self.handle_action(game_id, role, action_payload).await;
            }
            topic => unreachable!("not subscribed to {}", topic.join("/")),
        }
    }

    pub async fn engine_loop(mut self) {
        loop {
            tokio::select! {
                res = self.eventloop.poll() => {
                    let packet = match res {
                        Ok(Event::Incoming(Packet::Publish(packet))) => {
                            packet
                        }
                        Err(err) => {
                            eprintln!("{err}");
                            continue;
                        }
                        _ => continue
                    };

                    self.handle_packet(packet).await;
                }
                _ = tokio::signal::ctrl_c() => {
                    println!("\n❗ CTRL+C detected! Initiating graceful shutdown...");

                    if let Err(e) = self.client.disconnect().await {
                        eprintln!("⚠️ Failed to disconnect cleanly: {}", e);
                    } else {
                        println!("🔌 Disconnected!");
                    }

                    break;
                }
            }
        }
    }
}
