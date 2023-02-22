use serde::{Deserialize, Serialize};
use serenity::{model::channel::Message, prelude::*};
use std::env;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::vec::Vec;

#[derive(Serialize, Deserialize, Debug)]
struct StatsRaces {
    data: vec<StatsRace>,
}

#[derive(Serialize, Deserialize, Debug)]
struct StatsRace {
    race: String,
    force: u8,
    resistance: u8,
    vitesse: u8,
    force_magique: u8,
    resistance_magique: u8,
}

pub fn get_race_stats(race: String) -> StatsRace {
    let path = env::var("STATS_RACE_JSON").expect("Error in the env variable");
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    let data: StatsRaces = serde_json::from_reader(reader).unwrap();
    let res = StatsRace {
        race: "Erreur",
        force: 0,
        resistance: 0,
        vitesse: 0,
        force_magique: 0,
        resistance_magique: 0,
    };

    for i in data.data {
        if i.race == race {
            res = i
        }
    }

    res
}
