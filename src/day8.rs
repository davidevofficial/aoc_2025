use crate::lines_from_bytes;
use crate::load_input;

pub const DAY: i32 = 8;

/// solution to the problem
pub fn solution() -> u64{
    let input = load_input(DAY);
    let lines = parse_input(input);
    let mut junction_boxes = parse_parsed_input(lines);
    let mut circuits = Vec::new();
    let mut connections = Vec::new();
    //Tests
    assert_eq!(25,(JunctionBox::from(0,0,0,0)-JunctionBox::from(3,4,0,0)));
    assert_eq!(JunctionBox::from(58660,9565,9912,0),junction_boxes[0]);
    assert_eq!(JunctionBox::from(87631,12487,66875,1),junction_boxes[1]);

    for j in 0..junction_boxes.len(){
        circuits.push(Circuit::new(j as u64));
    }
    for i in 0..junction_boxes.len(){
        for j in i+1..junction_boxes.len(){
            connections.push(Connection::from(i as u64,j as u64,junction_boxes[i], junction_boxes[j]));
        }
    }
    connections.sort();
    connections = connections[0..1000].to_vec();
    for c in connections{
        let old_circuit = junction_boxes[c.from as usize].c;
        let new_circuit = junction_boxes[c.to as usize].c;
        if old_circuit != new_circuit{
            for j in circuits[old_circuit as usize].clone().j_idx{
                junction_boxes[j as usize].c = new_circuit;
                circuits[new_circuit as usize].add_jbox(j);
            }
            circuits[old_circuit as usize].destroy_circuit();
        }

    }
    let mut ranking = Vec::new();
    for c in circuits{
        ranking.push(c.size);
    }
    ranking.sort();
    ranking[999]*ranking[998]*ranking[997]

}
/// Parses the input to return the data structure to resolve the problem
fn parse_input(input: Vec<u8>) -> Vec<Vec<u8>>{
    lines_from_bytes(input)
}
fn parse_parsed_input(lines: Vec<Vec<u8>>) -> Vec<JunctionBox>{
    let mut junction_boxes = Vec::new();
    let mut i = 0;
    for line in lines{
        let mut comma_idx = Vec::new();
        for i in 0..line.len(){
            if line[i] == b','{comma_idx.push(i)}
            if line[i] == b'\n'{comma_idx.push(i)}
        }
        let x: i32 = String::from_utf8(line[0..comma_idx[0]].to_vec()).unwrap().parse().unwrap();
        let y: i32 = String::from_utf8(line[comma_idx[0]+1..comma_idx[1]].to_vec()).unwrap().parse().unwrap();
        let z: i32 = String::from_utf8(line[comma_idx[1]+1..comma_idx[2]].to_vec()).unwrap().parse().unwrap();
        junction_boxes.push(JunctionBox::from(x,y,z,i));
        i+=1
    }

    junction_boxes
}
#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct JunctionBox{
    x: i32,
    y: i32,
    z: i32,
    c: u64
}
impl JunctionBox {
    fn from(x: i32, y: i32, z: i32, c: u64) -> JunctionBox{
        return JunctionBox{x,y,z,c}
    }
}
impl std::ops::Sub for JunctionBox{
    type Output = u64;
    fn sub(self, rhs: Self) -> Self::Output {
        (self.x as i64 - rhs.x as i64).pow(2) as u64+
        (self.y as i64 - rhs.y as i64).pow(2) as u64+
        (self.z as i64 - rhs.z as i64).pow(2) as u64
    }
}
#[derive(Debug, Default,Clone)]
struct Circuit{
    j_idx: Vec<u64>,
    size: u64,
}
impl Circuit{
    fn new(j_idx: u64) -> Circuit{
        Circuit { j_idx: vec![j_idx], size: 1 }
    }
    fn destroy_circuit(self: &mut Self){
        self.j_idx = Vec::new();
        self.size = 0;
    }
    fn add_jbox(self: &mut Self, jbox_idx:u64){
        self.j_idx.push(jbox_idx);
        self.size += 1;
    }
}
#[derive(Debug, Default,PartialEq, Eq, PartialOrd, Ord,Clone, Copy)]
struct Connection{
    distance: u64,
    from: u64,
    to: u64,
}
impl Connection{
    fn from(i: u64, j: u64, j1: JunctionBox, j2: JunctionBox) -> Connection{
        Connection { distance: j1-j2, from: i, to: j }
    }
}