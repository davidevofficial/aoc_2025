use crate::lines_from_bytes;
use crate::load_input;

pub const DAY: i32 = 9;

/// solution to the problem
pub fn solution() -> u64{
    let input = load_input(DAY);
    let lines = parse_input(input);
    let red_tiles = parse_parsed_input(lines);
    let mut horizontal_lines = Vec::new();
    let mut vertical_lines = Vec::new();
    //Tests
    assert_eq!(9,(RedTile::from(0,0)*RedTile::from(2,2)));
    assert_eq!(1,(RedTile::from(0,0)*RedTile::from(0,0)));
    assert_eq!(6,(RedTile::from(7,3)*RedTile::from(2,3)));
    assert_eq!(RedTile::from(98366,50148),red_tiles[0]);
    assert_eq!(RedTile::from(98366,51373),red_tiles[1]);



    for i in 0..red_tiles.len(){
        let r1 = red_tiles[i];
        let r2;
        if i+1<red_tiles.len(){r2=red_tiles[i+1]}
        else{r2=red_tiles[0]}

        if r1.x == r2.x{
            horizontal_lines.push(Line::from(true, r1.y, r2.y));
        }else if r1.y == r2.y{
            vertical_lines.push(Line::from(false, r1.x, r2.x));
        }

    }
    let mut max = Vec::new();
    for i in 0..red_tiles.len(){
        for j in i..red_tiles.len(){
            let x = red_tiles[i]*red_tiles[j];
            max.push((x,i,j));
        }
    }
    max.sort();
    max.reverse();
    for i in 0..max.len(){
        //try each area and check if borders intersect with all other lines
        let r1 = red_tiles[max[i].1];
        let r2 = red_tiles[max[i].2];
        let x1;
        if r1.x<r2.x{x1=r1.x}else{x1=r2.x}
        let x2;
        if r1.x<r2.x{x2=r2.x}else{x2=r1.x}
        let y1;
        if r1.y<r2.y{y1=r1.y}else{y1=r2.y}
        let y2;
        if r1.y<r2.y{y2=r2.y}else{y2=r1.y}
        let rect = todo!();

        let mut is_this_the_one = true;
        for k in horizontal_lines{
            if is_this_the_one{
                if k.intersect(rect){
                    is_this_the_one = false;
                }
            }
        }
        for k in vertical_lines{
            if is_this_the_one{
                if k.intersect(rect){
                    is_this_the_one = false;
                }
            }
        }

    }

    return max[0].0;

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
/// Could be either horizontal or vertical
#[derive(Debug, Default,Clone, Copy)]
struct Line{
    /// Number refers to, for example, (number, from) -> (number, to) so like (5, 3) -> (5, 7)
    // number: i32,
    /// From must be smaller than to
    from: (i32,i32),
    /// To must be higher than from
    to: (i32,i32),
}
struct Rect{a: (i32,i32), b: (i32,i32), c: (i32,i32), d: (i32,i32)}
impl Line{
    fn from(horizontal:bool,number:i32, x1: i32, x2: i32) -> Line{
        if x1 < x2{
            if horizontal{
                return Line {from:(x1,number), to:(x2, number)};
            }else{
                Line { from: (number, x1), to: (number, x2) }
            }
        }else{
            if horizontal{
                return Line {from:(x2,number), to:(x1, number)};
            }else{
                Line { from: (number, x2), to: (number, x1) }
            }
        }
    }
    fn intersect(self: &Self, rect: Rect) -> bool{


        return false;
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