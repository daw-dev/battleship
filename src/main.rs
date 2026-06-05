use std::time::Duration;
use rumqttc::{ClientError, MqttOptions};
use crate::mqtt::engine::Engine;

mod game;
mod mqtt;

// TOPICS for MQTT
// battleship/register -> ESP32 asks for a game
// battleship/{MAC}/assign -> Pie sends the game id
// battleship/game/{GAMEID}/state -> Pie sends events to both players (e.g. Turn: HOST)
// battleship/game/{GAMEID}/host/action -> ESP32 HOST sends messages to Pie (e.g. Shoot: 4,7)
// battleship/game/{GAMEID}/host/event -> Pie sends events to ESP32 HOST (e.g. Hit)

#[tokio::main]
async fn main() -> Result<(), ClientError> {
    let mut mqtt_options = MqttOptions::new("battleship", "localhost", 1883);
    mqtt_options.set_keep_alive(Duration::from_secs(5));

    println!("🌍 Instantiating client...");

    let engine = Engine::new(mqtt_options, 10);


    println!("✉️ Subscribing to topics...");

    engine.subscribe().await?;

    println!("⛴️ Engine running!");

    engine.engine_loop().await;

    println!("👋 Engine stopped running.");

    Ok(())
}
