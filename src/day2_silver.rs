use crate::load_input;
use crate::lines_from_comma_separated_bytes;

pub const DAY: i32 = 2;

/// solution to the problem
pub fn solution() -> u64{
    let input = load_input(DAY);
    let list_of_ranges = parse_input(input);
    let mut counter = 0;
    //Tests
    assert_eq!(Range{from:851786270,to:851907437},list_of_ranges[0]);
    assert_eq!(check_double(11), true);
    assert_eq!(check_double(1010), true);
    assert_eq!(check_double(123), false);
    assert_eq!(check_double(38593859), true);
    assert_eq!(check_double(2121212121), true);


    for list in list_of_ranges{
        for x in (list.from)..(list.to+1){
            if check_double(x){
                counter += x;
            }
        }
    }
    return counter;
}
/// Parses the input to return the data structure to resolve the problem
fn parse_input(input: Vec<u8>) -> Vec<Range>{
    let mut vector = Vec::new();
    let lines = lines_from_comma_separated_bytes(input);
    for line in lines{
        let l = String::from_utf8(line.clone()).unwrap();
        let dash_idx = line.iter().position(|x|x == &b'-').unwrap();

        let from = &line[0..dash_idx];
        let from: u64 = String::from_utf8(from.to_vec())
            .map_err(|_| "Invalid UTF-8").unwrap()
            .parse()
            .map_err(|_| "Not a number").unwrap();

        let to = &line[dash_idx+1..l.len()-1];
        let to: u64 = String::from_utf8(to.to_vec())
            .map_err(|_| "Invalid UTF-8").unwrap()
            .parse()
            .map_err(|_| "Not a number").unwrap();
        vector.push(Range{from,to});
    }
    return vector
}
fn check_double(number: u64) -> bool{
    // if divisible by 2 && two equal parts -> true
    // if divisible by 3 && three equal parts -> true
    // if divisible by 4 && four equal parts -> true
    let number_string = number.to_string();
    let mut equal = false;
    for x in 2..12{
        let len = number_string.len();
        if len % x == 0{
            let mut list: [&str; 12] = [""; 12];
            for i in 0..x{
                list[i] = &(number_string[(len/x)*i..(len/x)*(i+1)]);
            }
            let mut equal2 = true;
            for i in 0..x-1{
                if list[i] != list[i+1]{
                    equal2 = false;
                }
            }
            if equal2{
                equal = true;
            }
        }
    }
    return equal;
}
#[derive(Debug, Default, PartialEq)]
struct Range{
    from: u64,
    to: u64,
}