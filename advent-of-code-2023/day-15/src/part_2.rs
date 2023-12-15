#[derive(Clone, Debug)]
struct Len {
    label: String,
    focal_length: usize,
}

pub fn solve(input: &str) -> usize {
    let mut boxes: Vec<Vec<Len>> = vec![Vec::new(); 256];
    let operations: Vec<&str> = input.split(',').collect();

    for op in operations {
        // for (i, b) in boxes.iter().enumerate() {
        //     if b.is_empty() {
        //         continue;
        //     }
        //     println!("{}: {:?}", i, b);
        // }
        // println!("[{}]", op);

        if op.ends_with('-') {
            let label = op.trim_end_matches('-');
            let box_number = hash(label);
            // println!("{} | {} | {}", op, label, box_number);

            let mut len_idx: Option<usize> = None;
            for (i, l) in boxes[box_number].iter().enumerate() {
                if l.label.eq(&label) {
                    len_idx = Some(i);
                    break;
                }
            }
            if len_idx.is_none() {
                continue;
            }
            let len_idx = len_idx.unwrap();
            for i in (1..=len_idx).rev() {
                boxes[box_number][i] = boxes[box_number][i - 1].clone();
            }
            boxes[box_number] = boxes[box_number][1..].to_vec()
        } else {
            let (label, focal_length) = op.split_once('=').unwrap();
            let focal_length: usize = focal_length.parse().unwrap();
            let box_number = hash(label);

            // println!("{} | {} | {} | {}", op, label, box_number, focal_length);

            let mut len_idx: Option<usize> = None;
            for (i, l) in boxes[box_number].iter().enumerate() {
                if l.label.eq(&label) {
                    len_idx = Some(i);
                    break;
                }
            }
            if len_idx.is_none() {
                boxes[box_number].push(Len {
                    label: label.to_string(),
                    focal_length,
                });
                continue;
            }
            let len_idx = len_idx.unwrap();
            // println!("{}", len_idx);
            boxes[box_number][len_idx] = Len {
                label: label.to_string(),
                focal_length,
            }
        }
    }

    for (i, b) in boxes.iter().enumerate() {
        if b.is_empty() {
            continue;
        }
        println!("{}: {:?}", i, b);
    }

    let mut sum = 0;
    for (b_idx, b) in boxes.iter().enumerate() {
        for (l_idx, l) in b.iter().enumerate() {
            sum += (b_idx + 1) * (l_idx + 1) * l.focal_length
        }
    }

    sum
}

fn hash(input: &str) -> usize {
    let mut current = 0;
    for c in input.chars().filter(|&c| c != '\n') {
        // println!("current - 1 {}", current);
        current += c as usize;
        // println!("current - 2 {}", current);
        current *= 17;
        // println!("current - 3 {}", current);
        current %= 256;
        // println!("current - 4 {}", current);
    }

    current
}

mod test {
    use super::hash;

    #[test]
    fn test_hash() {
        println!("{}", hash("rn"))
    }
}
