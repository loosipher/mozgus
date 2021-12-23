use std::fs::File;
use std::io::{Read, Write, Error};


struct Row (Vec<String>);
pub struct Table (Vec<Row>);
impl Table {
    pub fn parse(csv: &str) -> Table {
        let mut t = Vec::new() as Vec<Row>;
        for row in csv.split("\n") {
            let mut r = Vec::new() as Vec<String>;
            for col in row.split(",") {
                r.push(col.to_string());
            }
            t.push(Row(r));
        }
        Table(t)
    }

    pub fn get_cell(cell_code: [char; 2]) -> String {unimplemented!()}
}

pub fn get_sheet(cname: &str) -> Result<String, Error> {
    let mut f = File::open(format!("{}.csv", cname))?;
    let mut retval = String::new();
    f.read_to_string(&mut retval)?;
    Ok(retval)
}
