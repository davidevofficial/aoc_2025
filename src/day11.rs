use crate::lines_from_bytes;
use crate::load_input;
use std::collections::HashMap;


pub const DAY: i32 = 11;
/// solution to the problem
pub fn solution() -> u64{
    let input = load_input(DAY);
    let lines = parse_input(input);
    let mut hashmap: HashMap<String, (u64 ,Vec<String>)> = parse_parsed_input(lines);
    hashmap.insert(String::from("out"),(0, Vec::new()));
    //Tests
    assert_eq!(1,hashmap["you"].0);
    assert_eq!(0,hashmap["zsv"].0);

    update_child(&String::from("you"), &mut hashmap, 1);

    return hashmap["out"].0;
}
fn update_child(key: &String, hashmap: &mut HashMap<String, (u64 ,Vec<String>)>, num: u64){
    if hashmap.contains_key(key){
        let (_n,children) = hashmap[key].clone();
        if children.len() == 0{
            // Key is out
            hashmap.get_mut(key).unwrap().0 += num;
        }
        for child in children{
            update_child(&child, hashmap, num);
        }
    }

}
/// Parses the input to return the data structure to resolve the problem
fn parse_input(input: Vec<u8>) -> Vec<Vec<u8>>{
    lines_from_bytes(input)
}
fn parse_parsed_input(lines: Vec<Vec<u8>>) -> HashMap<String, (u64 ,Vec<String>)> {
    let mut hashmap = HashMap::new();
    for line in lines{
        let key = String::from_utf8(line[0..3].to_vec()).unwrap();
        let len = line.len();
        let n = (len-1)/4; //number of children (including self you: xxx, yyy is 3 children)
        let mut children = Vec::new();
        for i in 1..n{
            children.push(String::from_utf8(line[1+4*i..4*(i+1)].to_vec()).unwrap());
        }
        if key == String::from("you"){
            hashmap.insert(key, (1,children));
        }else{
            hashmap.insert(key, (0,children));
        }
    }
    hashmap
}