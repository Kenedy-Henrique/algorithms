use std::fmt::Debug;

pub fn sort<T: Copy + PartialOrd + Debug + std::fmt::Display>(
    unsorted_array_pointer: &mut [T],
) -> &[T] {
    if unsorted_array_pointer.len() <= 1 {
        return unsorted_array_pointer;
    } else {
        let unsorted_array_last_index = unsorted_array_pointer.len();
        let unsorted_array_middle_index = unsorted_array_last_index / 2;

        sort(&mut unsorted_array_pointer[0..unsorted_array_middle_index]);
        sort(&mut unsorted_array_pointer[unsorted_array_middle_index..unsorted_array_last_index]);

        let merged_array: Vec<T> = merge(
            &unsorted_array_pointer[0..unsorted_array_middle_index],
            &unsorted_array_pointer[unsorted_array_middle_index..unsorted_array_last_index],
        );
        //println!("Merged array: {:?}\n\n", merged_array);
        if unsorted_array_pointer.len() == merged_array.len() {
            unsorted_array_pointer.copy_from_slice(&merged_array);
        }
        return unsorted_array_pointer;
    }
}

pub fn merge<T: Copy + PartialOrd + Debug + std::fmt::Display>(
    array_to_merge_left_side: &[T],
    array_to_merge_right_side: &[T],
) -> Vec<T> {

    let left_side_last_index;
    if array_to_merge_left_side.len() == 0 {
        left_side_last_index = 0;
    } else {
        left_side_last_index = array_to_merge_left_side.len() - 1;
    }

    let right_side_last_index;
    if array_to_merge_right_side.len() == 0 {
        right_side_last_index = 0;
    } else {
        right_side_last_index = array_to_merge_right_side.len() - 1;
    }

    /*println!("Left side to merge: {:?}", array_to_merge_left_side);
    println!("Left side last index: {:?}", left_side_last_index);
    println!("Right side to merge: {:?}", array_to_merge_right_side);
    println!("Right side last index: {:?}", right_side_last_index);*/

    let mut current_left_side_index = 0;
    let mut current_right_side_index = 0;

    let mut new_array: Vec<T> = Vec::new();

    let mut is_already_all_left_array_elements_pushed = false;
    let mut is_already_all_right_array_elements_pushed = false;

    while is_already_all_left_array_elements_pushed == false || is_already_all_right_array_elements_pushed == false {
        if is_already_all_left_array_elements_pushed == false && array_to_merge_left_side[current_left_side_index] <= array_to_merge_right_side[current_right_side_index] {
            new_array.push(array_to_merge_left_side[current_left_side_index]);
            if current_left_side_index == left_side_last_index {
                is_already_all_left_array_elements_pushed = true;
            }
            if current_left_side_index < left_side_last_index {
                current_left_side_index += 1;
            }
            /*println!("Merging left side: {:?}", new_array);
            println!("Current left side index: {:?}", current_left_side_index);*/
            continue;
        }
        if is_already_all_right_array_elements_pushed == false && array_to_merge_right_side[current_right_side_index] <= array_to_merge_left_side[current_left_side_index] {
            new_array.push(array_to_merge_right_side[current_right_side_index]);
            if current_right_side_index == right_side_last_index {
                is_already_all_right_array_elements_pushed = true;
            }
            if current_right_side_index < right_side_last_index {
                current_right_side_index += 1;
            }
            /*println!("Merging right side: {:?}", new_array);
            println!("Current right side index: {:?}", current_right_side_index);*/
            continue;
        }
        if is_already_all_left_array_elements_pushed == true && is_already_all_right_array_elements_pushed == false {
            new_array.push(array_to_merge_right_side[current_right_side_index]);
            if current_right_side_index == right_side_last_index {
                is_already_all_right_array_elements_pushed = true;
            }
            if current_right_side_index < right_side_last_index {
                current_right_side_index += 1;
            }
            /*println!("Merging right side: {:?}", new_array);
            println!("Current right side index: {:?}", current_right_side_index);*/
            continue;
        }
        if is_already_all_right_array_elements_pushed == true && is_already_all_left_array_elements_pushed == false {
            new_array.push(array_to_merge_left_side[current_left_side_index]);
            if current_left_side_index == left_side_last_index {
                is_already_all_left_array_elements_pushed = true;
            }
            if current_left_side_index < left_side_last_index {
                current_left_side_index += 1;
            }
            /*println!("Merging left side: {:?}", new_array);
            println!("Current left side index: {:?}", current_left_side_index);*/
            continue;
        }
    }
    return new_array;
}