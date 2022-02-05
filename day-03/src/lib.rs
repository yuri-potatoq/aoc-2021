#![allow(dead_code)]

use std::ops::Deref;

const INPUT_NAME: &'static str = "input.txt";


fn find_most_comum_bit(column: &Vec<char>) -> char {
    ' '
}


fn transpose <T: std::ops::Deref> (list: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let first = |ls: &Vec<T>| *ls.first().unwrap();
    let last = |ls: &Vec<T>| *ls.last().unwrap();

    let heads = list.iter()
        .map(first)
        .collect::<Vec<T>>()
    ;

    let tails = list.iter()
        .map(last)
        .collect::<Vec<T>>()
    ;

    transpose(vec![heads, tails])
}

fn calculate_submarine_power(list: &Vec<String>) -> u32 {
    let lls = list.iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .map(|cs| {
            cs.iter()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    
    // transpose(lls)
    //     .iter()
    //     .map(|vc| find_most_comum_bit(vc))
    //     .map(|c| c.to_digit(10).unwrap())
    //     .collect::<Vec<u32>>()        
    // ;
    dbg!( &lls[0]);
    dbg!( &transpose(lls)[0] );

    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_submarine_power_comsumption() {
        let answer = 0;

        let lines = utils::FileHolder::
            build(INPUT_NAME)
            .expect("Error on opened file")
            .list_lines();        
            

        assert_eq!(answer, calculate_submarine_power(&lines));
    }
}

