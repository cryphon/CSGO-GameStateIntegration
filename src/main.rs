use std::{io::{self, Write}, fs, path::Path};
use fs::File;
use winreg::{RegKey, enums::*};
mod http_service;


extern crate winreg;

fn main() -> io::Result<()> {
    let _create_res = verify_cfg_presence();
    http_service::init_server();
    Ok(())
}

//present -> ignore || !present -> create
fn verify_cfg_presence()-> io::Result<()>{
    let reg = RegKey::predef(HKEY_CURRENT_USER);
    let crd = reg.open_subkey("SOFTWARE\\Valve\\Steam")?;
    let path: String = crd.get_value("SteamPath")?;
    let steam_cfg_folder: String = path + "/SteamApps/common/Counter-Strike Global Offensive/csgo/cfg/";

    let file_name = "gamestate_integration_csCrypt.cfg";
    let full_path: &str = &(steam_cfg_folder.to_owned() + file_name);
    
    let rs: bool = Path::new(&full_path).exists();
    if rs{
        //println!("File '{}' already exists", file_name);
        return Ok(())
    }
    let _gen_res = gen_cfg(full_path);
    Ok(())
}

fn gen_cfg(full_path: &str) -> io::Result<()>{
    //verify class location

    let mut file = File::create(full_path)?;
    let contents = "New content";
    file.write(contents.as_bytes())?;
    Ok(())
}

fn read_cfg(cfg_file: &str) -> io::Result<()>{
    let data = fs::read_to_string(cfg_file).expect("Unable to read file");
    println!("{}", data);
    Ok(())
}