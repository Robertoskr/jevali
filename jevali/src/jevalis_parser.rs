//jevalis parses is the responsible of parsing the jevalis file, and get the schema and its rules
use std::collections::{HashSet, HashMap};
use std::process;

mod utilities;
mod jevalis_object;

use utilities::*;
use jevalis_object::*;

pub struct JevalisSettings {
    check_types: bool,
} //this is the Core of this file, the responsible of validating the json input 

pub struct JevalisParser{
    pub objects: HashMap<String, JevalisObject>, 
    pub required_objects: HashSet<String>, 
    pub settings: JevalisSettings, 
    file_object_index: HashMap<String, usize>,
}

impl JevalisParser {
    pub fn new(jevalis_file: &Vec<String>) -> Self {
        //for creating the parser, wee need to parse the jevalis file and get all the info 

        let mut parser = Self{
            objects: HashMap::new(),
            required_objects: HashSet::new(),
            settings: JevalisSettings::default(),
            file_object_index: HashMap::new(),
        };
        
        parser.parse_objects(&jevalis_file).expect("Failed to parse Jevalis File, invalid file");

        parser
    }

    fn parse_objects(&mut self, jevalis_file: &Vec<String>) -> Result<(), &str> {
        //parses the objects of the jevalis file, and returns a result for error handling purposes

        //first parse the main object, always at the top of the file 
        if !jevalis_file[0].eq(&String::from("[schema]")){
            return Err("Wrong format on jevalis file, file must start with the main schema");
        }
        
        self.file_object_index.insert(String::from("main"), 0);
        self.fill_file_object_index(&jevalis_file)
            .expect("Failed to fill the file Object Index, incorrect jevalis format");

        //parse the main object, and recursively, parse its childrens
        self.parse_object(&jevalis_file, 0);
        
        Ok(())
    }
    
    pub fn parse_object(
        &mut self, 
        jevalis_lines: &Vec<String>,
        object_start_idx: usize,
        ) -> Result<(), ()>{
        //object_start_idx is expected to be the idx of the line, in wich the object starts
        //[object=object_name]
        //if not it will not work properly
        let name = JevalisParser::get_object_name_from_line(&jevalis_lines[object_start_idx]);

        if !self.file_object_index.contains_key(&name) {
            return Err(());
        }
        
        let mut object = JevalisObject::new_empty();
        object.name = name.clone();
        
        //perform the actual parsing of the object
        let mut actual_idx = object_start_idx + 1;
        while !jevalis_lines[actual_idx].eq("") {
            actual_idx = self.parse_field(&jevalis_lines, &actual_idx, &name);
        }
        Ok(())

    }
    
    pub fn parse_field(&mut self, jevalis_lines: &Vec<String>, idx: &usize, name: &String) -> usize {
        match self.field_type(&jevalis_lines, &idx) {
            JevalisFieldType::basic => {
                return self.parse_basic_field(&jevalis_lines, &idx, &name);
            },
            JevalisFieldType::compound => {
                return self.parse_compound_field(&jevalis_lines, &idx, &name);
            },
        }
        0
    }

    pub fn parse_basic_field(
        &mut self, 
        jevalis_lines: &Vec<String>, 
        idx: &usize,
        name: &String, 
    ) -> usize {
        //this is a basic field, only name and type 
        //parse it and add to self.objects[name].fields with the field name as key, and field
        //object as value
        let line_words: Vec<String> = get_line_words(&jevalis_lines[*idx]);
        println!("{:?}", line_words);
        let field_name = line_words[0].clone();
        let field_type = self.parse_field_type(&line_words);

        self.objects.get_mut(name).unwrap().fields.insert(field_name, field_type);
        *idx + 1
    }

    pub fn field_type(&self, jevalis_lines: &Vec<String>, idx: &usize) -> JevalisFieldType {
        if *idx < jevalis_lines.len() - 1 {
            if starts_with(&jevalis_lines[*idx], &String::from(" ".repeat(2))) {
                //this is a compound field type ( has settings ) 
                return JevalisFieldType::compound;
            }else if starts_with(&jevalis_lines[*idx], &String::from(" ".repeat(3))) {
                //invalid file type 
                process::exit(1);
            }
        }
        JevalisFieldType::basic
    }

    pub fn parse_field_type(&self, words: &Vec<String>) -> JevalisObject {
        JevalisObject::new_empty()
    }

    pub fn parse_compound_field(
        &mut self, 
        jevalis_lines: &Vec<String>,
        idx: &usize, 
        name: &String, 
    ) -> usize {
        0
    }

    
    pub fn fill_file_object_index(&mut self, file_lines: &Vec<String>)
    -> Result<(),()>{
       //fill the file_object_index with the position of all the objects in the file 
        for file_index in 1..file_lines.len() {
            if file_lines[file_index].starts_with("[object="){
                let object_name = JevalisParser::get_object_name_from_line(&file_lines[file_index]);
                if self.file_object_index.contains_key(&object_name) {
                    return Err(());
                }
                self.file_object_index.insert(object_name, file_index);
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
