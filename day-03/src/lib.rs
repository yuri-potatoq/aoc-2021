#![allow(dead_code)]

const INPUT_NAME: &'static str = "input.txt";


fn find_most_comum_bit<T: std::cmp::Eq + Copy>(column: &Vec<T>) -> T {    
    
    let a = column.into_iter()
        .filter(|e| {
            column[0] == **e
        })
        .count();

    println!("{}", a);

    column[0]
}


fn transpose <T: Copy + std::fmt::Debug> (list: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if list.len() < 1 { println!("Returning itself"); return list.clone() }    

    let heads = |xss: &Vec<Vec<T>>| -> Vec<T> {
        xss.iter()
            .filter_map(|ls| ls.first().cloned())
            .collect::<Vec<T>>()    
    };

    let tails = |xss: &Vec<Vec<T>>| -> Vec<Vec<T>> {
        xss[1..].into_iter()
            .map(|e| e.clone())
            .collect()
    };

    // let mut rest = Vec::<Vec<T>>::new();

    // for x in 0..list[1].len() {
    //     rest.push(heads(
    //         &list[x..].into_iter()
    //             .map(|e| e.clone())
    //             .collect::<Vec<Vec<T>>>()
    //     ));     
    // }

    // rest
    vec![ heads(&list) ]
        .into_iter()
        .chain(transpose(&tails(&list)).into_iter())
        .collect::<Vec<Vec<T>>>()       
}

fn calculate_submarine_power(list: &Vec<String>) -> u32 {
    let lls = list.iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    
    println!("rows {} columns {}", lls.len(), lls[1].len());
    
    let vt = transpose(&lls);
    
    println!("rows {} columns {}", vt.len(), vt[1].len());    
    
    // let a = vt
    //     .iter()
    //     .map(|vc| { dbg!(vc.len()); find_most_comum_bit(vc)})
    //     .map(|c| c.to_digit(10).unwrap())
    //     .collect::<Vec<u32>>()        
    // ;

    

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

