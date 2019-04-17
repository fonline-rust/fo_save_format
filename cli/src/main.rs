use fo_client_format::ClientSaveData;
use ron::ser::{to_string_pretty, PrettyConfig};
use std::path::{Path, PathBuf};

mod json_pretty;

fn main() -> std::io::Result<()> {
    let mut args = std::env::args();
    let path = args.nth(1).expect("Pass file!");
    let path = Path::new(&path).canonicalize()?;

    if !path.exists() && !path.is_file() {
        panic!("File doesn't exist!");
    }

    let ext = path.extension();
    let file_name = path.file_name().unwrap();
    match ext {
        Some(ext) if ext == "client" => {
            let mut file = std::fs::File::open(&path)?;
            let client = ClientSaveData::read_unsafe(&mut file)?;

            let pretty = PrettyConfig {
                depth_limit: 3,
                separate_tuple_members: false,
                enumerate_arrays: false,
                ..PrettyConfig::default()
            };
            let ron = to_string_pretty(&client, pretty).expect("Serialization failed");
            let mut new_path = PathBuf::new();
            new_path.set_file_name(file_name);
            new_path.set_extension("ron");
            std::fs::write(&new_path, &ron)?;
            //let ron = to_string_pretty(&client, pretty).expect("Serialization failed");

            let mut json = Vec::with_capacity(32 * 1024);
            let formatter = json_pretty::PrettyFormatter::new();
            let mut serializer = serde_json::ser::Serializer::with_formatter(&mut json, formatter);
            //let data = serde_json::to_string(&client).expect("Can't serialize into Json!");
            use serde::ser::Serialize;
            client
                .serialize(&mut serializer)
                .expect("Can't serialize into Json!");
            let mut new_path = PathBuf::new();
            new_path.set_file_name(file_name);
            new_path.set_extension("json");
            std::fs::write(&new_path, &json)?;
        }
        Some(ext) if ext == "ron" => {
            let mut file = std::fs::File::open(&path)?;
            let client: ClientSaveData =
                ron::de::from_reader(&mut file).expect("Can't parse file!");
            let data = client.write();
            let mut new_path = PathBuf::new();
            new_path.set_file_name(file_name);
            new_path.set_extension("client");
            std::fs::write(&new_path, &data)?;
        }
        Some(ext) if ext == "json" => {
            let mut file = std::fs::File::open(&path)?;
            let client: ClientSaveData =
                serde_json::from_reader(&mut file).expect("Can't parse file!");
            let data = client.write();
            let mut new_path = PathBuf::new();
            new_path.set_file_name(file_name);
            new_path.set_extension("client");
            std::fs::write(&new_path, &data)?;
        }
        _ => panic!("Pass either '.client', '.json' or '.ron'!"),
    }

    //let json = serde_json::to_string_pretty(&client).expect("Can't serialize into Json!");
    //println!("{}", json);

    //
    //println!("{}", ron);
    //let output = client.write();
    Ok(())
}
