pub fn _merge_sort(list: &mut Vec<i32>) {
    let length_of_list = list.len();

    if length_of_list > 1 {
        let mid = length_of_list / 2;

        let mut left_half = list[0..mid].to_vec();
        let mut right_half = list[mid..length_of_list].to_vec();

        let length_of_left_half = left_half.len();
        let length_of_right_half = right_half.len();

        merge_sort(&mut left_half);
        merge_sort(&mut right_half);

        let mut i = 0;
        let mut j = 0;
        let mut k = 0;

        while i < length_of_left_half && j < length_of_right_half {
            if left_half[i] < right_half[j] {
                list[k] = left_half[i];
                i += 1;
            }
            else {
                list[k] = right_half[j];
                j += 1;
            }
            k += 1;
        }
        while i < length_of_left_half {
            list[k] = left_half[i];
            i += 1;
            k += 1;
        }
        while j < length_of_right_half {
            list[k] = right_half[j];
            j += 1;
            k += 1;
        }
    }
}