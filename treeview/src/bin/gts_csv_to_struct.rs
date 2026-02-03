// -------------------------------------
// #![allow(unused)]
// -------------------------------------

// use riced::Color;
// use serde::Deserialize;
// use serde::Deserializer;
// use serde::de;
// use std::{error::Error, process, str::FromStr};
// use treeview::Float;

fn main() {
    // if let Err(err) = run() {
    //     println!("Error: {}", err);
    //     process::exit(1);
    // }
}

// fn run() -> Result<(), Box<dyn Error>> {
//     let mut rdr = csv::Reader::from_path("tests/data/gts_from_json.csv")?;
//     let mut recs: Vec<GtsRecord> = Vec::new();

//     for result in rdr.deserialize() {
//         let r: GtsRecord = result?;
//         recs.push(r);
//     }

//     recs.sort_by(|a, b| a.end.total_cmp(&b.end));
//     recs.sort_by(|a, b| a.beg.total_cmp(&b.beg));

//     println!("[");
//     recs.iter().for_each(|r| {
//         println!(
//             "GtsRecord {{
//         broader: {},
//         name: String::from(\"{}\"),
//         rank: GtsRank::{:?},
//         beg: {:.4},
//         end: {:.4},
//         beg_margin_of_error: {:?},
//         end_margin_of_error: {:?},
//         color: {:?},
// }},",
//             if let Some(broader) = &r.broader {
//                 format!("Some(String::from(\"{broader}\"))")
//             } else {
//                 "None".to_string()
//             },
//             r.name,
//             r.rank,
//             r.beg.max(r.end),
//             r.beg.min(r.end),
//             r.beg_margin_of_error,
//             r.end_margin_of_error,
//             r.color,
//         );
//     });
//     println!("];");
//     Ok(())
// }

// #[derive(Debug, serde::Deserialize)]
// enum GtsRank {
//     Eon,
//     Era,
//     Period,
//     Epoch,
//     #[serde(rename = "Sub-Period")]
//     SubPeriod,
//     Age,
//     Other,
// }

// #[derive(Debug, serde::Deserialize)]
// struct GtsRecord {
//     broader: Option<String>,
//     name: String,
//     rank: GtsRank,
//     beg: Float,
//     end: Float,
//     beg_margin_of_error: Option<Float>,
//     end_margin_of_error: Option<Float>,
//     #[serde(deserialize_with = "deserialize_hex_color")]
//     color: Color,
// }

// fn deserialize_hex_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let s = String::deserialize(deserializer)?;
//     match Color::parse(&s) {
//         Some(color) => Ok(color),
//         None => Err(de::Error::custom("Color parsing error")),
//     }
// }
