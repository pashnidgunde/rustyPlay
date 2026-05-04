pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize> {
    let x: usize = nums.len();
    let y: usize = nums.len();
    for i in 0..x {
        for j in 0..y {
            if nums[i] + nums[j] == target {
                return vec![i,j];
            }
        }
    }
    return vec![]
}
