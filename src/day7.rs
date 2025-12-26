use crate::lines_from_bytes;
use crate::load_input;

pub const DAY: i32 = 7;

//Math homeworks: just build the parser and for how many n are in first line just: o=l5[x] (*|+)  l1[x]o l2[x]o l3[x]o l4[x]

/// solution to the problem
pub fn solution() -> u64{
    let input = load_input(DAY);
    let lines = parse_input(input);

    let mut counter = 0;
    //Tests

    let mut line = Vec::new();
    for _i in 0..lines[0].len()-1{
        line.push(false);
    }
    for i in 0..lines.len(){
        let r = update_line(line, lines[i].clone());
        line = r.0;
        counter += r.1;
    }



    return counter;
}
/// Parses the input to return the data structure to resolve the problem
fn parse_input(input: Vec<u8>) -> Vec<Vec<u8>>{
    lines_from_bytes(input)
}

fn update_line(line: Vec<bool>, other_line: Vec<u8>)-> (Vec<bool>,u64){
    let mut new_line = line.clone();
    let mut changes = 0;
    for i in 0..other_line.len()-1{
        // First Line where all the bools are still 0 and S is the generator
        if other_line[i] == b'S'{
            new_line[i] = true;
        }
        if other_line[i] == b'^' && line[i] == true{
            new_line[i] = false;
            new_line[i+1] = true;
            new_line[i-1] = true;
            changes += 1;
        }
    }
    (new_line, changes)
}