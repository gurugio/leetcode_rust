pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let max: i32 = nums.len() as i32;
    let mut left: i32 = 0;
    let mut right: i32 = max - 1;
    let mut middle: i32 = 0;
    while left <= right {
        middle = (left + right) / 2;
        if left < 0 || right >= max {
            return -1;
        }
        if nums[middle as usize] == target {
            return middle as i32;
        } else if nums[middle as usize] < target {
            left = middle + 1;
        } else {
            right = middle - 1;
        }
    }

    if left > right {
        return -1;
    } else {
        return middle;
    }
}


