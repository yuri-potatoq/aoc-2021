#![allow(dead_code)]

const INPUT_NAME: &'static str = "input.txt";

fn measurements_increased(meass: Vec<u32>) -> u32 {
    meass[0..]
        .iter()
        .zip(meass[1..].iter())
        .filter(|(a, b)| a < b)
        .count() as u32
}

//  sliding window technique/algorithm
fn sliding_windows(list: &Vec<u32>, size_window: usize) -> Vec<u32> {
    (0..(list.len() - size_window + 1))
        .map(|i| &list[i..(i + size_window)])
        .map(|x| x.iter().fold(0, |acc, x| acc + x))
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_messurements_part_1() {
        let answer = 1477;

        let lines = utils::FileHolder::
            build(INPUT_NAME).expect("Error on opened file")
            .list_lines_by_int();

        assert_eq!(answer, measurements_increased(lines));
    }

    #[test]
    fn test_messurements_part_2() {
        let answer = 1523;

        let lines = utils::FileHolder::
            build(INPUT_NAME).expect("Error on opened file")
            .list_lines_by_int();

        let windows_sum = sliding_windows(&lines, 3);
        
        assert_eq!(answer, measurements_increased(windows_sum));
    }
}
