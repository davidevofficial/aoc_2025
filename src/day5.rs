use crate::lines_from_bytes;
use crate::load_input;

pub const DAY: i32 = 5;

//Check if number is greater than from and less than to for each range

/// solution to the problem
pub fn solution() -> u64{
    let input = load_input(DAY);
    let ranges_and_ids = parse_input(input);
    let mut counter = 0;
    //Tests
    assert_eq!(ranges_and_ids.ranges[0].from, 124495185257650);
    assert_eq!(ranges_and_ids.ranges[0].to, 128523382238124);
    assert_eq!(ranges_and_ids.ids[0], 345448950878627);




    for i in 0..ranges_and_ids.ids.len(){
        if ranges_and_ids.check_id(ranges_and_ids.ids[i]){
            counter += 1;
        }
    }


    return counter;
}
/// Parses the input to return the data structure to resolve the problem
fn parse_input(input: Vec<u8>) -> RangesAndIds{
    let lines = lines_from_bytes(input);
    let mut sep = 0;
    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    for i in 0..lines.len(){
        if lines[i] == vec![10]{
            sep = i
        }
    }
    if sep == 0{todo!()}
    for i in 0..sep{
        ranges.push(parse_range(lines[i].clone()));
    }
    for i in sep+1..lines.len(){
        let mut l = lines[i].clone();
        l.pop().unwrap();
        ids.push(String::from_utf8(l).unwrap().parse().unwrap());
    }
    return RangesAndIds { ranges, ids }
}
fn parse_range(input: Vec<u8>) -> Range{
    let l = String::from_utf8(input.clone()).unwrap();
    let dash_idx = input.iter().position(|x|x == &b'-').unwrap();

    let from = &input[0..dash_idx];
    let from: u64 = String::from_utf8(from.to_vec())
        .map_err(|_| "Invalid UTF-8").unwrap()
        .parse()
        .map_err(|_| "Not a number").unwrap();

    let to = &input[dash_idx+1..l.len()-1];
    let to: u64 = String::from_utf8(to.to_vec())
        .map_err(|_| "Invalid UTF-8").unwrap()
        .parse()
        .map_err(|_| "Not a number").unwrap();
    Range{from,to}
}

struct RangesAndIds{
    ranges: Vec<Range>,
    ids: Vec<u64>
}
impl RangesAndIds{
    fn check_id(self: &Self, id: u64) -> bool{
        let mut b = false;
        for r in &self.ranges{
            if id >= r.from && id <= r.to{
                b = true;
            }
        }
        b
    }
}
struct Range{
    from: u64,
    to: u64
}