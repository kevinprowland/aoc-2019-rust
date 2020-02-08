use std::collections::HashMap;
use std::fmt;

/// So, for each module mass, calculate its fuel and add it to the total. Then,
/// treat the fuel mass amount you just calculated as the input mass and repeat
/// the process, continuing until a fuel requirement is zero or negative. For
/// example:
pub fn calculate_fuel_requirement_rec(mass: usize) -> usize {
    if mass <= 8 {
        0
    } else {
        mass / 3 - 2 + calculate_fuel_requirement_rec(mass / 3 - 2)
    }
}

/// Fuel required to launch a given module is based on its mass. Specifically,
/// to find the fuel required for a module, take its mass, divide by three,
/// round down, and subtract 2.
pub fn calculate_fuel_requirement(mass: usize) -> usize {
    mass / 3 - 2
}

fn day1() {
    let module_masses =
        std::fs::read_to_string("day1-1-input.txt").expect("could not read file to string");

    let module_masses = module_masses.split("\n");

    let module_masses = module_masses
        .filter(|e| !e.is_empty())
        .map(|e| usize::from_str_radix(e, 10).expect("Couldn't parse input module mass"));

    let fuel_requirements = module_masses.clone().map(|e| calculate_fuel_requirement(e));

    println!("{:?}", fuel_requirements.sum::<usize>());

    let fuel_requirements = module_masses.map(|e| calculate_fuel_requirement_rec(e));

    println!("{:?}", fuel_requirements.sum::<usize>());
}

pub enum Opcode {
    Uninit = 0,
    Add = 1,
    Mul = 2,
    Halt = 99,
}

impl From<usize> for Opcode {
    fn from(val: usize) -> Opcode {
        match val {
            0 => Opcode::Uninit,
            1 => Opcode::Add,
            2 => Opcode::Mul,
            99 => Opcode::Halt,
            _ => Opcode::Uninit,
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Opcode::Uninit => "UNINIT",
                Opcode::Add => "ADD",
                Opcode::Mul => "MUL",
                Opcode::Halt => "HALT",
            }
        )
    }
}

pub struct IntcodeState {
    pub pc: usize,
    pub done: bool,
    pub op: Opcode,
    pub debug: bool,
}

impl IntcodeState {
    pub fn new(intcode: &Vec<usize>) -> IntcodeState {
        IntcodeState {
            pc: 0,
            done: false,
            op: Opcode::from(intcode[0]),
            debug: false,
        }
    }
}

fn execute_opcode(mut s: IntcodeState, intcode: &mut Vec<usize>) -> IntcodeState {
    if s.debug {
        println!("{}", s.op);
    }

    match s.op {
        Opcode::Add => {
            let (idx1, idx2, idx3) = (intcode[s.pc + 1], intcode[s.pc + 2], intcode[s.pc + 3]);
            intcode[idx3] = intcode[idx1] + intcode[idx2];
            s.pc = s.pc + 4;
        }
        Opcode::Mul => {
            let (idx1, idx2, idx3) = (intcode[s.pc + 1], intcode[s.pc + 2], intcode[s.pc + 3]);
            intcode[idx3] = intcode[idx1] * intcode[idx2];
            s.pc = s.pc + 4;
        }
        Opcode::Halt => {
            s.done = true;
        }
        _ => {
            s.pc = s.pc + 1;
        }
    };

    // Advance state by updating Opcode
    s.op = Opcode::from(intcode[s.pc]);

    s
}

fn execute_intcode(mut intcode: &mut Vec<usize>) {
    let mut s: IntcodeState = IntcodeState::new(&intcode);
    loop {
        s = execute_opcode(s, &mut intcode);
        if s.done {
            break;
        }
    }
}

fn day2() {
    println!();
    println!("Day 2");
    println!();
    let intcode =
        std::fs::read_to_string("day2-1-input.txt").expect("could not read file to string");

    let mut intcode: Vec<usize> = intcode
        .trim()
        .split(",")
        .filter(|e| !e.is_empty())
        .map(|e| usize::from_str_radix(e, 10).expect("Couldn't parse input opcode"))
        .collect();

    intcode[1] = 12;
    intcode[2] = 2;

    execute_intcode(&mut intcode);

    println!("{}", intcode[0]);

    'nouns: for noun in 0..=99 {
        '_verbs: for verb in 0..99 {
            let intcode =
                std::fs::read_to_string("day2-1-input.txt").expect("could not read file to string");
            let mut intcode: Vec<usize> = intcode
                .trim()
                .split(",")
                .filter(|e| !e.is_empty())
                .map(|e| usize::from_str_radix(e, 10).expect("Couldn't parse input opcode"))
                .collect();

            intcode[1] = noun;
            intcode[2] = verb;

            execute_intcode(&mut intcode);

            if intcode[0] == 19690720 {
                println!("{}{}", noun, verb);
                break 'nouns;
            }
        }
    }
}

/// Given a coordinate on the wire grid, a string that describes the next
/// motion, a reference to the wire grid, and the current length of the wire,
/// update the wire grid with new entries and return the final coordinate and
/// new length.
fn do_step(
    mut coord: (isize, isize),
    s: &str,
    grid: &mut HashMap<(isize, isize), isize>,
    mut length: isize,
) -> ((isize, isize), isize) {
    match &s[0..=0] {
        "U" => {
            for _ in 0..isize::from_str_radix(&s[1..], 10).unwrap() {
                coord.1 += 1;
                length += 1;
                grid.insert((coord.0, coord.1), length);
            }
        }
        "R" => {
            for _ in 0..isize::from_str_radix(&s[1..], 10).unwrap() {
                coord.0 += 1;
                length += 1;
                grid.insert((coord.0, coord.1), length);
            }
        }
        "L" => {
            for _ in 0..isize::from_str_radix(&s[1..], 10).unwrap() {
                coord.0 -= 1;
                length += 1;
                grid.insert((coord.0, coord.1), length);
            }
        }
        "D" => {
            for _ in 0..isize::from_str_radix(&s[1..], 10).unwrap() {
                coord.1 -= 1;
                length += 1;
                grid.insert((coord.0, coord.1), length);
            }
        }
        _ => println!("bad direction!"),
    };

    (coord, length)
}

fn day3() {
    println!();
    println!("Day 3");
    println!();

    let path = std::fs::read_to_string("day3-2-input.txt")
        .expect("could not read file to string");

    let paths = path.split("\n").filter(|e| !e.is_empty());

    let mut master: HashMap<(isize, isize), isize> = HashMap::new();
    let mut grid: HashMap<(isize, isize), isize> = HashMap::new();

    let mut closest_collision = isize::max_value();
    let mut closest_collision_2 = isize::max_value();

    for (_i, p) in paths.enumerate() {
        let steps = p.split(",").filter(|e| !e.is_empty());

        let mut loc = ((0isize, 0isize), 0isize);

        for s in steps {
            loc = do_step(loc.0, s, &mut grid, loc.1);
        }

        // let collisions = master.intersection(&grid);
        let mut collisions: HashMap<(isize, isize), isize> = HashMap::new();

        for elem in master.keys() {
            if let Some(val) = grid.get(elem) {
                collisions.insert(*elem, master.get(elem).unwrap() + val);
            }
        }

        for c in collisions {
            // Collisions in part 1 are ordered by their Hamming distance from
            // the origin (x-coordinate + y-coordinate).
            closest_collision = std::cmp::min(
                isize::abs((c.0).0) + isize::abs((c.0).1),
                closest_collision,
            );

            // Collisions in part 2 are ordered by the length of the 2 wires
            // causing the collision, a value that is tracked in the collisions
            // HashMap
            closest_collision_2 = std::cmp::min(c.1, closest_collision_2);
        }

        // We keep track of a master grid after the first and second wires have
        // been placed in order to accommodate future tasks with more than two
        // wires
        for elem in grid.iter() {
            master.insert(*elem.0, *elem.1);
        }

        grid.clear()
    }

    println!("closest collision: {:?}", closest_collision);
    println!("closest collision 2: {:?}", closest_collision_2);
}

fn main() {
    // day1();
    // day2();
    day3();
}
