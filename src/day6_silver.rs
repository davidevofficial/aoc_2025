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
    assert_eq!(parsed_lines[0][0], Code::M);
    assert_eq!(parsed_lines[0][1], Code::N(7218));
    assert_eq!(parsed_lines[0][2], Code::N(312));
    assert_eq!(parsed_lines[1][0], Code::A);
    assert_eq!(parsed_lines[1][1], Code::N(26));
    assert_eq!(parsed_lines[1][2], Code::N(991));



    for x in 0..parsed_lines.len(){
        match parsed_lines[x][0]{
            Code::M => {
                let mut n = 1;
                for i in 1..parsed_lines[x].len(){
                    n *= u64::from(parsed_lines[x][i].clone());
                }
                counter += n
            },
            Code::A => {
                for i in 1..parsed_lines[x].len(){
                    counter += u64::from(parsed_lines[x][i].clone());
                }
            },
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
    N(i32), //Number
    #[default]
    M, //Multiply
    A, //Add
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
    let mut symbols_idx = Vec::new();
    for i in 0..lines[4].len(){
        if lines[4][i] == b'*' || lines[4][i] == b'+' {
            symbols_idx.push(i);
        }
    }
    for i in 0..symbols_idx.len(){
        parsed_lines.push(Vec::new());
        let symbol;
        if lines[4][symbols_idx[i]] == b'*'{symbol = Code::M}
        else {symbol = Code::A}
        parsed_lines[i].push(symbol);
    }
    for i in 0..symbols_idx.len(){
        if i + 1 >= symbols_idx.len(){
            let current = symbols_idx[i];
            let next = lines[0].len();
            for j in current..(next-1){
                let mut n = Vec::new();
                if lines[0][j] != b' '{n.push(lines[0][j] - b'0')}
                if lines[1][j] != b' '{n.push(lines[1][j] - b'0')}
                if lines[2][j] != b' '{n.push(lines[2][j] - b'0')}
                if lines[3][j] != b' '{n.push(lines[3][j] - b'0')}
                let mut number = 0;
                for k in 0..n.len(){
                    number += n[k] as i32 *10_i32.pow(n.len() as u32-k as u32-1 as u32);
                }
                parsed_lines[i].push(Code::N(number));
            }

        }else{
            let current = symbols_idx[i];
            let next = symbols_idx[i+1];
            for j in current..(next-1){
                let mut n = Vec::new();
                if lines[0][j] != b' '{n.push(lines[0][j] - b'0')}
                if lines[1][j] != b' '{n.push(lines[1][j] - b'0')}
                if lines[2][j] != b' '{n.push(lines[2][j] - b'0')}
                if lines[3][j] != b' '{n.push(lines[3][j] - b'0')}
                let mut number = 0;
                for k in 0..n.len(){
                    number += n[k] as i32 *10_i32.pow(n.len() as u32-k as u32-1 as u32);
                }
                parsed_lines[i].push(Code::N(number));
            }

        }

    }
    parsed_lines
}
