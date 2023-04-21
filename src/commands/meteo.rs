use serenity::{model::channel::Message, prelude::*};
use std::env;
use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Meteos {
    palliers: Vec<Pallier>
}

#[derive(Serialize, Deserialize, Clone)]
struct Pallier {
    banner_url: String,
    temperies: Vec<Climat>,
    intemperies: Vec<Climat>
}

#[derive(Serialize, Deserialize, Clone)]
struct Climat {
    name: String,
    description: String,
    image_url: String,
    temperature: u8,
    humidite: u8,
    duree: u8,
    protection: u8
}

pub async fn meteo(ctx: Context, msg: Message) {
    let path = env::var("METEO_JSON").expect("Error in the env variable");
    let file = File::open(path);
    let reader = BufReader::new(file.unwrap());
    let res: Meteos = serde_json::from_reader(reader).unwrap();

    let msg = msg.channel_id.send_message(&ctx.http, |m| {
        m.add_file(res.palliers.last().unwrap().banner_url.as_str())
            .embed(|e| {
                e.title(res.palliers.last().unwrap().temperies.last().unwrap().name.clone())
                    .description("L'humidité dans l'air s'intensifie pour laisser paraître les couleurs multicolorées d'un ar-en-ciel! Le beau temps avec un peu de nébulosité et une température douce.")
                    .image("https://cdn.discordapp.com/attachments/890628271157411891/892428906542555136/arc-en-ciel.gif")
                    .fields(vec![
                        ("__Température__", "19°C", true),
                        ("__Humidité__", "40%", true),
                        ("\u{200B}", "\u{200B}", false),
                        ("__Durée__", "1 semaine", true),
                        ("__Protection contre les intempéries__", "O semaine", true)
                    ])
            })
    }).await;
    if let Err(why) = msg {println!("Error sending meteo embed: {:?}", why);}
}
