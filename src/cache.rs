use std::collections::HashMap;
use crate::csreader::get_sheet;
use std::sync::{Arc, Mutex};

pub struct Cache {
    sheets: Vec<String>
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            sheets: Vec::new() as Vec<String>
        }
    }

    pub fn get_sheet(&self, name: &str) -> Option<Vec<Vec<String>>> {
        for (index, s) in self.sheets.iter().enumerate() {
            let sheet = self.gen_table(index);
            let sname = &sheet[0][1];
            if name == sname {
                return Some(sheet);
            }
        }
        None
    }

    pub fn gen_table(&self, index: usize) -> Vec<Vec<String>> {
        let sheet = &self.sheets[index];
        let mut table = Vec::new() as Vec<Vec<String>>;

        for r in sheet.split("\n") {
            let mut row = Vec::new() as Vec<String>;
            for col in r.split(",") {
                row.push(col.to_string());
            }
            table.push(row);
        }

        table
    }

    pub fn table_from_data(data: &str) -> Vec<Vec<String>> {
        let sheet = data;
        let mut table = Vec::new() as Vec<Vec<String>>;

        for r in sheet.split("\n") {
            let mut row = Vec::new() as Vec<String>;
            for col in r.split(",") {
                row.push(col.to_string());
            }
            table.push(row);
        }

        table
    }

    pub fn contains(&self, name: &str) -> bool {
        return if let Some(x) = self.get_sheet(name) {
            true
        } else {
            false
        }
    }

    pub fn add(&mut self, data: &str) -> Vec<Vec<String>> {
        let sheet = Cache::table_from_data(data);
        let check = self.contains(&sheet[0][1]);
        if !check {
            self.sheets.push(data.to_string());
            return sheet;
        } else {
            return self.get_sheet(&sheet[0][1]).unwrap();
        }
    }

    pub fn get_stat(&mut self, name: &str, stat: &str) -> Option<i32> {
        let index: (usize, usize) = match stat {
            "hp" => (9, 1),
            "sanity" => (13, 1),
            "luck" => (15, 1),
            "mp" => (18, 1),
            "str" => (0, 3),
            "dex" => (4, 3),
            "int" | "idea" => (8, 3),
            "con" => (13, 3),
            "app" => (16, 3),
            "pow" => (20, 3),
            "siz" => (24, 3),
            "edu" => (28, 3),
            _ => (0xff, 0xff)
        };
        let sheet = self.get_sheet(name);
        if sheet == None {
            let data = get_sheet(name).unwrap();
            let s = Cache::table_from_data(data.as_str());
            self.add(&data);
            let sretval = &s[index.0][index.1].trim();
            let retval = sretval.parse::<i32>().unwrap();
            return Some(retval)
        } else {
            let s = sheet.unwrap();
            let sretval = &s[index.0][index.1].trim();
            let retval = sretval.parse::<i32>().unwrap();
            return Some(retval);
        }
    }

    pub fn get_skill(&mut self, name: &str, s: &str) -> Option<i32> {
        let mut skill = s;
        let mut divide = 1;
        if skill.ends_with(" hard") {
            divide = 2;
            skill = &skill.replace(" hard", "");
        } else if skill.ends_with(" impossible") {
            divide = 3;
            skill = &skill.replace(" impossible", "");
        }
        let offset: usize = match skill {
            "Accounting" => 0,
            "Anthropology" => 1,
            "Appraise" => 2,
            "Archaeology" => 3,
            "Art" | "Craft" => 4,
            "Charm" => 5,
            "Climb" => 6,
            "Computer Use" => 7,
            "Credit Rating" => 8,
            "Cthulhu Mythos" => 9,
            "Disguise" => 10,
            "Dodge" => 11,
            "Drive Auto" => 12,
            "Elec Repair" => 13,
            "Electronics" => 14,
            "Fast Talk" => 15,
            "Brawl" => 16,
            "Handgun" => 17,
            "Rifle" | "Shotgun" => 18,
            "First Aid" => 19,
            "History" => 20,
            "Intimidate" => 21,
            "Jump" => 22,
            "Language Other" => 23,
            "Language Own" => 24,
            "Law" => 25,
            "Library Use" => 26,
            "Listen" => 27,
            "Locksmith" => 28,
            "Mech Repair" => 29,
            "Medicine" => 30,
            "NaturalWorld" => 31,
            "Navigate" => 32,
            "Occult" => 33,
            "Operate Heavy Machine" => 34,
            "Persuade" => 35,
            "Pilot" => 36,
            "Psychology" => 37,
            "Psychoanalysis" => 38,
            "Science" => 39,
            "Sleight Of Hand" => 40,
            "Spot Hidden" => 41,
            "Stealth" => 42,
            "Survival" => 43,
            "Swim" => 44,
            "Throw" => 45,
            "Track" => 46,
            _ => 0xff
        };
        let origin = 22;
        let index = origin + offset;
        if index >= 69 {
            return None;
        }

        let sheet = self.get_sheet(name);
        let mut value = 0;
        if sheet.is_none() {
            let data = get_sheet(name).unwrap();
            self.add(&data);
            let s = Cache::table_from_data(data.as_str());
            let base = s[index][5].trim();
            let modified = s[index][6].trim();
            let k = if modified.is_empty() {
                base
            } else {
                modified
            };
            value = k.parse::<i32>().unwrap();
        } else {
            let s = sheet.unwrap();
            let base = s[index][5].trim();
            let modified = s[index][6].trim();
            let k = if modified.is_empty() {
                base
            } else {
                modified
            };
            value = k.parse::<i32>().unwrap();
        }

        Some(value/divide)
    }
}

lazy_static! {
    pub static ref CACHE: Arc<Mutex<Cache>> = Arc::new(Mutex::new(Cache::new()));
}
