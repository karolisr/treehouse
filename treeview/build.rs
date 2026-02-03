use std::env::var_os;
use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::de;
use serde_json::Map as SerdeJsonMap;
use serde_json::Result as SerdeJsonResult;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_reader;
use serde_json::from_value;

use riced::Color;

type Float = f32;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed=../resources/data/gts.json");
    println!("cargo::rerun-if-changed=build.rs");

    let data_dir = Path::new("..").join("resources").join("data");
    let gts_json_file = data_dir.join("gts.json");
    let gts_csv_file = data_dir.join("gts.csv");

    write_gts_csv(&gts_json_file, &gts_csv_file)?;

    let code_out_dir = var_os("OUT_DIR").unwrap();
    let gts_rs_file = Path::new(&code_out_dir).join("gts.rs");

    write_gts_rs(&gts_csv_file, &gts_rs_file)?;

    Ok(())
}

fn write_gts_rs(
    gts_csv_file: &Path,
    gts_rs_file: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut lines: Vec<String> = Vec::new();

    let mut rdr = csv::Reader::from_path(gts_csv_file)?;
    let mut recs: Vec<GtsRecord> = Vec::new();

    for result in rdr.deserialize() {
        let r: GtsRecord = result?;
        recs.push(r);
    }

    recs.sort_by(|a, b| a.beg.total_cmp(&b.beg));
    recs.sort_by(|a, b| a.end.total_cmp(&b.end));

    lines.push(format!(
        "pub(crate) fn gts_data() -> [GtsRecord; {}] {{\n",
        recs.len()
    ));
    lines.push("    [".to_string());

    recs.iter().for_each(|r| {
        let line = format!(
            "\n        GtsRecord {{
            broader: {},
            name: String::from(\"{}\"),
            rank: GtsRank::{:?},
            end: {:.4},
            beg: {:.4},
            beg_margin_of_error: {:?},
            end_margin_of_error: {:?},
            color: {:?},
        }},",
            if let Some(broader) = &r.broader {
                format!("Some(String::from(\"{broader}\"))")
            } else {
                "None".to_string()
            },
            r.name,
            r.rank,
            r.end.min(r.beg),
            r.beg.max(r.end),
            r.beg_margin_of_error,
            r.end_margin_of_error,
            r.color,
        );
        lines.push(line);
    });

    lines.push("\n    ]\n}".to_string());

    fs::write(gts_rs_file, lines.concat())?;

    Ok(())
}

fn write_gts_csv(
    gts_json_file: &Path,
    gts_csv_file: &Path,
) -> Result<(), Box<dyn Error>> {
    let mut lines: Vec<String> = Vec::new();
    let line = "broader,name,rank,color,beg,beg_margin_of_error,end,end_margin_of_error\n".to_string();
    lines.push(line);

    let reader = BufReader::new(File::open(gts_json_file)?);
    let json_result: SerdeJsonResult<SerdeJsonValue> = from_reader(reader);

    match json_result {
        Ok(json) => {
            let tmp = json["hasTopConcept"].clone();
            let typed_result: SerdeJsonResult<Vec<Gts>> = from_value(tmp);

            match typed_result {
                Ok(typed) => {
                    let line = format!("{}", typed[0]);
                    lines.push(line);
                }

                Err(err) => return Err(Box::new(err)),
            }
        }
        Err(err) => return Err(Box::new(err)),
    }

    fs::write(gts_csv_file, lines.concat())?;

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct InMyaWithMargin {
    #[serde(rename = "inMYA")]
    in_mya: SerdeJsonMap<String, SerdeJsonValue>,
    #[serde(rename = "marginOfError")]
    margin_of_error: Option<SerdeJsonMap<String, SerdeJsonValue>>,
}

#[derive(Serialize, Deserialize)]
struct Gts {
    id: String,
    color: String,
    rank: SerdeJsonValue,
    broader: Option<SerdeJsonValue>,
    #[serde(rename = "hasBeginning")]
    has_beginning: Option<InMyaWithMargin>,
    #[serde(rename = "hasEnd")]
    has_end: Option<InMyaWithMargin>,
    narrower: Option<Vec<Gts>>,
}

impl Display for Gts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{}{}",
            match &self.broader {
                Some(broader_opt) => {
                    match broader_opt.as_array() {
                        Some(broader) => broader[0]
                            .to_string()
                            .replace("\"", "")
                            .replace("ischart:", "")
                            .replace("Series", " Series")
                            .replace("Stage", " Stage")
                            .replace("Upper", "Upper ")
                            .replace("Middle", "Middle ")
                            .replace("Lower", "Lower "),
                        None => "".to_string(),
                    }
                }
                None => "".to_string(),
            },
            self.id
                .replace("ischart:", "")
                .replace("Series", " Series")
                .replace("Stage", " Stage")
                .replace("Upper", "Upper ")
                .replace("Middle", "Middle ")
                .replace("Lower", "Lower "),
            match self.rank.as_str() {
                Some(rank) => rank.replace(
                    "http://resource.geosciml.org/ontology/timescale/rank/", ""
                ),
                None => {
                    if self.id.replace("ischart:", "") == "Pridoli" {
                        "Epoch".to_string()
                    } else {
                        "".to_string()
                    }
                }
            },
            self.color,
            match &self.has_beginning {
                Some(beg) => {
                    let time =
                        beg.in_mya["@value"].to_string().replace("\"", "");

                    let margin = match &beg.margin_of_error {
                        Some(m) => m["@value"].to_string().replace("\"", ""),
                        None => "".to_string(),
                    };

                    format!("{time},{margin}")
                }
                None => ",".to_string(),
            },
            match &self.has_end {
                Some(end) => {
                    let time =
                        end.in_mya["@value"].to_string().replace("\"", "");

                    let margin = match &end.margin_of_error {
                        Some(m) => m["@value"].to_string().replace("\"", ""),
                        None => "".to_string(),
                    };

                    format!("{time},{margin}")
                }
                None => ",".to_string(),
            },
            match &self.narrower {
                Some(children) => {
                    let mut children_string = String::from("\n");
                    children
                        .iter()
                        .for_each(|c| children_string += &format!("{c}"));
                    children_string
                }
                None => "\n".to_string(),
            },
        )
    }
}

#[derive(Debug, serde::Deserialize)]
enum GtsRank {
    Eon,
    Era,
    Period,
    Epoch,
    #[serde(rename = "Sub-Period")]
    SubPeriod,
    Age,
}

#[derive(Debug, serde::Deserialize)]
struct GtsRecord {
    broader: Option<String>,
    name: String,
    rank: GtsRank,
    beg: Float,
    end: Float,
    beg_margin_of_error: Option<Float>,
    end_margin_of_error: Option<Float>,
    #[serde(deserialize_with = "deserialize_hex_color")]
    color: Color,
}

fn deserialize_hex_color<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match Color::parse(&s) {
        Some(color) => Ok(color),
        None => Err(de::Error::custom("Color parsing error")),
    }
}
