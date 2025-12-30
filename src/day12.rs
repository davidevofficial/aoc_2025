use crate::lines_from_bytes;
use crate::load_input;


pub const DAY: i32 = 12;
// const PRESENT_0: [[u8; 3]; 3] =[[1,0,1],[1,0,1],[1,1,1]];
// const PRESENT_1: [[u8; 3]; 3] =[[1,1,1],[0,1,0],[1,1,1]];
// const PRESENT_2: [[u8; 3]; 3] =[[1,1,1],[1,1,0],[0,1,1]];
// const PRESENT_3: [[u8; 3]; 3] =[[1,1,1],[0,1,1],[0,0,1]];
// const PRESENT_4: [[u8; 3]; 3] =[[1,1,1],[1,1,1],[1,0,0]];
// const PRESENT_5: [[u8; 3]; 3] =[[0,1,1],[1,1,0],[1,0,0]];
/// solution to the problem
pub fn solution() -> u64{
    let mut counter = 0;
    let input = load_input(DAY);
    let lines = parse_input(input);
    let regions: Vec<Region> = parse_parsed_input(lines);
    //Tests
    assert_eq!(Region{width:44,height:48,n_of_presents:vec![35,35,39,42,41,32]},regions[0]);
    //assert random region is zzz
    //asset last region is yyy

    for r in regions{
        if r.min_area() as f64 * 1.3 < r.area() as f64{
            counter += 1;
        }
    }

    return counter;
}
#[derive(Debug, Default,PartialEq, Eq)]
struct Region{
    height: u64,
    width: u64,
    n_of_presents: Vec<u64> // [5 0 2 4 5] means 5 presents 0, 0 presents 1 etc...
}
impl Region{
    fn area(self: &Self)->u64{
        return self.height*self.width;
    }
    fn min_area(self: &Self)->u64{
        let mut counter = 0;
        for n in 0..self.n_of_presents.len(){
            match n{
                0 => {counter+=self.n_of_presents[n]*7},
                1 => {counter+=self.n_of_presents[n]*7},
                2 => {counter+=self.n_of_presents[n]*7},
                3 => {counter+=self.n_of_presents[n]*6},
                4 => {counter+=self.n_of_presents[n]*7},
                5 => {counter+=self.n_of_presents[n]*5},
                _ =>{}
            }
        }
        counter
    }
}
/// Parses the input to return the data structure to resolve the problem
fn parse_input(input: Vec<u8>) -> Vec<Vec<u8>>{
    lines_from_bytes(input)
}
fn parse_parsed_input(lines: Vec<Vec<u8>>) -> Vec<Region> {
    let mut regions = Vec::new();
    for line in lines[30..].to_vec(){
        let width = String::from_utf8(line[0..2].to_vec()).unwrap().parse::<u64>().unwrap();
        let height = String::from_utf8(line[3..5].to_vec()).unwrap().parse::<u64>().unwrap();
        let mut n_of_presents = Vec::new();
        let mut last_whitespace = 6;
        for _i in 0..6{

            let mut next_whitespace = line.len()-1;
            for j in last_whitespace+1..line.len(){
                if line[j] == b' ' || line[j] == b'\n' && next_whitespace == last_whitespace{
                    next_whitespace = j;
                    break;
                }
            }
            let bytes = line[last_whitespace+1..next_whitespace].to_vec();
            let bytes = String::from_utf8(bytes).unwrap();
            n_of_presents.push(bytes.parse::<u64>().unwrap());
            last_whitespace = next_whitespace;
        }
        regions.push(Region { height, width, n_of_presents })
    }
    regions
}