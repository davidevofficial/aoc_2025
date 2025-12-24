use crate::lines_from_bytes;
use crate::load_input;
use crate::lines_from_comma_separated_bytes;

pub const DAY: i32 = 3;

//Joltage... find biggest digit (that isn't last for example 14811224890: 8 on index 8) and then the biggest digit after that index (in that case 9)

/// solution to the problem
pub fn solution() -> u64{
    let input = load_input(DAY);
    let list_of_batteries = parse_input(input);
    let mut counter = 0;
    //Tests
    assert_eq!(check_line("987654321111111\n".to_string().as_bytes().to_vec()), 98);
    assert_eq!(check_line("811111111111119\n".to_string().as_bytes().to_vec()), 89);
    assert_eq!(check_line("234234234234278\n".to_string().as_bytes().to_vec()), 78);
    assert_eq!(check_line("818181911112111\n".to_string().as_bytes().to_vec()), 92);

    for batteries in list_of_batteries{
        counter += check_line(batteries);
    }

    return counter;
}
/// Parses the input to return the data structure to resolve the problem
fn parse_input(input: Vec<u8>) -> Vec<Vec<u8>>{
    lines_from_bytes(input)
}
fn check_line(number: Vec<u8>) -> u64{
    let mut highest_digit = 0;
    let mut idx_highest_digit = 0;
    let mut second_highest_digit = 0;
    for x in 0..number.len()-2{
        if number[x] > highest_digit{
            highest_digit = number[x];
            idx_highest_digit = x;
        }
    }
    for x in idx_highest_digit+1..number.len(){
        if number[x] > second_highest_digit{
            second_highest_digit = number[x];
        }
    }
    let first_digit = ((highest_digit-b'0')*10) as u64;
    let second_digit = ((second_highest_digit-b'0')) as u64;
    return first_digit + second_digit;
}
