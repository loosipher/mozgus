use std::collections::HashMap;


pub struct Character {
    // vitals
    name: String,
    max_hp: u8,
    cur_hp: u8,
    max_san: u8,
    cur_san: u8,
    luck: u8,
    max_mp: u8,
    cur_mp: u8,

    // stats
    str: u8,
    dex: u8,
    int: u8,
    con: u8,
    app: u8,
    pow: u8,
    siz: u8,
    edu: u8,

    // inventory
    weapons: Vec<String>,

    skills: HashMap<String, u8>
}

impl Character {
    // TODO *ERROR HANDLING!!!*
    // perhaps find a less tedious way to do this; possibly a serialization library or database kinda thing
    pub fn build_character(table: crate::csreader::Table) -> Character {
        let name = table.get_cell("B1");
        let max_hp = u8::from_str_radix(table.get_cell("B9").as_str(), 10).unwrap();
        let cur_hp = u8::from_str_radix(table.get_cell("B10").as_str(), 10).unwrap();
        let max_san = u8::from_str_radix(table.get_cell("B13").as_str(), 10).unwrap();
        let cur_san = u8::from_str_radix(table.get_cell("B14").as_str(), 10).unwrap();
        let luck = u8::from_str_radix(table.get_cell("B16").as_str(), 10).unwrap();
        let max_mp = u8::from_str_radix(table.get_cell("B18").as_str(), 10).unwrap();
        let cur_mp = u8::from_str_radix(table.get_cell("B19").as_str(), 10).unwrap();

        let str = u8::from_str_radix(table.get_cell("D1").as_str(), 10).unwrap();
        let dex = u8::from_str_radix(table.get_cell("D5").as_str(), 10).unwrap();
        let int = u8::from_str_radix(table.get_cell("D9").as_str(), 10).unwrap();
        let con = u8::from_str_radix(table.get_cell("D13").as_str(), 10).unwrap();
        let app = u8::from_str_radix(table.get_cell("D17").as_str(), 10).unwrap();
        let pow = u8::from_str_radix(table.get_cell("D21").as_str(), 10).unwrap();
        let siz = u8::from_str_radix(table.get_cell("D25").as_str(), 10).unwrap();
        let edu = u8::from_str_radix(table.get_cell("D29").as_str(), 10).unwrap();

        let weapons = Vec::new() as Vec<String>;

        let mut skills = HashMap::new() as HashMap<String, u8>;
        let digits = (23..=69).collect() as Vec<u8>;
        for index in digits {
            let skillname = table.get_cell(format!("E{}", index).as_str());
            let skilluser = table.get_cell(format!("G{}", index).as_str());
            let skillvalue = if skilluser.is_empty() {
                table.get_cell(format!("F{}", index).as_str())
            } else {
                skilluser
            };
            skills.insert(
                skillname,
                u8::from_str_radix(skillvalue.as_str(), 10).unwrap()
            );
        }

        Character {
            name, max_hp, cur_hp, max_san, cur_san, luck, max_mp, cur_mp,
            str, dex, int, con, app, pow, siz, edu,
            weapons,
            skills
        }
    }
}
