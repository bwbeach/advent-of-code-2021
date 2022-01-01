//
// Day 23
//
// First, a couple observations...
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

use ndarray::{Array2, ArrayBase};

type State = Array2<u8>;
type Point = (usize, usize);

/// Information about the problem.
///
/// Extracted from the state once to avoid duplicate work.
///
struct Info {
    // The width of the grid.
    width: usize,

    // The height of the grid.
    height: usize,

    // The x coordinate of each room
    room_xs: Vec<usize>,

    // The x coordinates of the places in the hall where amphipods can stop
    hall_seat_xs: Vec<usize>,
}

fn cost_of_moving(amphipod_type: u8) -> usize {
    match amphipod_type {
        b'A' => 1,
        b'B' => 10,
        b'C' => 100,
        b'D' => 1000,
        _ => panic!("unknown amphipod type: {:?}", amphipod_type),
    }
}

fn is_amphipod(c: u8) -> bool {
    match c {
        b'A' => true,
        b'B' => true,
        b'C' => true,
        b'D' => true,
        _ => false,
    }
}

fn parse_state(lines: &[&str]) -> State {
    let width = lines[0].as_bytes().len();
    let height = lines.len();
    let mut result = ArrayBase::zeros((width, height));
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            result[(x, y)] = *c;
        }
    }
    result
}

fn print_state(state: &State) {
    let shape = state.shape();
    let width = shape[0];
    let height = shape[1];
    for y in 0..height {
        for x in 0..width {
            print!("{}", state[(x, y)] as char)
        }
        println!("");
    }
    println!("");
}

/// Are all the right amphipods in the room?
fn is_room_done(state: &State, amphipod_type: u8, room_x: usize, height: usize) -> bool {
    (2..(height - 1)).all(|y| state[(room_x, y)] == amphipod_type)
}

/// Are all the right amphipods in the room?
fn is_room_partly_done(state: &State, amphipod_type: u8, room_x: usize, height: usize) -> bool {
    (2..(height - 1)).all(|y| {
        let c = state[(room_x, y)];
        c == amphipod_type || c == b'.'
    })
}

/// Returns true iff all of the amphipods are in the right place
fn is_done(state: &State, info: &Info) -> bool {
    for (amphipod_type, room_x) in (b'A'..).zip(info.room_xs.iter()) {
        if !is_room_done(state, amphipod_type, *room_x, info.height) {
            return false;
        }
    }
    true
}

/// Is the path between a home location and a hallway location clear?
fn is_path_clear(
    home_location: (usize, usize),
    hall_location: (usize, usize),
    state: &State,
) -> bool {
    let (home_x, home_y) = home_location;
    let (hall_x, hall_y) = hall_location;
    let vertical_clear = (1..home_y).all(|y| state[(home_x, y)] == b'.');
    let mut hall_range = if hall_x < home_x {
        (hall_x + 1)..home_x
    } else {
        (home_x + 1)..hall_x
    };
    let hall_clear = hall_range.all(|x| state[(x, 1)] == b'.');
    vertical_clear && hall_clear
}

#[derive(Debug)]
struct Move {
    src: Point,
    dest: Point,
}

impl Move {
    fn apply(&self, state: &mut State) {
        state[self.dest] = state[self.src];
        state[self.src] = b'.';
    }

    fn undo(&self, state: &mut State) {
        state[self.src] = state[self.dest];
        state[self.dest] = b'.';
    }

    fn score(&self, amphipod_type: u8) -> usize {
        fn absdiff(a: usize, b: usize) -> usize {
            (((a as i64) - (b as i64)).abs()) as usize
        }
        let manhattan_distance =
            absdiff(self.src.0, self.dest.0) + absdiff(self.src.1, self.dest.1);
        cost_of_moving(amphipod_type) * manhattan_distance
    }
}

fn get_info(state: &State) -> Info {
    let shape = state.shape();
    let width = shape[0];
    let height = shape[1];
    let room_xs: Vec<_> = (0..width).filter(|x| state[(*x, 2)] != b'#').collect();
    let hall_seat_xs = (0..width)
        .filter(|x| state[(*x, 1)] != b'#')
        .filter(|x| !room_xs.contains(x))
        .collect();
    Info {
        width,
        height,
        room_xs,
        hall_seat_xs,
    }
}

// There's a place to move home to if the room for this amphipod
// type is either fully empty, or has the top seat empty and the
// bottom seat already holds the right type.
fn find_move_home_dest(
    state: &State,
    room_x: usize,
    amphipod_type: u8,
    info: &Info,
) -> Option<Point> {
    for y in (1..(info.height - 1)).rev() {
        let c = state[(room_x, y)];
        if c == b'.' {
            return Some((room_x, y));
        } else if c != amphipod_type {
            return None;
        }
    }
    panic!("should not happen");
}

fn find_move_home(state: &State, info: &Info) -> Option<Move> {
    for x in info.hall_seat_xs.iter() {
        let src = (*x, 1);
        let a = state[src];
        if is_amphipod(a) {
            let room_x = info.room_xs[(a - b'A') as usize];
            if let Some(dest) = find_move_home_dest(state, room_x, a, info) {
                if is_path_clear(dest, (*x, 1), state) {
                    return Some(Move { src, dest });
                }
            }
        }
    }
    None
}

fn find_move_to_hall_src(
    state: &State,
    room_x: usize,
    amphipod_type: u8,
    height: usize,
) -> Option<Point> {
    if is_room_partly_done(state, amphipod_type, room_x, height) {
        None
    } else {
        for y in 2..(height - 1) {
            if state[(room_x, y)] != b'.' {
                return Some((room_x, y));
            }
        }
        panic!("should not happen");
    }
}

fn find_move_to_hall_dest(state: &State, src: Point, hall_x: usize) -> Option<Point> {
    let dest = (hall_x, 1);
    if state[(dest)] != b'.' {
        None
    } else if is_path_clear(src, dest, state) {
        Some(dest)
    } else {
        None
    }
}

fn search_with_info(state: &mut State, info: &Info) -> Option<usize> {
    let room_xs = &info.room_xs;
    if is_done(state, info) {
        Some(0)
    } else if let Some(mov) = find_move_home(state, info) {
        let amphipod_type = state[mov.src];
        mov.apply(state);
        let score_of_rest = search_with_info(state, info);
        mov.undo(state);
        score_of_rest.map(|s| s + mov.score(amphipod_type))
    } else {
        let mut best_score = None;
        for (i, room_x) in room_xs.iter().enumerate() {
            let room_amphipod_type = b'A' + (i as u8);
            if let Some(src) =
                find_move_to_hall_src(state, *room_x, room_amphipod_type, info.height)
            {
                for hall_x in info.hall_seat_xs.iter() {
                    if let Some(dest) = find_move_to_hall_dest(state, src, *hall_x) {
                        let mov = Move { src, dest };
                        let moved = state[(src)];
                        let move_score = mov.score(moved);
                        mov.apply(state);
                        if let Some(rest_of_score) = search_with_info(state, info) {
                            let this_score = move_score + rest_of_score;
                            best_score = Some(
                                best_score.map_or(this_score, |s| std::cmp::min(s, this_score)),
                            );
                        }
                        mov.undo(state);
                    }
                }
            }
        }
        best_score
    }
}

fn search(state: &mut State) -> Option<usize> {
    let info = get_info(&state);
    search_with_info(state, &info)
}

#[test]
fn test_search() {
    assert_eq!(
        Some(0),
        search(&mut parse_state(&[
            "#############",
            "#...........#",
            "###A#B#C#D###",
            "  #A#B#C#D#",
            "  #########",
        ]))
    );

    assert_eq!(
        Some(8),
        search(&mut parse_state(&[
            "#############",
            "#.........A.#",
            "###.#B#C#D###",
            "  #A#B#C#D#",
            "  #########",
        ]))
    );

    assert_eq!(
        Some(4008),
        search(&mut parse_state(&[
            "#############",
            "#.....D...A.#",
            "###.#B#C#.###",
            "  #A#B#C#D#",
            "  #########",
        ]))
    );

    assert_eq!(
        Some(7008),
        search(&mut parse_state(&[
            "#############",
            "#.....D.D.A.#",
            "###.#B#C#.###",
            "  #A#B#C#.#",
            "  #########",
        ]))
    );

    assert_eq!(
        Some(7011),
        search(&mut parse_state(&[
            "#############",
            "#.....D.D...#",
            "###.#B#C#.###",
            "  #A#B#C#A#",
            "  #########",
        ]))
    );

    assert_eq!(
        Some(9011),
        search(&mut parse_state(&[
            "#############",
            "#.....D.....#",
            "###.#B#C#D###",
            "  #A#B#C#A#",
            "  #########",
        ]))
    );
}

fn day_23_a(lines: &[&str]) -> AdventResult<Answer> {
    let mut state = parse_state(lines);
    print_state(&state);
    Ok(search(&mut state).unwrap() as Answer)
}

fn day_23_b(_lines: &[&str]) -> AdventResult<Answer> {
    Ok(0)
}

pub fn make_day_23() -> Day {
    Day::new(
        23,
        DayPart::new(day_23_a, 12521, 17400),
        DayPart::new(day_23_b, 0, 0),
    )
}
