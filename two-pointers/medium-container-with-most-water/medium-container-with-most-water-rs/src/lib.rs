// 4ms - 3MB
// shorter height -> need to change -> more chance of having a larger area
pub fn max_area(height: Vec<i32>) -> i32 {
    let mut max_area: usize = 0;
    let mut l: usize = 0;
    let mut r: usize = height.len() - 1;

    while l < r {
        let mut area = r - l;
        if height[r] < height[l] {
            area = area * height[r] as usize;
            r -= 1;
        } else {
            area = area * height[l] as usize;
            l += 1;
        }

        if area > max_area {
            max_area = area
        }
    }

    max_area as i32
}
