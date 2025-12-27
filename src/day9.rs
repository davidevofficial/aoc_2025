use crate::lines_from_bytes;
use crate::load_input;

pub const DAY: i32 = 9;

/// solution to the problem
pub fn solution() -> u64{
    let input = load_input(DAY);
    let lines = parse_input(input);
    let red_tiles = parse_parsed_input(lines);
    //Tests
    // assert_eq!(10000,(RedTile::from(100,100)*RedTile::from(200,200)));
    // assert_eq!(10000,(RedTile::from(100,100)*RedTile::from(0,200)));
    assert_eq!(1,(RedTile::from(0,0)*RedTile::from(0,0)));
    assert_eq!(6,(RedTile::from(7,3)*RedTile::from(2,3)));
    assert_eq!(RedTile::from(98366,50148),red_tiles[0]);
    assert_eq!(RedTile::from(98366,51373),red_tiles[1]);

    let mut max = 0;
    for i in 0..red_tiles.len(){
        for j in i..red_tiles.len(){
            let x = red_tiles[j]*red_tiles[i];
            if x > max{
                max = x;
            }
        }
    }
    return max;

}
/// Parses the input to return the data structure to resolve the problem
fn parse_input(input: Vec<u8>) -> Vec<Vec<u8>>{
    lines_from_bytes(input)
}
fn parse_parsed_input(lines: Vec<Vec<u8>>) -> Vec<RedTile>{
    let mut red_tiles = Vec::new();
    for line in lines{
        let mut comma_idx = Vec::new();
        for i in 0..line.len(){
            if line[i] == b','{comma_idx.push(i)}
            if line[i] == b'\n'{comma_idx.push(i)}
        }
        let x: i32 = String::from_utf8(line[0..comma_idx[0]].to_vec()).unwrap().parse().unwrap();
        let y: i32 = String::from_utf8(line[comma_idx[0]+1..comma_idx[1]].to_vec()).unwrap().parse().unwrap();
        red_tiles.push(RedTile::from(x,y));
    }

    red_tiles
}
#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct RedTile{
    x: i32,
    y: i32,
}
impl RedTile {
    fn from(x: i32, y: i32) -> RedTile{
        return RedTile{x,y}
    }
}
impl std::ops::Mul for RedTile{
    type Output = u64;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut delta_x = (self.x - rhs.x) as i64;
        if delta_x < 0{
            delta_x = -delta_x;
        }
        let mut delta_y = (self.y - rhs.y) as i64;
        if delta_y < 0{
            delta_y = -delta_y;
        }
        return ((delta_x+1)*(delta_y+1)) as u64;
    }
}
