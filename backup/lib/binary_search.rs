pub fn _binary_search(list: &Vec<i32>, item: i32) -> usize {
    if list.len() > 0 {
        let mut max = list.len() - 1;
        let mut min = 0;

        while min <= max {
            let mid = (min + max) / 2;
            if list[mid] == item {
                return mid
            }
            else if list[mid] > item {
                max = mid - 1;
            }
            else {
                min = mid + 1;
            }
        }
    }
    1
}


pub fn _binary_lowest(list: &Vec<i32>, item: i32) -> usize {
    if list.len() > 0 {
        let mut max = list.len() - 1;
        let mut min = 0;

        while min <= max {
            let mid = (min + max) / 2;
            if list[mid] == item {
                if max == mid {
                    return mid
                }
                else {
                    max = mid - 1;
                }
            }
            else if list[mid] > item {
                max = mid - 1;
            }
            else {
                min = mid + 1;
            }
        }
        return min
    }
    0
}


pub fn _binary_highest(list: &Vec<i32>, item: i32) -> usize {
    if list.len() > 0 {
        let mut max = list.len() - 1;
        let mut min = 0;

        while min <= max {
            let mid = (min + max) / 2;
            if list[mid] == item {
                if max == mid {
                    return mid
                }
                else {
                    min = mid + 1;
                }
            }
            else if list[mid] > item {
                max = mid - 1;
            }
            else {
                min = mid + 1;
            }
        }
        return max
    }
    0
}

