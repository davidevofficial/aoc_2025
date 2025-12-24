use crate::lines_from_bytes;
use crate::load_input;

pub const DAY: i32 = 5;

//Check if number is greater than from and less than to for each range

/// solution to the problem
pub fn solution() -> i64{
    let input = load_input(DAY);
    let mut ranges_and_ids = parse_input(input);
    let mut counter = 0;
    //Tests
    assert_eq!(ranges_and_ids.ranges[0].from, 124495185257650);
    assert_eq!(ranges_and_ids.ranges[0].to, 128523382238124);
    assert_eq!(ranges_and_ids.ids[0], 345448950878627);

    ranges_and_ids.merge_ranges();
    for r in ranges_and_ids.ranges{
        counter += r.to as i64 - r.from as i64 + 1
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
    #[allow(dead_code)]
    fn check_id(self: &Self, id: u64) -> bool{
        let mut b = false;
        for r in &self.ranges{
            if id >= r.from && id <= r.to{
                b = true;
            }
        }
        b
    }
    fn merge_ranges(self: &mut Self){
        for r in 0..self.ranges.len(){
            for other_r in r+1..self.ranges.len(){
                if r == other_r{
                    break;
                }
                let b1 = self.ranges[other_r].from >= self.ranges[r].from;
                let b2 = self.ranges[other_r].to >= self.ranges[r].to;
                let b3 = self.ranges[other_r].from > self.ranges[r].to;
                let b4 = self.ranges[other_r].to >= self.ranges[r].from;
                match (b1,b2,b3,b4) {
                    //ranges do not overlap
                    (true, true, true, true) => {},
                    (false, false, false, false) => {},
                    //second range is inside first one
                    (true, false, _, _) => {self.ranges[other_r].from =1;self.ranges[other_r].to = 0;},
                    //first range is inside second one
                    (false, true, _, _) => {self.ranges[r].from =1;self.ranges[r].to = 0;},
                    //there is an overlap on the left
                    (false, false, false, true) => {self.ranges[r].from = self.ranges[other_r].to + 1;},
                    //there is an overlap on the right
                    (true, true, false, true) => {self.ranges[r].to = self.ranges[other_r].from - 1;},
                     _         => {},
                }

            }
        }
    }
}
#[derive(Debug, Default)]
struct Range{
    from: u64,
    to: u64
}