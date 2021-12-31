//
// Day 23
//
// First, a couple observations...
//
// Every amphipod that is not in the right room, or is blocking
// another from leaving, must move into the hallway, and then
// move back.  The cost of moving to the hallway and then into
// place is constant, and we don't need to worry about it when
// searching for the minimum cost.
//
// If it's possible for an amphipod in the hallway to move into
// its correct room, there's no reason not to do so immediately.
// And, if there are multiple that can, the ordering doesn't
// matter.
//
// There's never a need to move an amphipod that's in the right
// room and not blocking anybody.
//

use crate::types::{AdventResult, Answer, Day, DayPart};

/// Types are Amber ('A'), Bronze ('B'), Copper ('C'), and Desert ('D')
type AmphipodType = char;

/// Names a position in the hallway.  The first position is always 0, and then
/// they count up from there.
type HallwayPosition = usize;

fn cost_of_moving(amphipod_type: AmphipodType) -> usize {
    match amphipod_type {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!("unknown amphipod type: {:?}", amphipod_type),
    }
}

fn is_amphipod_type(amphipod_type: AmphipodType) -> bool {
    'A' <= amphipod_type && amphipod_type <= 'D'
}

/// The state of one room.
#[derive(Debug)]
struct Room {
    // Which type of amphipod belongs in this room?
    whose_room: AmphipodType,

    // What type is in the spot nearest the hallway
    near_seat: Option<AmphipodType>,

    // What type is in the spot farther from the hallway
    far_seat: Option<AmphipodType>,

    // What position in the hallway does this room connect to?
    hallway_position: HallwayPosition,
}

impl Room {
    /// Returns the fixed cost of moving amphipods that don't
    /// belong in this room out to the nearest hallway location,
    /// and then moving the ones that belong here in from the
    /// nearest hallway location.
    ///
    /// Caller is responsible for making sure that the room is
    /// in the initial state, and fully occupied.
    ///
    fn fixed_cost(&self) -> usize {
        let owner = self.whose_room;
        let far_seat = self.far_seat.unwrap();
        let near_seat = self.near_seat.unwrap();
        if far_seat == owner {
            if near_seat == owner {
                0
            } else {
                cost_of_moving(near_seat) + cost_of_moving(owner)
            }
        } else {
            // both will have to move out, and then both owners
            // move in.
            2 * cost_of_moving(far_seat) + cost_of_moving(near_seat) + 3 * cost_of_moving(owner)
        }
    }

    /// Is there an amphipod in this room that needs to move out?
    fn needs_move_out(&self) -> bool {
        fn needs_move(whose_room: AmphipodType, seat: Option<AmphipodType>) -> bool {
            seat.is_some() && seat.unwrap() != whose_room
        }
        needs_move(self.whose_room, self.near_seat) || needs_move(self.whose_room, self.far_seat)
    }

    /// Is this room in the desired state?
    fn is_done(&self) -> bool {
        self.far_seat == Some(self.whose_room) && self.near_seat == Some(self.whose_room)
    }

    /// Restore the state of this room
    fn restore(&mut self, before: (Option<AmphipodType>, Option<AmphipodType>)) {
        self.near_seat = before.0;
        self.far_seat = before.1;
    }
}

/// The state of the hallway
#[derive(Debug)]
struct Hallway {
    /// What's at each position in the hallway?  These are
    /// either '.' for empty, an AmphipodType, or 'X' for
    /// not allowed.
    contents: Vec<AmphipodType>,
}

impl Hallway {
    fn len(&self) -> usize {
        self.contents.len()
    }
    fn is_walkable_to(&self, from: usize, to_exclusive: usize) -> bool {
        let range = if from < to_exclusive {
            from..to_exclusive
        } else {
            (to_exclusive + 1)..(from - 1)
        };
        range.all(|i| {
            let c = self.contents[i];
            c == '.' || c == 'X'
        })
    }
}

// TODO: test_is_walkable_to

#[derive(Debug)]
struct State {
    hallway: Hallway,
    rooms: Vec<Room>,
    fixed_cost: usize,
}

impl State {
    fn new(rooms: Vec<Room>, hallway_length: usize) -> State {
        println!("AAA {:?}", rooms);
        let mut hallway_contents = vec!['.'; hallway_length];
        for room in rooms.iter() {
            hallway_contents[room.hallway_position] = 'X';
        }
        let fixed_cost = rooms.iter().map(|r| r.fixed_cost()).sum();
        State {
            hallway: Hallway {
                contents: hallway_contents,
            },
            rooms: rooms,
            fixed_cost,
        }
    }

    /// Are all the rooms done?
    fn is_done(&self) -> bool {
        self.rooms.iter().all(|r| r.is_done())
    }

    /// Returns a reference to the room at the given position
    /// in the hallway.
    fn room_at<'a>(&'a self, pos: usize) -> Option<&'a Room> {
        for room in self.rooms.iter() {
            if room.hallway_position == pos {
                return Some(room);
            }
        }
        None
    }

    /// Prints the state in the same form as the input, with 'X'
    /// in the hallway positions that are not legal to occupy.
    fn print(&self) {
        for _ in 0..(self.hallway.len() + 2) {
            print!("#");
        }
        println!("");
        print!("#");
        for c in self.hallway.contents.iter() {
            print!("{}", c);
        }
        println!("#");
        print!("#");
        for pos in 0..self.hallway.len() {
            if let Some(room) = self.room_at(pos) {
                if let Some(amphipod) = room.near_seat {
                    print!("{}", amphipod);
                } else {
                    print!(".");
                }
            } else {
                print!("#");
            }
        }
        println!("#");
        let min_room = self.rooms.iter().map(|r| r.hallway_position).min().unwrap();
        let max_room = self.rooms.iter().map(|r| r.hallway_position).max().unwrap();
        let fill_in = (min_room - 1)..=(max_room + 1);
        print!(" ");
        for pos in 0..self.hallway.len() {
            if let Some(room) = self.room_at(pos) {
                if let Some(amphipod) = room.far_seat {
                    print!("{}", amphipod);
                } else {
                    print!(".");
                }
            } else {
                if fill_in.contains(&pos) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
        }
        println!("");
        print!(" ");
        for pos in 0..self.hallway.len() {
            if fill_in.contains(&pos) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
        println!("fixed cost = {:?}", self.fixed_cost);
        println!("");
    }
}

fn parse_state(lines: &[&str]) -> State {
    let hallway_length = lines[1].len() - 2;
    let mut rooms = Vec::new();
    let mut next_type = 'A';
    for (i, c) in lines[2].chars().enumerate() {
        if is_amphipod_type(c) {
            let room = Room {
                whose_room: next_type,
                near_seat: Some(c),
                far_seat: lines[3].chars().skip(i).next(), // TODO: do better
                hallway_position: i - 1,
            };
            rooms.push(room);
            next_type = ((next_type as u8) + 1) as char;
        }
    }
    State::new(rooms, hallway_length)
}

/// Instructions for one move, and for undoing the move.
struct Move {
    hallway_pos: usize,
    hallway_before: AmphipodType,
    hallway_after: AmphipodType,
    room_index: usize,
    room_before: (Option<AmphipodType>, Option<AmphipodType>),
    room_after: (Option<AmphipodType>, Option<AmphipodType>),
}

impl Move {
    fn find_move_home(state: &State, room_index: usize, hallway_pos: usize) -> Option<Move> {
        None
    }

    fn find_move_out(state: &State, room_index: usize, hallway_pos: usize) -> Option<Move> {
        let room = &state.rooms[room_index];
        if state.hallway.contents[hallway_pos] != '.' {
            None
        } else if !state
            .hallway
            .is_walkable_to(room.hallway_position, hallway_pos)
        {
            None
        } else if !room.needs_move_out() {
            None
        } else if let Some(a) = room.near_seat {
            Some(Move {
                hallway_pos,
                hallway_before: '.',
                hallway_after: a,
                room_index,
                room_before: (room.near_seat, room.far_seat),
                room_after: (None, room.far_seat),
            })
        } else if let Some(a) = room.far_seat {
            Some(Move {
                hallway_pos,
                hallway_before: '.',
                hallway_after: a,
                room_index,
                room_before: (None, room.far_seat),
                room_after: (None, None),
            })
        } else {
            // TODO: moving-out cases
            None
        }
    }

    fn cost(&self) -> usize {
        0
    }
}

/// Finds the lowest score that gets to the final state
fn search(state: &mut State) -> Option<usize> {
    if state.is_done() {
        Some(0)
    } else {
        // First, look and see if there's an amphipod that can move home.
        for room_index in 0..state.rooms.len() {
            for hallway_pos in 0..state.hallway.len() {
                if let Some(mov) = Move::find_move_home(state, room_index, hallway_pos) {
                    mov.apply(state);
                    let result = Some(mov.cost() + search(state));
                    mov.undo(state);
                    result
                }
            }
        }
        // Nobody could move home, try all the moves out to the hallway.
        let mut best_score: Option<usize> = None;
        for room in state.rooms.iter_mut() {
            if let Some((who, before)) = room.move_out() {
                for pos in 0..state.hallway.len() {
                    let before = state.hallway.contents[pos];
                    if before == '.' {
                        state.hallway.contents[pos] = who;

                        state.hallway.contents[pos] = before;
                    }
                }
                room.restore(before);
            }
        }
        best_score
    }
}

#[test]
fn test_search() {
    // Starting from an already-done state should be 0
    assert_eq!(
        Some(0),
        search(&mut parse_state(&[
            "#############",
            "#...........#",
            "###A#B#C#D###",
            "  #A#B#C#D#",
            "  #########"
        ]))
    );

    // Starting from a place where one needs to be moved home.
    // (answer does not include fixed cost)
    assert_eq!(
        Some(7),
        search(&mut parse_state(&[
            "#############",
            "#.........A.#",
            "###.#B#C#D###",
            "  #A#B#C#D#",
            "  #########"
        ]))
    );
}

fn day_23_a(lines: &[&str]) -> AdventResult<Answer> {
    let state = parse_state(lines);
    state.print();
    Ok(0)
}

fn day_23_b(_lines: &[&str]) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_23() -> Day {
    Day::new(
        23,
        DayPart::new(day_23_a, 0, 0),
        DayPart::new(day_23_b, 0, 0),
    )
}
