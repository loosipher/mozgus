use std::fs::File;
use std::io::{Read, Write, Error};


pub struct Cfg {
    pub token: String,
    pub adjust_rolls: bool
}


fn load_cfg() -> Result<Cfg, Error> {
    let f = File::open("config");
    if let Err(_why) = f {
        let data_string = "TOKEN\nfalse";
        let mut n = File::create("config")?;
        n.write_all(data_string.as_bytes())?;
        Ok(Cfg {
            token: "TOKEN".to_string(),
            adjust_rolls: false
        })
    } else {
        let mut data_string = String::new();
        let mut new_f = f?;
        new_f.read_to_string(&mut data_string)?;
        let data = data_string.split("\n").collect::<Vec<&str>>();
        let ar = data[1] == "true";
        let token = data[0].to_string();
        Ok(
            Cfg {
                token: token,
                adjust_rolls: ar
            }
        )
    }
}


lazy_static! {
    pub static ref CFG: Cfg = load_cfg().unwrap();
}
