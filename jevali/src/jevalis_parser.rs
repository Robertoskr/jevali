//jevalis parses is the responsible of parsing the jevalis file, and get the schema and its rules
use std::collections::{HashSet, HashMap};

mod jevalis_object;

use jevalis_object::*;

pub struct JevalisSettings {
    check_types: bool,
}

//this is the Core of this file, the responsible of validating the json input 
pub struct JevalisParser{
    objects: HashMap<String, JevalisObject>,
    required_objects: HashSet<String>,
    settings: JevalisSettings,
}

impl JevalisParser {
    pub fn new(jevalis_file: &Vec<String>) -> Self {
        //for creating the parser, wee need to parse the jevalis file and get all the info 
        let jevalis_objects: Vec<JevalisObject> = JevalisParser::parse_objects(&jevalis_file)
            .expect("Failed to parse jevalis file, invalid file");
        Self {
            objects: HashMap::new(),
            required_objects: HashSet::new(),
            settings: JevalisSettings::default(),
        }
    }

    fn parse_objects(jevalis_file: &Vec<String>) -> Result<Vec<JevalisObject>, &str> {
        let mut result: Vec<JevalisObject> = Vec::new();
        let mut actual_idx = 0;
        //first parse the main object, always at the top of the file 
        if !jevalis_file[0].eq(&String::from("[schema]")){
            return Err("Wrong format on jevalis file, file must start with the main schema");
        }
        
        //create an object file position index
        let mut fileObjectIndex: HashMap<String, usize> = HashMap::new();
        
        fileObjectIndex.insert(String::from("main"), 0);
        JevalisParser::fill_file_object_index(&mut fileObjectIndex, &jevalis_file)
            .expect("Failed to fill the file Object Index, incorrect jevalis format");
        
        //parse the main object, and recursively, parse its childrens
        JevalisParser::parse_object(&jevalis_file, &mut result, 0);
        
        actual_idx += 1;
        Ok(result)
    }

    pub fn fill_file_object_index(file_object_index: &mut HashMap<String, usize>, file_lines: &Vec<String>)
    -> Result<(),()>{
       //fill the file_object_index with the position of all the objects in the file 
        for file_index in 1..file_lines.len() {
            if file_lines[file_index].starts_with("[object="){
                let object_name = JevalisParser::get_object_name_from_line(&file_lines[file_index]);
                if file_object_index.contains_key(&object_name) {
                    return Err(());
                }
                file_object_index.insert(object_name, file_index);
            }
        }
        Ok(())
    }

    fn get_object_name_from_line(file_line: &String) -> String {
        let mut file_line_clone = file_line.clone();
        let chars: Vec<char> = file_line_clone.chars().map(|ch| {ch}).collect();
        let mut result: String = String::new();
        for i in 8..file_line.len()-1 {
            result.push(chars[i]);
        }
        result
    }
}




impl JevalisSettings {
    pub fn default() -> Self {
        Self {
            check_types: false,
        }
    }
}
