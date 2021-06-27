pub fn sort(mut array_to_sort: [u32; 400]) -> [u32; 400] {
    let mut index = 0;
    let mut temporary_valor;
    while index < 399 {
        if array_to_sort[index + 1] < array_to_sort[index] {
            temporary_valor = array_to_sort[index + 1];
            array_to_sort[index + 1] = array_to_sort[index];
            let mut actual_looking_index = index - 1;
            loop {
                if actual_looking_index == 0 {
                    array_to_sort[actual_looking_index] = temporary_valor;
                    break;
                } else if temporary_valor < array_to_sort[actual_looking_index - 1] {
                    array_to_sort[actual_looking_index] = array_to_sort[actual_looking_index - 1];
                    actual_looking_index -= 1;
                } else {
                    array_to_sort[actual_looking_index] = temporary_valor;
                    break;
                }
            }
        }
        index += 1;
    }
    return array_to_sort;
}