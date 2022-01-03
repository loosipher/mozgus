use std::fs::File;
use std::io::{Read, Write, Error};


pub fn get_sheet(cname: &str) -> Result<String, Error> {
    let mut f = File::open(format!("{}.csv", cname))?;
    let mut retval = String::new();
    f.read_to_string(&mut retval)?;
    Ok(retval)
}
