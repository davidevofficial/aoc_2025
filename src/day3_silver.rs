use std::process::abort;

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
    assert_eq!(check_line("987654321111111\n".to_string().as_bytes().to_vec()), 987654321111);
    assert_eq!(check_line("811111111111119\n".to_string().as_bytes().to_vec()), 811111111119);
    assert_eq!(check_line("234234234234278\n".to_string().as_bytes().to_vec()), 434234234278);
    assert_eq!(check_line("818181911112111\n".to_string().as_bytes().to_vec()), 888911112111);
    assert_eq!(check_line("111999999999999\n".to_string().as_bytes().to_vec()), 999999999999);
    assert_eq!(check_line("123456789010\n".to_string().as_bytes().to_vec()), 123456789010);
    assert_eq!(check_line("100223456789010\n".to_string().as_bytes().to_vec()), 223456789010);

    for batteries in list_of_batteries{
        counter += check_line(batteries);
    }

    return counter;
}
/// Parses the input to return the data structure to resolve the problem
fn parse_input(input: Vec<u8>) -> Vec<Vec<u8>>{
    lines_from_bytes(input)
}
fn check_line(mut number: Vec<u8>) -> u64{
    let n = 12;
    number.pop().unwrap();
    let mut digits: [u64; 12] = [0; 12];
    let mut indices = [0; 13];
    for i in 0..n{
        for j in indices[i]..number.len() - n + 1 + i {
            if digits[i] < number[j] as u64{
                digits[i] = number[j] as u64;
                indices[i+1] = j+1;
            }
        }
    }
    digits[0] = (digits[0]- (b'0' as u64))*100000000000;
    digits[1] = (digits[1]- (b'0' as u64))*10000000000;
    digits[2] = (digits[2]- (b'0' as u64))*1000000000;
    digits[3] = (digits[3]- (b'0' as u64))*100000000;
    digits[4] = (digits[4]- (b'0' as u64))*10000000;
    digits[5] = (digits[5]- (b'0' as u64))*1000000;
    digits[6] = (digits[6]- (b'0' as u64))*100000;
    digits[7] = (digits[7]- (b'0' as u64))*10000;
    digits[8] = (digits[8]- (b'0' as u64))*1000;
    digits[9] = (digits[9]- (b'0' as u64))*100;
    digits[10] = (digits[10]- (b'0' as u64))*10;
    digits[11] =digits[11]- (b'0' as u64);
    return digits[0]+digits[1]+digits[2]+
            digits[3]+digits[4]+digits[5]+
            digits[6]+digits[7]+digits[8]+
            digits[9]+digits[10]+digits[11];
}
