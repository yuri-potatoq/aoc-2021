use utils::FileHolder;

const INPUT_NAME: &'static str = "input.txt";


pub fn measurements_increased(meass: Vec<u32>) -> u32 {    
    meass[0..]
        .iter()
        .zip(meass[1..].iter())
        .filter(|(a, b)| a < b)
        .count() as u32
}

#[cfg(test)]
mod tests {    
    use super::*;

    #[test]
    fn test_messurements_part_1() {
        let answer = 1477;

        let lines = FileHolder::build(INPUT_NAME)
            .unwrap()
            .list_lines();

        assert_eq!(answer, measurements_increased(lines));
    }
}