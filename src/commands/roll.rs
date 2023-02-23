use rand::thread_rng;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serenity::{model::channel::Message, prelude::*};
use std::env;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::vec::Vec;

use danjon_bot::stats::{calc_stats, get_race_stats};

use crate::commands::adventurer::{get_adventurer, Adventurer};

enum DiffStatsState {
    SousDomination,
    Souspuissance,
    SousEfficace,
    SousAvantage,
    SousFaveur,
    Neutre,
    Faveur,
    Avantage,
    Efficace,
    Surpuissance,
    Domination,
}

impl fmt::Display for DiffStatsState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiffStatsState::SousDomination => write!(f, "une sous-domination"),
            DiffStatsState::Souspuissance => write!(f, "une sous-puissance"),
            DiffStatsState::SousEfficace => write!(f, "un sous-efficace"),
            DiffStatsState::SousAvantage => write!(f, "un sous-avantage"),
            DiffStatsState::SousFaveur => write!(f, "une sous-faveur"),
            DiffStatsState::Neutre => write!(f, "un neutre"),
            DiffStatsState::Faveur => write!(f, "une faveur"),
            DiffStatsState::Avantage => write!(f, "un avantage"),
            DiffStatsState::Efficace => write!(f, "un efficace"),
            DiffStatsState::Surpuissance => write!(f, "une surpuissance"),
            DiffStatsState::Domination => write!(f, "une domination"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct DiffStats {
    faveur: f32,
    avantage: f32,
    efficace: f32,
    surpuissance: f32,
    domination: f32,
}

struct AttackResult {
    gagnant: String,
    perdant: String,
    diff_vitesse: DiffStatsState,
    diff_force: DiffStatsState,
    usure: f32,
}

impl fmt::Display for AttackResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "```{} touche {} avec {} en vitesse, son attaque causant {}, qui perd {} de durabilité suite au coup infligé si il a fait usage d'une arme```",
            self.gagnant, self.perdant, self.diff_vitesse, self.diff_force, self.usure
        )
    }
}

#[derive(Debug, Clone)]
struct Entities {
    entity_one: Entity,
    entity_second: Entity,
}

impl Default for Entities {
    fn default() -> Self {
        Entities {
            entity_one: Default::default(),
            entity_second: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
struct Entity {
    name: String,
    level: u8,
    force: f32,
    resistance: f32,
    vitesse: f32,
    resistance_magique: f32,
    force_magique: f32,
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            name: "entity".to_string(),
            level: 1,
            force: 5.0,
            resistance: 5.0,
            vitesse: 5.0,
            resistance_magique: 5.0,
            force_magique: 5.0,
        }
    }
}

fn set_entity_stats(name: String) -> Entity {
    let adventurer: Adventurer = get_adventurer(name).unwrap();
    let stats = calc_stats(
        adventurer.iv,
        adventurer.level,
        get_race_stats(format!("{}", adventurer.race)),
        None,
    );
    let result: Entity = Entity {
        name: adventurer.name,
        level: adventurer.level,
        force: stats.force,
        resistance: stats.resistance,
        vitesse: stats.vitesse,
        resistance_magique: stats.resistance_magique,
        force_magique: stats.force_magique,
        ..Default::default()
    };

    result
}

fn search_entities(command: Vec<String>) -> Entities {
    let mut result: Entities = Entities::default();
    let mut nb = 1;

    result.entity_one = set_entity_stats(command[nb].clone());
    println!("{:?}", result.entity_one);
    for (i, j) in command.clone().iter().enumerate() {
        if i > nb {
            if get_adventurer(j.to_string()).is_some() {
                nb = i;
                break;
            }
            match j.as_str() {
                "-weapon" => match command[i + 1].as_str() {
                    "Leger" => {
                        result.entity_one.force *= 0.9; //res.force + (0.25 - res.force % 0.25)
                        result.entity_one.force += 0.25 - result.entity_one.force % 0.25;
                        result.entity_one.vitesse *= 1.05;
                    }
                    "Moyen" => {}
                    "Lourd" => {
                        result.entity_one.force *= 1.1;
                        result.entity_one.vitesse *= 0.9;
                    }
                    _ => {
                        result.entity_one.force *= 0.85;
                        result.entity_one.vitesse *= 1.075;
                    }
                },
                _ => {}
            }
        }
    }
    result.entity_second = set_entity_stats(command[nb].clone());
    println!("{:?}", result.entity_second);
    for (i, j) in command.clone().iter().enumerate() {
        if i > nb {
            if get_adventurer(j.to_string()).is_some() {
                break;
            }
            match j.as_str() {
                "-weapon" => match command[i + 1].as_str() {
                    "Leger" => {
                        result.entity_second.force *= 0.9;
                        result.entity_second.vitesse *= 1.05;
                    }
                    "Moyen" => {}
                    "Lourd" => {
                        result.entity_second.force *= 1.1;
                        result.entity_second.vitesse *= 0.9;
                    }
                    _ => {
                        result.entity_second.force *= 0.85;
                        result.entity_second.vitesse *= 1.075;
                    }
                },
                _ => {}
            }
        }
    }
    result.entity_one.force += 0.25 - result.entity_one.force % 0.25;
    result.entity_one.vitesse += 0.25 - result.entity_one.vitesse % 0.25;
    result.entity_second.force += 0.25 - result.entity_second.force % 0.25;
    result.entity_second.vitesse += 0.25 - result.entity_second.vitesse % 0.25;
    println!("{:?}", result.entity_one);
    println!("{:?}", result.entity_second);

    result
}

fn result_roll_vitesse(entities: Entities, mut result: AttackResult) -> AttackResult {
    let path = env::var("DIFF_STATS").expect("Error in the env variable");
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    let mut data: DiffStats = serde_json::from_reader(reader).unwrap();
    let mut roll = thread_rng();
    match entities.entity_one.vitesse {
        x if x > entities.entity_second.vitesse => {
            data.faveur *= entities.entity_second.level as f32;
            data.avantage *= entities.entity_second.level as f32;
            data.efficace *= entities.entity_second.level as f32;
            data.surpuissance *= entities.entity_second.level as f32;
            data.domination *= entities.entity_second.level as f32;
            match entities.entity_one.vitesse - entities.entity_second.vitesse {
                x if x < data.faveur => match roll.gen_range(1..=2) {
                    2 => {
                        result.gagnant = entities.entity_second.name.clone();
                        result.diff_vitesse = DiffStatsState::Neutre
                    }
                    _ => {
                        result.gagnant = entities.entity_one.name.clone();
                        result.diff_vitesse = DiffStatsState::Neutre
                    }
                },
                x if x < data.avantage => match roll.gen_range(1..=3) {
                    3 => {
                        result.gagnant = entities.entity_second.name.clone();
                        result.diff_vitesse = DiffStatsState::SousFaveur
                    }
                    _ => {
                        result.gagnant = entities.entity_one.name.clone();
                        result.diff_vitesse = DiffStatsState::Faveur
                    }
                },
                x if x < data.efficace => match roll.gen_range(1..=4) {
                    4 => {
                        result.gagnant = entities.entity_second.name.clone();
                        result.diff_vitesse = DiffStatsState::SousAvantage
                    }
                    _ => {
                        result.gagnant = entities.entity_one.name.clone();
                        result.diff_vitesse = DiffStatsState::Avantage
                    }
                },
                x if x < data.surpuissance => match roll.gen_range(1..=5) {
                    5 => {
                        result.gagnant = entities.entity_second.name.clone();
                        result.diff_vitesse = DiffStatsState::SousEfficace
                    }
                    _ => {
                        result.gagnant = entities.entity_one.name.clone();
                        result.diff_vitesse = DiffStatsState::Efficace
                    }
                },
                x if x < data.domination => match roll.gen_range(1..=6) {
                    6 => {
                        result.gagnant = entities.entity_second.name.clone();
                        result.diff_vitesse = DiffStatsState::Souspuissance
                    }
                    _ => {
                        result.gagnant = entities.entity_one.name.clone();
                        result.diff_vitesse = DiffStatsState::Surpuissance
                    }
                },
                x if x > data.domination => {
                    result.gagnant = entities.entity_one.name.clone();
                    result.diff_vitesse = DiffStatsState::Domination
                }
                _ => result.gagnant = entities.entity_one.name.clone(),
            }
        }
        _ => {
            data.faveur *= entities.entity_one.level as f32;
            data.avantage *= entities.entity_one.level as f32;
            data.efficace *= entities.entity_one.level as f32;
            data.surpuissance *= entities.entity_one.level as f32;
            data.domination *= entities.entity_one.level as f32;
            match entities.entity_second.vitesse - entities.entity_one.vitesse {
                x if x < data.faveur => match roll.gen_range(1..=2) {
                    2 => {
                        result.gagnant = entities.entity_one.name.clone();
                        result.diff_vitesse = DiffStatsState::Neutre
                    }
                    _ => {
                        result.gagnant = entities.entity_second.name.clone();
                        result.diff_vitesse = DiffStatsState::Neutre
                    }
                },
                x if x < data.avantage => match roll.gen_range(1..=3) {
                    3 => {
                        result.gagnant = entities.entity_one.name.clone();
                        result.diff_vitesse = DiffStatsState::SousFaveur
                    }
                    _ => {
                        result.gagnant = entities.entity_second.name.clone();
                        result.diff_vitesse = DiffStatsState::Faveur
                    }
                },
                x if x < data.efficace => match roll.gen_range(1..=4) {
                    4 => {
                        result.gagnant = entities.entity_one.name.clone();
                        result.diff_vitesse = DiffStatsState::SousAvantage
                    }
                    _ => {
                        result.gagnant = entities.entity_second.name.clone();
                        result.diff_vitesse = DiffStatsState::Avantage
                    }
                },
                x if x < data.surpuissance => match roll.gen_range(1..=5) {
                    5 => {
                        result.gagnant = entities.entity_one.name.clone();
                        result.diff_vitesse = DiffStatsState::SousEfficace
                    }
                    _ => {
                        result.gagnant = entities.entity_second.name.clone();
                        result.diff_vitesse = DiffStatsState::Efficace
                    }
                },
                x if x < data.domination => match roll.gen_range(1..=6) {
                    6 => {
                        result.gagnant = entities.entity_one.name.clone();
                        result.diff_vitesse = DiffStatsState::Souspuissance
                    }
                    _ => {
                        result.gagnant = entities.entity_second.name.clone();
                        result.diff_vitesse = DiffStatsState::Surpuissance
                    }
                },
                x if x > data.domination => {
                    result.gagnant = entities.entity_second.name.clone();
                    result.diff_vitesse = DiffStatsState::Domination
                }
                _ => result.gagnant = entities.entity_second.name.clone(),
            }
        }
    }

    if result.gagnant == entities.entity_one.name {
        result.perdant = entities.entity_second.name;
    } else {
        result.perdant = entities.entity_one.name;
    }

    result
}

fn result_roll_attack(entities: Entities) -> AttackResult {
    let mut result: AttackResult = AttackResult {
        gagnant: "Aucun".to_string(),
        perdant: "Aucun".to_string(),
        diff_vitesse: DiffStatsState::Neutre,
        diff_force: DiffStatsState::Neutre,
        usure: 0.,
    };
    result = result_roll_vitesse(entities.clone(), result);

    let path = env::var("DIFF_STATS").expect("Error in the env variable");
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    let data: DiffStats = serde_json::from_reader(reader).unwrap();

    if result.gagnant == entities.entity_one.name {
        match entities.entity_one.force - entities.entity_second.resistance {
            x if x <= 0.0 => match x * -1. {
                x if x < data.faveur * entities.entity_one.level as f32 => {
                    result.diff_force = DiffStatsState::Neutre;
                    result.usure = 3.;
                }
                x if x < data.avantage * entities.entity_one.level as f32 => {
                    result.diff_force = DiffStatsState::SousFaveur;
                    result.usure = 4.;
                }
                x if x < data.efficace * entities.entity_one.level as f32 => {
                    result.diff_force = DiffStatsState::SousAvantage;
                    result.usure = 5.;
                }
                x if x < data.surpuissance * entities.entity_one.level as f32 => {
                    result.diff_force = DiffStatsState::SousEfficace;
                    result.usure = 6.;
                }
                x if x < data.domination * entities.entity_one.level as f32 => {
                    result.diff_force = DiffStatsState::Souspuissance;
                    result.usure = 7.;
                }
                _ => {
                    result.diff_force = DiffStatsState::SousDomination;
                    result.usure = 8.;
                }
            },
            x if x < data.faveur * entities.entity_second.level as f32 => {
                result.diff_force = DiffStatsState::Neutre;
                result.usure = 3.;
            }
            x if x < data.avantage * entities.entity_second.level as f32 => {
                result.diff_force = DiffStatsState::Faveur;
                result.usure = 2.;
            }
            x if x < data.efficace * entities.entity_second.level as f32 => {
                result.diff_force = DiffStatsState::Avantage;
                result.usure = 1.5;
            }
            x if x < data.surpuissance * entities.entity_second.level as f32 => {
                result.diff_force = DiffStatsState::Efficace;
                result.usure = 1.;
            }
            x if x < data.domination * entities.entity_second.level as f32 => {
                result.diff_force = DiffStatsState::Surpuissance;
                result.usure = 0.5;
            }
            _ => {
                result.diff_force = DiffStatsState::Domination;
                result.usure = 0.;
            }
        }
    } else {
        match entities.entity_second.force - entities.entity_one.resistance {
            x if x <= 0.0 => match x * -1. {
                x if x < data.faveur * entities.entity_second.level as f32 => {
                    result.diff_force = DiffStatsState::Neutre;
                    result.usure = 3.;
                }
                x if x < data.avantage * entities.entity_second.level as f32 => {
                    result.diff_force = DiffStatsState::SousFaveur;
                    result.usure = 4.;
                }
                x if x < data.efficace * entities.entity_second.level as f32 => {
                    result.diff_force = DiffStatsState::SousAvantage;
                    result.usure = 5.;
                }
                x if x < data.surpuissance * entities.entity_second.level as f32 => {
                    result.diff_force = DiffStatsState::SousEfficace;
                    result.usure = 6.;
                }
                x if x < data.domination * entities.entity_second.level as f32 => {
                    result.diff_force = DiffStatsState::Souspuissance;
                    result.usure = 7.;
                }
                _ => {
                    result.diff_force = DiffStatsState::SousDomination;
                    result.usure = 8.;
                }
            },
            x if x < data.faveur * entities.entity_one.level as f32 => {
                result.diff_force = DiffStatsState::Neutre;
                result.usure = 3.;
            }
            x if x < data.avantage * entities.entity_one.level as f32 => {
                result.diff_force = DiffStatsState::Faveur;
                result.usure = 2.;
            }
            x if x < data.efficace * entities.entity_one.level as f32 => {
                result.diff_force = DiffStatsState::Avantage;
                result.usure = 1.5;
            }
            x if x < data.surpuissance * entities.entity_one.level as f32 => {
                result.diff_force = DiffStatsState::Efficace;
                result.usure = 1.;
            }
            x if x < data.domination * entities.entity_one.level as f32 => {
                result.diff_force = DiffStatsState::Surpuissance;
                result.usure = 0.5;
            }
            _ => {
                result.diff_force = DiffStatsState::Domination;
                result.usure = 0.;
            }
        }
    }

    result
}

pub async fn attack_roll(ctx: Context, msg: Message) {
    let command: Vec<String> = msg
        .content
        .clone()
        .split_whitespace()
        .map(String::from)
        .collect();
    let attack_result: AttackResult = result_roll_attack(search_entities(command));
    println!("Here");
    msg.channel_id.say(&ctx.http, attack_result).await;
}
