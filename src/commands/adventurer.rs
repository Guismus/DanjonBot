use serde::{Deserialize, Serialize};
use serenity::{model::channel::Message, prelude::*};
use std::env;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::vec::Vec;

use danjon_bot::stats::{calc_stats, get_race_stats, IvStats, Stats};

#[derive(Serialize, Deserialize, Debug)]
struct Adventurers {
    adventurer: Vec<Adventurer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Adventurer {
    id: u8,
    pub name: String,
    pub race: Race,
    rank: char,
    pub level: u8,
    pub iv: IvStats,
    jobs: Jobs,
    energy: Energy,
    health: Health,
}

impl fmt::Display for Adventurer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.race {
            Race::Jiaodan => {
                let stats_human: Stats = calc_stats(
                    self.iv,
                    self.level,
                    get_race_stats("JiaodanHumain".to_string()),
                    None,
                );
                let stats_dragon: Stats = calc_stats(
                    self.iv,
                    self.level,
                    get_race_stats("JiaodanDragon".to_string()),
                    None,
                );
                write!(
                    f,
                    "```\nAventurier: {}\nRace: {}\nRank: {}\nLevel: {}\nStats Humain: {}\nStats Dragon: {}\nBlessures: {} ({})\nMétiers: {}\nEnergie physique: {}\n",
                    self.name, self.race, self.rank, self.level, stats_human, stats_dragon, self.health.description, self.health.state, self.jobs, self.energy.physical)?;
            }
            _ => {
                let stats: Stats = calc_stats(
                    self.iv,
                    self.level,
                    get_race_stats(format!("{}", self.race)),
                    None,
                );
                write!(
                f,
                "```\nAventurier: {}\nRace: {}\nRank: {}\nLevel: {}\nStats: {}\nBlessures: {} ({})\nMétiers: {}\nEnergie physique: {}\n",
                self.name, self.race, self.rank, self.level, stats, self.health.description, self.health.state, self.jobs, self.energy.physical)?;
            }
        }
        match self.energy.magical.len() {
            0 => write!(f, "```"),
            _ => {
                write!(f, "Energie magique:")?;
                for i in &self.energy.magical {
                    write!(f, "{}", i)?;
                }
                write!(f, "\n```")
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Jobs {
    alchimiste_pharmacien: u8,
    alchimiste_artificer: u8,
    chevalier: u8,
    archer: u8,
    combattant: u8,
    escarpe: u8,
    medecin: u8,
    dresseur: u8,
    chasseur: u8,
    agriculteur: u8,
    couturier: u8,
    historien: u8,
    forgeron: u8,
    cartographe: u8,
    cuisinier: u8,
    erudit: u8,
    musicien: u8,
    machiniste: u8,
    ingenieur: u8,
}

impl fmt::Display for Jobs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fields = vec![
            ("Alchimiste pharmacien", self.alchimiste_pharmacien),
            ("Alchimiste artificer", self.alchimiste_artificer),
            ("Chevalier", self.chevalier),
            ("Archer", self.archer),
            ("Combattant", self.combattant),
            ("Escarpe", self.escarpe),
            ("Medecin", self.medecin),
            ("Dresseur", self.dresseur),
            ("Chasseur", self.chasseur),
            ("Agriculteur", self.agriculteur),
            ("Couturier", self.couturier),
            ("Historien", self.historien),
            ("Forgeron", self.forgeron),
            ("Cartographe", self.cartographe),
            ("Cuisinier", self.cuisinier),
            ("Erudit", self.erudit),
            ("Musicien", self.musicien),
            ("Machiniste mécanicien", self.machiniste),
            ("Machiniste ingénieur", self.ingenieur),
        ];

        let fields: Vec<_> = fields
            .into_iter()
            .filter(|(_, count)| *count > 0)
            .map(|(name, count)| format!("{} {}", name, count))
            .collect();

        if fields.is_empty() {
            write!(f, "Aucun métier")
        } else {
            write!(f, "{}", fields.join(", "))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Energy {
    physical: Physical,
    magical: Vec<Magic>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Physical {
    actual_energy: u8,
    energy: u8,
}

impl fmt::Display for Physical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.actual_energy, self.energy)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Magic {
    name: String,
    actual_energy: u8,
    energy: u8,
}

impl fmt::Display for Magic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " {} {}/{}", self.name, self.actual_energy, self.energy)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Race {
    Jiaodan,
    JiaodanHumain,
    JiaodanDragon,
    Marwoeth,
    Demon,
    Elfe,
    Ange,
    FerosumPassif,
    FerosumExtreme,
    Horya,
    Humain,
    Gwisin,
    Stens,
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Race::Jiaodan => write!(f, "JiaodanHumain"),
            Race::Marwoeth => write!(f, "Marwoeth"),
            Race::Demon => write!(f, "Demon"),
            Race::Elfe => write!(f, "Elfe"),
            Race::Ange => write!(f, "Ange"),
            Race::FerosumPassif => write!(f, "Ferosum Passif"),
            Race::FerosumExtreme => write!(f, "Ferosum Extreme"),
            Race::Horya => write!(f, "Horya"),
            Race::Humain => write!(f, "Humain"),
            Race::Gwisin => write!(f, "Gwisin"),
            Race::Stens => write!(f, "Stens"),
            _ => write!(f, "None"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Health {
    state: HealthState,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
enum HealthState {
    Aucune,
    Important,
    DeathDoor,
    Mort,
}

impl fmt::Display for HealthState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HealthState::Aucune => write!(f, "Aucune"),
            HealthState::Important => write!(f, "Important"),
            HealthState::DeathDoor => write!(f, "Death door"),
            HealthState::Mort => write!(f, "Mort"),
        }
    }
}

fn get_adventurers() -> Adventurers {
    let path = env::var("ADVENTURER_JSON").expect("Error in the env variable");
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    let res: Adventurers = serde_json::from_reader(reader).unwrap();

    res
}

pub fn get_adventurer(name: String) -> Option<Adventurer> {
    let contents: Adventurers = get_adventurers();
    for i in contents.adventurer {
        if i.name == name {
            return Some(i);
        }
    }

    return None;
}

pub async fn read_adventurer_stat(ctx: Context, msg: Message) {
    //let list: Adventurers = get_adventurers();
    let contents: Adventurers = get_adventurers();
    let command: Vec<String> = msg
        .content
        .clone()
        .split_whitespace()
        .map(String::from)
        .collect();
    if command.len() < 2 {
        return;
    }
    for i in contents.adventurer {
        if i.name == command[1] {
            msg.channel_id.say(&ctx.http, i).await;
        }
    }
}

//pub async fn remove_adventurer(ctx: Context, msg: Message) {
//    //TODO
//}
//
//pub async fn add_adventurer(ctx: Context, msg: Message) {
//    //TODO
//}
//
//pub async fn edit_adventurer(ctx: Context, msg: Message) {
//    //TODO
//}
