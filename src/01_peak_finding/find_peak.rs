use std::convert::TryInto;

pub fn one_dimensional(array: [u32; 1000]) {
    let mut is_peak = false;
    let mut element: usize;
    if array.len() % 2 == 0 {
        element = array.len() / 2;
    } else {
        let mut x: u32 = array.len().try_into().unwrap();
        x /= 2;
        element = x.try_into().unwrap();
    }
    while is_peak == false {

        /*
        This stops the code if the element
        is already on the last position of the
        left or right side of the row, this is
        made to not index non existent array
        positions
        */
        if element == 0 || element == array.len() - 1 {
            is_peak = true;
            println!("Peak = {}", array[element]);
            break;
        }

        if array[element] < array[element - 1] {
            element -= 1;
        } else if array[element] < array[element + 1] {
            element += 1;
        } else {
            is_peak = true;
            println!("Peak = {}", array[element]);
        }
    }
}

pub fn two_dimensional(square_matrix: [[u32; 4]; 4]) {
    let mut row_to_search: usize = (square_matrix.len() - 1) / 2;
    let mut column_to_search: usize = square_matrix[row_to_search].len()/2;
    let mut column_global_max: u32 = 0;
    let mut is_peak: bool = false;
    while is_peak == false {
        let mut i: usize = 0;
        while i < square_matrix.len() {
            if square_matrix[i][column_to_search] > column_global_max {
                column_global_max = square_matrix[i][column_to_search];
                row_to_search = i;
            } else {
                i += 1;
            }
        }

        /*
        This stops the code if the column
        is alredy on the last element of the
        left or right side of the row this is
        made to not index non existent array
        positions
        */
        if column_to_search == square_matrix[row_to_search].len() - 1 || column_to_search == 0 {
            println!("Peak = {}", column_global_max);
            is_peak = true;
            break;
        }

        if square_matrix[row_to_search][column_to_search - 1] > square_matrix[row_to_search][column_to_search] {
                column_to_search -= 1;
        } else if square_matrix[row_to_search][column_to_search + 1] > square_matrix[row_to_search][column_to_search] {
            column_to_search += 1;
        } else {
            is_peak = true;
            println!("Peak = {}", square_matrix[row_to_search][column_to_search]);
        }
    }
}