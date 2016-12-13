use std::collections::VecDeque;
use std::collections::BTreeSet;

type Material = u8;

// These might not be very rusty
const MAX_MATERIALS: Material = 8;
const MAX_UNIQUE_COMPONENTS: usize = (MAX_MATERIALS * 2) as usize;
const FLOORS: usize = 4;

#[derive(Clone, Copy)]
// Represent a single component. Each component
// has an unique position on the floor.
enum Component {
    Chip(Material),
    Generator(Material)
}

impl Component {
    fn from_floor_pos(pos: usize) -> Component {
        let material = pos / 2;
        match pos % 2 {
            0 => Component::Chip(material as u8),
            1 => Component::Generator(material as u8),
            _ => panic!()
        }
    }

    fn floor_pos(&self) -> usize {
        match *self {
            Component::Chip(material) => (material as usize) * 2,
            Component::Generator(material) => (material as usize) * 2 + 1
        }
    }
}

#[derive(Clone, Copy, Debug, Ord, Eq, PartialOrd, PartialEq)]
struct Floor {
    components: [bool; MAX_UNIQUE_COMPONENTS]
}

impl Floor {
    fn put(&mut self, c: Component) {
        assert!(!self.components[c.floor_pos()]);

        self.components[c.floor_pos()] = true;
    }
    
    fn take(&mut self, pos: usize) -> Option<Component> {
        if self.components[pos] {
            self.components[pos] = false;
            return Some(Component::from_floor_pos(pos));
        }
        
        None
    }
    
    fn has_generators(&self) -> bool {
        for i in 0..(MAX_MATERIALS as usize) {
            if self.components[i * 2 + 1] {
                return true
            }
        }
        
        false
    }

    fn validate(&self) -> bool {
        // No generators -> must be a valid floor
        if !self.has_generators() {
            return true
        }
        
        // This floor has generators (and possibly chips) - each chip must have
        // a generator present to power up the shields
        for i in 0..(MAX_MATERIALS as usize) {
            if self.components[i * 2] && !self.components[i * 2 + 1] {
                return false;
            }
        }

        true
    }
    
    fn is_empty(&self) -> bool {
        self.components.iter().all(|b| !*b)
    }
}


#[derive(Clone, Debug, Ord, Eq, PartialOrd, PartialEq)]
struct State
{
    elevator: usize,
    floors: [Floor; FLOORS as usize]
}

impl State {
    fn new() -> State {
        State {
            elevator: 0,
            floors: [Floor{ components: [false; MAX_UNIQUE_COMPONENTS] }; FLOORS]
        }
    }
    
    fn is_valid(&self) -> bool {
        (self.elevator as usize) < FLOORS && self.floors.iter().all(|f| f.validate())
    }

    // Might generate new state by moving the given components on the floors
    fn elevator_transfer(&self, elevator_shift: i32, c1: usize, c2: Option<usize>) -> Option<State> {
        let new_floor = self.elevator as i32 + elevator_shift;

        if new_floor < 0 || new_floor >= FLOORS as i32 {
            return None
        }

        let old_floor = self.elevator as usize;
        let new_floor = new_floor as usize;

        let mut new_state = self.clone();
        new_state.elevator = new_floor;

        let mut transferred = new_state.floors[old_floor].take(c1 as usize).map_or(false,
            |c| { new_state.floors[new_floor].put(c); true });

        transferred |= c2.map_or(false, |ind| {
            new_state.floors[old_floor].take(ind as usize).map_or(false, |c| {
                new_state.floors[new_floor].put(c); true
            })
        });

        match transferred && new_state.is_valid() {
            true => Some(new_state),
            false => None
        }
    }
    
    fn is_final(&self) -> bool {
        self.floors[0..FLOORS - 1].iter().all(|f| f.is_empty())
    }
}

fn find_shortest_path(state: State) -> u32 {
    let mut front: VecDeque<(State, u32)> = VecDeque::new();
    let mut states: BTreeSet<State> = BTreeSet::new();
    
    front.push_back((state, 0));

    while let Some((state, length)) = front.pop_front() {
        // println!("{} {}", front.len(), length);
        if state.is_final() {
            return length
        }

        let mut add_state = |s: Option<State>, length| {
            match s {
                Some(s) => {
                    if !states.contains(&s) {
                        front.push_back((s.clone(), length + 1));
                        states.insert(s);
                    }
                },
                None => ()
            }
        };

        // Generate mutations
        for i in [-1, 1].iter() {
            for c1 in 0..MAX_UNIQUE_COMPONENTS {
                add_state(state.elevator_transfer(*i, c1, None), length);

                for c2 in (c1 + 1)..MAX_UNIQUE_COMPONENTS {
                    add_state(state.elevator_transfer(*i, c1, Some(c2)), length);
                }
            }
        }
    }

    0
}

fn main() {
    let mut a = State::new();

    // 0 = thulium, 1 = plutonium, 2=strontium, 3=promethium, 4=ruthinium
    // The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium generator, and a strontium generator.
    // The second floor contains a plutonium-compatible microchip and a strontium-compatible microchip.
    // The third floor contains a promethium generator, a promethium-compatible microchip, a ruthenium generator, and a ruthenium-compatible microchip.
    a.floors[0].put(Component::Chip(0));
    a.floors[0].put(Component::Generator(0));
    a.floors[0].put(Component::Generator(1));
    a.floors[0].put(Component::Generator(2));
    a.floors[1].put(Component::Chip(1));
    a.floors[1].put(Component::Chip(2));
    a.floors[2].put(Component::Generator(3));
    a.floors[2].put(Component::Chip(3));
    a.floors[2].put(Component::Generator(4));
    a.floors[2].put(Component::Chip(4));
    
    let mut b = a.clone();

    println!("A: {}", find_shortest_path(a));

    // 5, 6 extra materials
    b.floors[0].put(Component::Chip(5));
    b.floors[0].put(Component::Generator(5));
    b.floors[0].put(Component::Chip(6));
    b.floors[0].put(Component::Generator(6));

    println!("B: {}", find_shortest_path(b));
}
