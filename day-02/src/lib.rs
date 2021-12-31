#![allow(dead_code)]

const INPUT_NAME: &'static str = "input.txt";

fn matching_command(pattern_s: &String) -> (u32, u32) {
    match 
        &*pattern_s.split(" ").collect::<Vec<&str>>() 
    {
        // (cmd, unit)
        &["up", a] => (1, a.parse::<u32>().unwrap()),
        &["down", b] => (2, b.parse::<u32>().unwrap()),
        &["forward", c] => (3, c.parse::<u32>().unwrap()),
        _ => (0, 0)
    }
}

fn calculate_course_plane_part_1(cmds: &Vec<String>) -> u32 {
    let (horizontal, depth) = cmds.iter()
        .map(matching_command)
        .fold((0, 0), |acc, n| match n {
            // (horizontal, depth)
            (1, unit) => (acc.0, acc.1 - unit), // up 
            (2, unit) => (acc.0, acc.1 + unit), // down
            (3, unit) => (acc.0 + unit, acc.1), // forward
            _ => acc
        });

    horizontal * depth
}

fn calculate_course_plane_part_2(cmds: &Vec<String>) -> u32 {
    let (horizontal, depth, _) = cmds.iter()
        .map(matching_command)
        .fold((0, 0, 0), |acc, n| match n {
            // (horizontal, depth, aim)
            (1, unit) => (acc.0, acc.1, acc.2 - unit), // up 
            (2, unit) => (acc.0, acc.1, acc.2 + unit), // down
            (3, unit) => (acc.0 + unit, acc.1 + (acc.2 * unit), acc.2), // forward
            _ => acc
        });

    horizontal * depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_course_plane_part_1() {
        let answer = 1488669;

        let lines = utils::FileHolder::
            build(INPUT_NAME).unwrap()
            .list_lines();        
            
        let course_plane = calculate_course_plane_part_1(&lines);

        assert_eq!(answer, course_plane);
    }

    #[test]
    fn test_course_plane_part_2() {
        let answer = 1176514794;

        let lines = utils::FileHolder::
            build(INPUT_NAME).unwrap()
            .list_lines();        
            
        let course_plane = calculate_course_plane_part_2(&lines);

        assert_eq!(answer, course_plane);
    }
}
