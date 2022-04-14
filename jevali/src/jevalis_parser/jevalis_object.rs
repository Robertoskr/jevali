//jevalis Object is the responsible for storing each object of the json schema
//an object is composed of primitive json types, and other objects
//example: 
//[object=Person]
//name: str
//age: int
//childrens: list[person]
//
//This defines a person, that can have persons as childs

use std::collections::HashMap;


pub enum JevalisType{
    string,
    int, 
    float,
    boolean,
    list,
    object,
    any,
}

pub enum JevalisFieldType{
    basic,
    compound,
}

pub struct JevalisObjectSettings {
    required: bool,
    max_len: i32,
}

impl JevalisObjectSettings {
    pub fn default() -> Self {
        Self {
            required: false,
            max_len: -1, //when max_len == -1, there is no max_len
        }
    }
}

pub struct JevalisObject{
    //an object is the building block of a schema, and it has the job
    //of validating an object with itself
    //for example if i have an object person, and i see person in 
    //a json, the object is the responsible of validating the person json
    pub name: String,
    //the type, of the object
    pub object_type: JevalisType,
    //only filled if the object_type is an object
    pub fields: HashMap<String, JevalisObject>,
    //the settings of the object
    pub settings: JevalisObjectSettings,
}

impl JevalisObject {
    pub fn new_empty() -> Self {
        Self {
            name: String::from(""),
            object_type: JevalisType::any,
            fields: HashMap::new(),
            settings: JevalisObjectSettings::default(),
        }
    }
}


