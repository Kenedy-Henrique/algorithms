use std::convert::TryInto;

pub fn one_dimension(array: [u32; 1000]) {
    let mut is_peak = false;
    let mut element: usize;
    if array.len()%2 == 0 {
        element = array.len()/2;
    } else {
        let mut x: u32 = array.len().try_into().unwrap();
        x /= 2;
        element = x.try_into().unwrap();
    }
    while is_peak == false {
        if array[element] < array[element - 1] {
            if element == 1 {
                is_peak = true;
                println!("Peak = {}", array[element - 1]);
            } else {
                element -= 1;
            }
        } else if array[element] < array[element + 1] {
            if element == array.len() - 2{
                is_peak = true;
                println!("Peak = {}", array[element + 1]);
            } else {
                element += 1;
            }
        } else {
            is_peak = true;
            println!("Peak = {}", array[element]);
        }
    }
}