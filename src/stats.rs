use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::vec::Vec;
use std::{env, fmt};

#[derive(Serialize, Deserialize, Debug)]
struct StatsRaces {
    data: Vec<StatsRace>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatsRace {
    pub race: String,
    pub force: u8,
    pub resistance: u8,
    pub vitesse: u8,
    pub force_magique: u8,
    pub resistance_magique: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct IvStats {
    pub force: u8,
    pub resistance: u8,
    pub vitesse: u8,
    pub resistance_magique: u8,
    pub force_magique: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub force: f32,
    pub resistance: f32,
    pub vitesse: f32,
    pub force_magique: f32,
    pub resistance_magique: f32,
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:.2} for, {:.2} res, {:.2} vit, {:.2} f.mag, {:.2} r.mag",
            self.force, self.resistance, self.vitesse, self.force_magique, self.resistance_magique
        )
    }
}

pub fn calc_stats(
    iv: IvStats,
    level: u8,
    stats_race: StatsRace,
    stats_mod: Option<Stats>,
) -> Stats {
    let stats_mod = stats_mod.unwrap_or(Stats {
        force: 0.,
        resistance: 0.,
        vitesse: 0.,
        force_magique: 0.,
        resistance_magique: 0.,
    });
    let mut res = Stats {
        force: (((2. * (stats_mod.force + stats_race.force as f32) + iv.force as f32)
            * (level as f32 + 2.))
            / 150.
            + 5.),
        resistance: ((2. * (stats_mod.force + stats_race.resistance as f32)
            + iv.resistance as f32)
            * (level as f32 + 2.))
            / 150.
            + 5.,
        vitesse: ((2. * (stats_mod.force + stats_race.vitesse as f32) + iv.vitesse as f32)
            * (level as f32 + 2.))
            / 150.
            + 5.,
        force_magique: ((2. * (stats_mod.force + stats_race.force_magique as f32)
            + iv.force_magique as f32)
            * (level as f32 + 2.))
            / 150.
            + 5.,
        resistance_magique: ((2. * (stats_mod.force + stats_race.resistance_magique as f32)
            + iv.resistance_magique as f32)
            * (level as f32 + 2.))
            / 150.
            + 5.,
    };

    res.force = res.force + (0.25 - res.force % 0.25);
    res.resistance = res.resistance + (0.25 - res.resistance % 0.25);
    res.vitesse = res.vitesse + (0.25 - res.vitesse % 0.25);
    res.force_magique = res.force_magique + (0.25 - res.force_magique % 0.25);
    res.resistance_magique = res.resistance_magique + (0.25 - res.resistance_magique % 0.25);

    res
}

pub fn get_race_stats(race: String) -> StatsRace {
    let path = env::var("STATS_RACE_JSON").expect("Error in the env variable");
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    let data: StatsRaces = serde_json::from_reader(reader).unwrap();
    let mut res = StatsRace {
        race: "Erreur".to_string(),
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
