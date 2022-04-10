//jevalis parses is the responsible of parsing the jevalis file, and get the schema and its rules
use std::collections::{HashSet, HashMap};

enum JevalisType{
    string,
    int, 
    float,
    boolean,
    list,
    object,
    any,
}

pub struct JevalisObjectSettings {
    required: bool,
    max_len: u32,
}

pub struct JevalisObject{
    type: JevalisType,
    settings: JevalisObjectSettings,
}

pub struct JevalisSettings {
    check_types: bool,
}

//this is the Core of this file, the responsible of validating the json input 
pub struct JevalisParser{
    objects: HashMap<String, JevalisObject>,
    required_object: HashSet<String>,
    settings: JevalisSettings,
}

impl JevalisParser {
    pub fn new(jevalis_file: &Vec<String>) -> Self {
        //for creating the parser, wee need to parse the jevalis file and get all the info 
        let jevalis_objects: Vec<JevalisObject> = JevalisParser::parse_objects(&jevalis_file);
        Self {
            objects: HashMap::new(),
            required_objects: HashSet::new(),
            settings: JevalisSettings::default(),
        }
    }

    fn parse_objects(jevalis_file: &Vec<String>) -> Result<Vec<JevalisObject>, Err> {
        let mut result: Vec<String> = Vec::new();
        let mut actual_idx = 0;
        //first parse the main object, always at the top of the file 
        if !jevalis_file[0].eq(&String::from("[schema]"){
            return Err("Wrong format on jevalis file");
        }
        actual_idx += 1;
        Ok(result)
    }
}


impl JevalisSettings {
    pub fn default() -> Self {
        Self {
            check_types: false,
        }
    }
}
