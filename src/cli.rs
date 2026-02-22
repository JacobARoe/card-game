use bevy::prelude::*;
use std::sync::mpsc::{Receiver, channel};
use std::thread;

// --- CLi Events ---
#[derive(Event, Debug, Clone)]
pub struct PlayCardRequest {
    pub hand_index: usize,
    pub target_index: Option<usize>,
}

#[derive(Event, Debug, Clone)]
pub struct EndTurnRequest;

#[derive(Event, Debug, Clone)]
pub struct TriggerReflexRequest;

#[derive(Event, Debug, Clone)]
pub struct MapNodeSelectRequest {
    pub index: usize, // Usually only 1 or 2 options
}

// --- CLI Resource ---
#[derive(Resource)]
pub struct CliReceiver {
    pub rx: std::sync::Mutex<Receiver<String>>,
}

pub struct CliPlugin;

impl Plugin for CliPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = channel::<String>();

        // Spawn a background thread to read from stdin continuously without blocking Bevy
        thread::spawn(move || {
            let stdin = std::io::stdin();
            loop {
                let mut line = String::new();
                if stdin.read_line(&mut line).is_ok() {
                    let trimmed = line.trim().to_string();
                    if !trimmed.is_empty() {
                        if tx.send(trimmed).is_err() {
                            break; // Receiver dropped, exit thread
                        }
                    }
                }
            }
        });

        app.insert_resource(CliReceiver {
            rx: std::sync::Mutex::new(rx),
        })
        .add_event::<PlayCardRequest>()
        .add_event::<EndTurnRequest>()
        .add_event::<TriggerReflexRequest>()
        .add_event::<MapNodeSelectRequest>()
        .add_systems(Update, cli_poll_system);
    }
}

// This system consumes strings from the background thread and converts them into Bevy Events
pub fn cli_poll_system(
    receiver: Res<CliReceiver>,
    mut ev_play: EventWriter<PlayCardRequest>,
    mut ev_end: EventWriter<EndTurnRequest>,
    mut ev_reflex: EventWriter<TriggerReflexRequest>,
    mut ev_map: EventWriter<MapNodeSelectRequest>,
) {
    if let Ok(rx) = receiver.rx.lock() {
        while let Ok(cmd_str) = rx.try_recv() {
            let parts: Vec<&str> = cmd_str.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let cmd = parts[0].to_lowercase();

            match cmd.as_str() {
                "help" => {
                    println!("--- CLI Commands ---");
                    println!("play <hand_index> [target_index] : Play a card");
                    println!("end                              : End your turn");
                    println!("reflex                           : Trigger a reflex (spacebar)");
                    println!("map <node_index>                 : Select a map node");
                }
                "play" => {
                    if parts.len() >= 2 {
                        if let Ok(hand_index) = parts[1].parse::<usize>() {
                            let target_index = if parts.len() >= 3 {
                                parts[2].parse::<usize>().ok()
                            } else {
                                None
                            };
                            ev_play.send(PlayCardRequest {
                                hand_index,
                                target_index,
                            });
                            println!(
                                ">> Action: Play Card {} on Target {:?}",
                                hand_index, target_index
                            );
                        } else {
                            println!(">> Error: Invalid hand index.");
                        }
                    } else {
                        println!(">> Usage: play <hand_index> [target_index]");
                    }
                }
                "end" => {
                    ev_end.send(EndTurnRequest);
                    println!(">> Action: End Turn");
                }
                "reflex" => {
                    ev_reflex.send(TriggerReflexRequest);
                    println!(">> Action: Trigger Reflex");
                }
                "map" => {
                    if parts.len() >= 2 {
                        if let Ok(index) = parts[1].parse::<usize>() {
                            ev_map.send(MapNodeSelectRequest { index });
                            println!(">> Action: Select Map Node {}", index);
                        } else {
                            println!(">> Error: Invalid node index.");
                        }
                    } else {
                        println!(">> Usage: map <node_index>");
                    }
                }
                _ => {
                    println!(">> Unknown command: '{}'. Type 'help' for options.", cmd);
                }
            }
        }
    }
}
