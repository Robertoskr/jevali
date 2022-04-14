//this file is intended to content utilities used along all the project 
use std::collections::HashSet;


//returns if the string a starts with the string b
pub fn starts_with(a: &String, b: &String) -> bool {
    if b.len() > a.len() {
        return false;
    }
    if b.len() == 0{
        return true;
    }
    for (i, ch) in b.chars().enumerate() {
        if ch != a.chars().nth(i).unwrap() {
            return false;
        }
    }
    true
}

//returns the words of the line in a vector
//a word is any sequence of string without spaces and without reserved characters
//reserved characters = [:, =, (, ), {, [, ], }, ^, /, \, |] 
pub fn get_line_words(line: &String) -> Vec<String> {
    let reserved_characters: HashSet<char> = HashSet::from_iter(
        vec![':', '=', '(', ')', '{', '}', '[', ']', '^', '/', '|', ' ']
    );
    let mut result: Vec<String> = Vec::new();
    let mut len_result = 0;
    for (i, ch) in line.chars().enumerate() {
        if !reserved_characters.contains(&ch){
            if result.len() == 0{
                result.push(String::from(""));
                len_result += 1;
            }
            result[len_result-1].push(ch);
        }else{
            if result.len() == 0 {
                result.push(String::from(""));
                len_result += 1;
            }
            else if result[len_result-1].len() > 0 {
                result.push(String::from(""));
                len_result+=1;
            }
        }
    }       
    result   
}
