pub struct Solution {}

impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        let mut box_types = box_types.clone();
        box_types.sort_by(|a, b| b[1].cmp(&a[1]));
        let mut total_units = 0;

        for b in box_types.iter() {
            if truck_size == 0 {
                break;
            }
            let (num_box, unit_per_box) = (b[0], b[1]);

            total_units += num_box.min(truck_size) * unit_per_box;
            truck_size -= num_box.min(truck_size);
        }

        total_units
    }
}
