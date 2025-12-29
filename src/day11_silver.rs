use crate::lines_from_bytes;
use crate::load_input;
use std::collections::HashMap;


pub const DAY: i32 = 11;
/// solution to the problem
pub fn solution() -> usize{
    let input = load_input(DAY);
    let lines = parse_input(input);
    let mut hashmap: HashMap<String,Vec<String>> = parse_parsed_input(lines);
    hashmap.insert(String::from("out"), Vec::new());
    //Tests
    // assert_eq!(0,hashmap["you"].0);
    // assert_eq!(0,hashmap["zsv"]);

    let mut cache = HashMap::new();
    return count_paths(&mut cache, &mut hashmap,String::from("svr"), false, false);
    // return count_paths(&mut cache, &mut hashmap,String::from("you"), true, true);

}
// Compared to last day we implemented cache because the result is 557332758684000 so without cache it would be too many function calls
fn count_paths(cache: &mut HashMap<(String, bool, bool), usize>,g: &mut HashMap<String, Vec<String>>,
    node: String,mut dac: bool,mut fft: bool,) -> usize {
    if node == "out" {
        return if dac && fft {1} else {0};
    }
    dac |= node == "dac";
    fft |= node == "fft";
    let key = (node.clone(), dac, fft);
    if !cache.contains_key(&key) {
        let mut res = 0;
        for i in 0..g[&node.clone()].len(){
            res += count_paths(cache, g, g[&node.clone()][i].clone(), dac, fft);
        }
        cache.insert(key.clone(), res);
    }
    cache[&key]
}

/// Parses the input to return the data structure to resolve the problem
fn parse_input(input: Vec<u8>) -> Vec<Vec<u8>>{
    lines_from_bytes(input)
}
fn parse_parsed_input(lines: Vec<Vec<u8>>) -> HashMap<String,Vec<String>> {
    let mut hashmap = HashMap::new();
    for line in lines{
        let key = String::from_utf8(line[0..3].to_vec()).unwrap();
        let len = line.len();
        let n = (len-1)/4; //number of children (including self you: xxx, yyy is 3 children)
        let mut children = Vec::new();
        for i in 1..n{
            children.push(String::from_utf8(line[1+4*i..4*(i+1)].to_vec()).unwrap());
        }
        hashmap.insert(key,children);
    }
    hashmap
}