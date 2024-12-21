use std::collections::HashMap;

const INPUT: [&str; 5] = [&"480A", &"143A", &"983A", &"382A", &"974A"];

fn main() {
    println!("Part 1: result={}", solve(2));
    println!("Part 2: result={}", solve(25));
}

// After RobotN has pressed a key, all the next ones are at 'A'.
//
// After a movement of a RobotN, RobotN+1 is placed at the key corresponding to that
// movement. RobotN+2 and nexts onwards are at 'A' because RobotN+1 has pressed an arrow.
//
// For example:
// - After each movement of Robot0 (the one at the door), Robot1 is at an arrow and
//   from Robot2 onwards are at 'A'.
// - After Robot0 has pressed a key, all the others are at 'A'.
//
// So, after pressing each key in the numpad, we start from a clean state of all the
// rest of robots. This implies that each of the actions of Robot0 (^>v<A) is done in
// the same way all the times. As the calculation over 25 levels of nesting is
// expensive, we can cache the results of previous calculations and save lot of
// computing time.
//
// To calculate the best route:
// - If there is a straight line, it's the best.
// - If not, move in an L shape. That means, changing direction a single time. This
//   is because each change in direction implies lot of movements in the whole chain
//   of robots, but continuing to move in the current direction only implies a single
//   press to 'A'.
// - When moving in an L shape we don't know what is cheaper: first vertical or first
//   horizontal. We compute both and keeps the cheapest.
fn solve(intermediate_robots: usize) -> usize {
    let mut result = 0;
    let mut cache = HashMap::new();

    for code in INPUT {
        let mut keys_count = 0;

        // Robot0 (the robot in the door) starts at 'A'.
        // Iterate all the keys that it needs to press. For each key, it has to move to it
        // from its previous position and press it.
        let mut key = 'A';
        for key_next in code.chars() {
            if let Some(val) = cache.get(&(0, key, key_next)) {
                // We already calculated the keys_count of this robot and movement
                keys_count += *val;
            } else {
                // Calculate the min number of robot actions needed to move from key to key_next.
                // There might be 1 or 2 different combinations of actions. An action can be
                // a movement or pressing the current key.
                let possible_actions = nums_pad_get_actions_to_move_and_press(key, key_next);
                let val = possible_actions
                    .into_iter()
                    .map(|robot_actions| count_keys(robot_actions, intermediate_robots, &mut cache))
                    .min()
                    .unwrap();
                cache.insert((0, key, key_next), val);
                keys_count += val;
            }

            key = key_next;
        }

        let code_num: usize = code[..3].parse().unwrap();
        result += keys_count * code_num;
    }

    result
}

fn count_keys(
    robot_actions: String,
    intermediate_robots: usize,
    cache: &mut HashMap<(usize, char, char), usize>,
) -> usize {
    // If there are no more intermediate robots, this is the human at the end of the
    // chain. Each action (movement or key press) that the robot has to make requires
    // only one key press from the human.
    if intermediate_robots == 0 {
        return robot_actions.len();
    }

    let mut keys_count = 0;

    // RobotN, a robot operated by another RobotN+1.
    // RobotN+1 starts at 'A'.
    // Iterate all the actions that RobotN needs to do. For each action, RobotN+1 has
    // to move from its previous position to the action's key and press it.
    let mut key = 'A';
    for key_next in robot_actions.chars() {
        if let Some(val) = cache.get(&(intermediate_robots, key, key_next)) {
            // We already calculated the keys_count of this robot and movement
            keys_count += *val;
        } else {
            // Calculate the min number of robot actions needed to move from key to key_next.
            // There might be 1 or 2 different combinations of actions. An action can be
            // a movement or pressing the current key.
            let possible_actions = arrows_pad_get_keys_to_move_and_press(key, key_next);
            let val = possible_actions
                .into_iter()
                .map(|robot1_actions| count_keys(robot1_actions, intermediate_robots - 1, cache))
                .min()
                .unwrap();
            cache.insert((intermediate_robots, key, key_next), val);
            keys_count += val;
        }

        key = key_next;
    }

    keys_count
}

fn nums_pad_get_actions_to_move_and_press(src: char, dst: char) -> Vec<String> {
    let src = nums_pad_get_pos(src);
    let dst = nums_pad_get_pos(dst);

    let up_down = match dst.1.cmp(&src.1) {
        std::cmp::Ordering::Greater => "v".repeat((dst.1 - src.1) as usize),
        std::cmp::Ordering::Equal => String::new(),
        std::cmp::Ordering::Less => "^".repeat((src.1 - dst.1) as usize),
    };
    let left_right = match dst.0.cmp(&src.0) {
        std::cmp::Ordering::Greater => ">".repeat((dst.0 - src.0) as usize),
        std::cmp::Ordering::Equal => String::new(),
        std::cmp::Ordering::Less => "<".repeat((src.0 - dst.0) as usize),
    };

    if up_down.is_empty() {
        vec![left_right + "A"]
    } else if left_right.is_empty() {
        vec![up_down + "A"]
    } else {
        if src.0 == 0 && dst.1 == 3 {
            // from left column to bottom row: don't go down&right to avoid the hole
            vec![left_right + &up_down + "A"]
        } else if src.1 == 3 && dst.0 == 0 {
            // from bottom row to left column: don't go left&up to avoid the hole
            vec![up_down + &left_right + "A"]
        } else {
            vec![
                left_right.clone() + &up_down + "A",
                up_down + &left_right + "A",
            ]
        }
    }
}

fn nums_pad_get_pos(key: char) -> (u32, u32) {
    match key {
        'A' => (2, 3),
        '0' => (1, 3),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        _ => panic!(),
    }
}

fn arrows_pad_get_keys_to_move_and_press(src: char, dst: char) -> Vec<String> {
    let src = arrows_pad_get_pos(src);
    let dst = arrows_pad_get_pos(dst);

    let up_down = match dst.1.cmp(&src.1) {
        std::cmp::Ordering::Greater => "v".repeat((dst.1 - src.1) as usize),
        std::cmp::Ordering::Equal => String::new(),
        std::cmp::Ordering::Less => "^".repeat((src.1 - dst.1) as usize),
    };
    let left_right = match dst.0.cmp(&src.0) {
        std::cmp::Ordering::Greater => ">".repeat((dst.0 - src.0) as usize),
        std::cmp::Ordering::Equal => String::new(),
        std::cmp::Ordering::Less => "<".repeat((src.0 - dst.0) as usize),
    };

    if up_down.is_empty() {
        vec![left_right + "A"]
    } else if left_right.is_empty() {
        vec![up_down + "A"]
    } else {
        if src.0 == 0 && dst.1 == 0 {
            // from left column to top row: don't go up&right to avoid the hole
            vec![left_right + &up_down + "A"]
        } else if src.1 == 0 && dst.0 == 0 {
            // from top row to left column: don't go left&down to avoid the hole
            vec![up_down + &left_right + "A"]
        } else {
            vec![
                left_right.clone() + &up_down + "A",
                up_down + &left_right + "A",
            ]
        }
    }
}

fn arrows_pad_get_pos(key: char) -> (u32, u32) {
    match key {
        'A' => (2, 0),
        '^' => (1, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        _ => panic!(),
    }
}
