// -------------------------------------
// #![allow(non_snake_case)]
// -------------------------------------

// use serde::Deserialize;
// use serde::Serialize;
// use serde_json::Map as SerdeJsonMap;
// use serde_json::Result as SerdeJsonResult;
// use serde_json::Value as SerdeJsonValue;
// use serde_json::from_reader;
// use serde_json::from_value;
// use std::fmt::Display;
// use std::fs::File;
// use std::io::BufReader;
// use std::path::PathBuf;

// #[derive(Serialize, Deserialize)]
// struct InMyaWithMargin {
//     inMYA: SerdeJsonMap<String, SerdeJsonValue>,
//     marginOfError: Option<SerdeJsonMap<String, SerdeJsonValue>>,
// }

// #[derive(Serialize, Deserialize)]
// struct Gts {
//     id: String,
//     color: String,
//     rank: SerdeJsonValue,
//     broader: Option<SerdeJsonValue>,
//     hasBeginning: Option<InMyaWithMargin>,
//     hasEnd: Option<InMyaWithMargin>,
//     narrower: Option<Vec<Gts>>,
// }

// impl Display for Gts {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{},{},{},{},{},{}{}",
//             match &self.broader {
//                 Some(broader_opt) => {
//                     match broader_opt.as_array() {
//                         Some(broader) => broader[0]
//                             .to_string()
//                             .replace("\"", "")
//                             .replace("ischart:", "")
//                             .replace("Series", " Series")
//                             .replace("Stage", " Stage")
//                             .replace("Upper", "Upper ")
//                             .replace("Middle", "Middle ")
//                             .replace("Lower", "Lower "),
//                         None => "".to_string(),
//                     }
//                 }
//                 None => "".to_string(),
//             },
//             self.id
//                 .replace("ischart:", "")
//                 .replace("Series", " Series")
//                 .replace("Stage", " Stage")
//                 .replace("Upper", "Upper ")
//                 .replace("Middle", "Middle ")
//                 .replace("Lower", "Lower "),
//             match self.rank.as_str() {
//                 Some(rank) => rank.replace(
//                     "http://resource.geosciml.org/ontology/timescale/rank/", ""
//                 ),
//                 None => {
//                     if self.id.replace("ischart:", "") == "Pridoli" {
//                         "Epoch".to_string()
//                     } else {
//                         "".to_string()
//                     }
//                 }
//             },
//             self.color,
//             match &self.hasBeginning {
//                 Some(beg) => {
//                     let time =
//                         beg.inMYA["@value"].to_string().replace("\"", "");

//                     let margin = match &beg.marginOfError {
//                         Some(m) => m["@value"].to_string().replace("\"", ""),
//                         None => "".to_string(),
//                     };

//                     format!("{time},{margin}")
//                 }
//                 None => ",".to_string(),
//             },
//             match &self.hasEnd {
//                 Some(end) => {
//                     let time =
//                         end.inMYA["@value"].to_string().replace("\"", "");

//                     let margin = match &end.marginOfError {
//                         Some(m) => m["@value"].to_string().replace("\"", ""),
//                         None => "".to_string(),
//                     };

//                     format!("{time},{margin}")
//                 }
//                 None => ",".to_string(),
//             },
//             match &self.narrower {
//                 Some(children) => {
//                     let mut children_string = String::from("\n");
//                     children
//                         .iter()
//                         .for_each(|c| children_string += &format!("{c}"));
//                     children_string
//                 }
//                 None => "\n".to_string(),
//             },
//         )
//     }
// }

fn main() {
    // let path_buf = PathBuf::from("tests/data/gts.json");
    // let file_result = File::open(path_buf);

    // match file_result {
    //     Ok(file) => {
    //         let reader = BufReader::new(file);
    //         let json_result: SerdeJsonResult<SerdeJsonValue> =
    //             from_reader(reader);

    //         match json_result {
    //             Ok(json) => {
    //                 let tmp = json["hasTopConcept"].clone();
    //                 let typed_result: SerdeJsonResult<Vec<Gts>> =
    //                     from_value(tmp);

    //                 match typed_result {
    //                     Ok(typed) => print!("{}", typed[0]),
    //                     Err(err) => println!("{err}"),
    //                 }
    //             }
    //             Err(err) => println!("{err}"),
    //         }
    //     }
    //     Err(_) => (),
    // }
}
