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
    let l1 = Line::from(5, 0, 10, Direction::Right);
    let l2 = Line::from(10, 5, 10, Direction::Up);
    let l11 = Line::from(10, 0, 10, Direction::Left);
    let l22 = Line::from(0, 5, 10, Direction::Down);

    let l3 = Line::from(5, 0, 10, Direction::Up);
    let l4 = Line::from(5, 0, 5, Direction::Up);
    let l5 = Line::from(5, 5, 10, Direction::Up);
    let l6 = Line::from(5, 10, 15, Direction::Up);
    let l7 = Line::from(5, 10, -10, Direction::Up);
    assert_eq!(true,l1.intersect(&l2,&l3));
    assert_eq!(false,l1.intersect(&l2,&l4));
    assert_eq!(false,l2.intersect(&l11,&l7));
    assert_eq!(true,l11.intersect(&l22,&l7));
    assert_eq!(false,l22.intersect(&l1,&l7));
    assert_eq!(false,l2.intersect(&l11,&l7));


    for i in 0..red_tiles.len(){
        let r1 = red_tiles[i];
        let r2;
        if i+1<red_tiles.len(){r2=red_tiles[i+1]}
        else{r2=red_tiles[0]}

        if r1.x == r2.x{
            vertical_lines.push(Line::from(r1.x, r1.y, r2.y, Direction::Up));
        }else if r1.y == r2.y{
            horizontal_lines.push(Line::from(r1.y, r1.x, r2.x, Direction::Right));
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
        let border_lines = [Line::from(y1, x1, x2, Direction::Right),Line::from(x2, y1, y2, Direction::Up),
                                        Line::from(y2, x1, x2, Direction::Left),Line::from(x1, y1, y2, Direction::Down)];

        let mut is_this_the_one = true;
        for j in 0..border_lines.len(){
            let j1 = j;
            let j2;
            if j+1>3{j2=0}else{j2=j+1}
            let j1 = border_lines[j1];
            let j2 = border_lines[j2];
            for k in vertical_lines.clone(){
                if is_this_the_one && j % 2 == 0{
                    if j1.intersect(&j2, &k){is_this_the_one=false;}
                }
            }
            for k in horizontal_lines.clone(){
                if is_this_the_one && j % 2 == 1{
                    if j1.intersect(&j2, &k){is_this_the_one=false;}
                }
            }

        }
        if is_this_the_one{
            return max[i].0;
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
    number: i32,
    /// From must be smaller than to
    from: i32,
    /// To must be higher than from
    to: i32,
    direction: Direction
}
#[derive(Debug, Default,Copy,Clone,PartialEq, Eq, PartialOrd, Ord)]
enum Direction{
    Up,
    #[default]
    Down,
    Left,
    Right
}
impl Line{
    fn from(number: i32, x1: i32, x2: i32, direction: Direction) -> Line{
        if x1 < x2{
            return Line { number, from: x1, to: x2, direction }
        }else{
            return Line { number, from: x2, to: x1, direction }
        }
    }
    fn intersect(self: &Self, next_line: &Line, other: &Line) -> bool{
        match (self.direction, next_line.direction){
            (Direction::Up, Direction::Right)=>{
                // First case: intersects a line that is exiting
                if self.number == other.to && self.from < other.number && self.to > other.number{return false}
                // Second case: intersects a line
                else if self.number >= other.from && self.number <= other.to && self.from < other.number && self.to > other.number{return true}
            },
            (Direction::Up, Direction::Left)=>{
                // First case: intersects a line that is exiting
                if self.number == other.from && self.from < other.number && self.to > other.number{return false}
                // Second case: intersects a line
                else if self.number >= other.from && self.number <= other.to && self.from < other.number && self.to > other.number{return true}
            },
            (Direction::Down, Direction::Right)=>{
                // First case: intersects a line that is exiting
                if self.number == other.to && self.from < other.number && self.to > other.number{return false}
                // Second case: intersects a line
                else if self.number >= other.from && self.number <= other.to && self.from < other.number && self.to > other.number{return true}
            },
            (Direction::Down, Direction::Left)=>{
                // First case: intersects a line that is exiting
                if self.number == other.from && self.from < other.number && self.to > other.number{return false}
                // Second case: intersects a line
                else if self.number >= other.from && self.number <= other.to && self.from < other.number && self.to > other.number{return true}
            },
            (Direction::Right, Direction::Up)=>{
                // First case: intersects a line that is exiting
                if self.number == other.to && self.from < other.number && self.to > other.number{return false}
                // Second case: intersects a line
                else if self.number >= other.from && self.number <= other.to && self.from < other.number && self.to > other.number{return true}
            },
            (Direction::Right, Direction::Down)=>{
                // First case: intersects a line that is exiting
                if self.number == other.from && self.from < other.number && self.to > other.number{return false}
                // Second case: intersects a line
                else if self.number >= other.from && self.number <= other.to && self.from < other.number && self.to > other.number{return true}
            },
            (Direction::Left, Direction::Up)=>{
                // First case: intersects a line that is exiting
                if self.number == other.to && self.from < other.number && self.to > other.number{return false}
                // Second case: intersects a line
                else if self.number >= other.from && self.number <= other.to && self.from < other.number && self.to > other.number{return true}
            },
            (Direction::Left, Direction::Down)=>{
                // First case: intersects a line that is exiting
                if self.number == other.from && self.from < other.number && self.to > other.number{return false}
                // Second case: intersects a line
                else if self.number >= other.from && self.number <= other.to && self.from < other.number && self.to > other.number{return true}
            },
            _=>{}
        }
        return false;
    }
}
// todo!() implement intersection between two perpendicular lines
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