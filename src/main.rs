mod client_save_data;
use client_save_data::ClientSaveData;

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("../../FO4RP/save/clients/qthree.client")?;
    let client = ClientSaveData::read(&mut file)?;
    println!("Client: {:#?}", &client);
    Ok(())
}
