use crate::load_input;
use crate::lines_from_bytes;

pub const DAY: i32 = 1;

/// solution to the problem
pub fn solution() -> i32{
    let input = load_input(DAY);
    let list_of_movements = parse_input(input);

    //Checking if it parses the file correctly
    assert_eq!(Movement{movementtype:MovementType::Left, amount:32},list_of_movements[0]);
    assert_eq!(Movement{movementtype:MovementType::Left, amount:15},list_of_movements[1]);
    assert_eq!(Movement{movementtype:MovementType::Right, amount:38},list_of_movements[2]);

    let mut lock = Lock{clicks:0,current_number:50};
    for movement in list_of_movements{
        lock.apply_movement(movement);
    }
    return lock.clicks;
}
/// Parses the input to return the data structure to resolve the problem
fn parse_input(input: Vec<u8>) -> Vec<Movement>{
    let lines = lines_from_bytes(input);
    let mut vector = Vec::new();
    for line in lines{
        let mut movementtype = MovementType::Right;
        if line[0] == b'L'{
            movementtype = MovementType::Left;
        }
        let mut num = 0;
        for &byte in &line[1..] {
            if let b'0'..=b'9' = byte {
                num = num * 10 + (byte - b'0') as i32;
            } else {
                break;
            }
        }
        vector.push(Movement { movementtype, amount: num });
    }
    return vector
}
struct Lock{
    clicks: i32, //The amount of times it has got to exactly 0
    current_number: i32, //The number the lock is currently on
}
impl Lock {
    fn apply_movement(self: &mut Self, movement: Movement){
        if movement.movementtype == MovementType::Left{
            for _i in 0..movement.amount{
                self.current_number -= 1;
                if self.current_number == 0{
                    self.clicks += 1;
                } else if self.current_number < 0{
                    self.current_number += 100;
                }
            }
        }else{
            for _i in 0..movement.amount{
                self.current_number += 1;
                if self.current_number == 0{
                    self.clicks += 1;
                } else if self.current_number > 99{
                    self.current_number -= 100;
                    self.clicks += 1;
                }
            }
        }
    }
}
#[derive(PartialEq, Debug)]
struct Movement{
    movementtype: MovementType,
    amount: i32,
}
#[derive(PartialEq, Debug)]
enum MovementType{
    Left,
    Right
}