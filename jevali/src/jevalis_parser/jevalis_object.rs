//jevalis Object is the responsible for storing each object of the json schema
//an object is composed of primitive json types, and other objects
//example: 
//[object=Person]
//name: str
//age: int
//childrens: list[person]
//
//This defines a person, that can have persons as childs

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
    object_type: JevalisType,
    settings: JevalisObjectSettings,
}

impl JevalisObject {
    pub fn new_empty() -> Self {
        Self {
            object_type: JevalisType::any,
            settings: JevalisObjectSettings::default(),
        }
    }
}


