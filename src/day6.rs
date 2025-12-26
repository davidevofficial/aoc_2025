use crate::lines_from_bytes;
use crate::load_input;

pub const DAY: i32 = 6;

//Math homeworks: just build the parser and for how many n are in first line just: o=l5[x] (*|+)  l1[x]o l2[x]o l3[x]o l4[x]

/// solution to the problem
pub fn solution() -> u64{
    let input = load_input(DAY);
    let lines = parse_input(input);
    let parsed_lines = parse_lines(lines);
    let mut counter = 0;
    //Tests
    assert_eq!(parsed_lines[0][0], Code::N(73));
    assert_eq!(parsed_lines[0][1], Code::N(95));
    assert_eq!(parsed_lines[0][2], Code::N(62));
    assert_eq!(parsed_lines[1][0], Code::N(21));
    assert_eq!(parsed_lines[1][1], Code::N(986));
    assert_eq!(parsed_lines[4][0], Code::M);
    for x in 0..parsed_lines[0].len(){
        match parsed_lines[4][x]{
            Code::M => {counter += u64::from(parsed_lines[0][x].clone())*u64::from(parsed_lines[1][x].clone())*
                                   u64::from(parsed_lines[2][x].clone())*u64::from(parsed_lines[3][x].clone())},
            Code::A => {counter += u64::from(parsed_lines[0][x].clone())+u64::from(parsed_lines[1][x].clone())+
                                   u64::from(parsed_lines[2][x].clone())+u64::from(parsed_lines[3][x].clone())},
            _ => {}
        }
    }

    return counter;
}
/// Parses the input to return the data structure to resolve the problem
fn parse_input(input: Vec<u8>) -> Vec<Vec<u8>>{
    lines_from_bytes(input)
}
#[derive(Debug, Default, PartialEq, Clone)]
enum Code{
    N(i32),
    #[default]
    M,
    A
}
impl From<Code> for u64 {
    fn from(code: Code) -> u64 {
        match code {
            Code::N(n) => n as u64,
            _ => 0,  // M/A default to 0
        }
    }
}
fn parse_lines(lines: Vec<Vec<u8>>) -> Vec<Vec<Code>>{
    let mut parsed_lines = Vec::new();
    for i in 0..lines.len(){
        parsed_lines.push(Vec::new());
        let mut idx = 0;
        let mut pidx = 0;
        let mut inside_space = false;
        while idx < lines[i].len(){
            if lines[i][idx] == b'\n'{
                if lines[i][pidx] == b'*'{parsed_lines[i].push(Code::M);}
                else if lines[i][pidx] == b'+'{parsed_lines[i].push(Code::A);}
                else{parsed_lines[i].push(Code::N(String::from_utf8(lines[i][pidx..idx].to_vec()).unwrap().parse().unwrap()))}
            }
            if lines[i][idx] == b' '{
                if !inside_space{
                    if lines[i][pidx] == b'*'{parsed_lines[i].push(Code::M);}
                    else if lines[i][pidx] == b'+'{parsed_lines[i].push(Code::A);}
                    else{parsed_lines[i].push(Code::N(String::from_utf8(lines[i][pidx..idx].to_vec()).unwrap().parse().unwrap()))}
                }
                inside_space = true;
            }else{
                if inside_space == true{
                    pidx = idx;
                }
                inside_space = false;
            }
            idx += 1
        }
    }
    parsed_lines
}