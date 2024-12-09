use std::{collections::BTreeMap, fs, io::Read};

fn main() {
    let data = read("input_1.txt");
    solve1(&data);
    solve2(&data);
}

fn read(p: &str) -> Vec<i64> {
    let mut s = String::new();
    fs::File::open(p).unwrap().read_to_string(&mut s).unwrap();

    s.lines().filter(|l| !l.is_empty()).collect::<Vec<&str>>()[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect()
}

fn solve1(data: &[i64]) {
    let mut disk = Vec::<i64>::new();

    // build disk
    let mut current_file_idx = 0;
    for (i, v) in data.iter().enumerate() {
        // file size
        if i % 2 == 0 {
            disk.append(&mut vec![current_file_idx; *v as usize]);
            current_file_idx += 1;
            continue;
        }

        // empty
        disk.append(&mut vec![-1; *v as usize]);
    }

    // println!("disk: {:?}", disk);

    let (mut l, mut r) = (0, disk.len() - 1);
    // find first empty
    loop {
        if disk[l] == -1 {
            break;
        }
        l += 1
    }
    // find last idx
    loop {
        if disk[r] != -1 {
            break;
        }
        r -= 1
    }

    while (l < r) && (l < disk.len() && r < disk.len()) {
        // swap
        disk[l] = disk[r];
        disk[r] = -1;

        // find first empty
        loop {
            if disk[l] == -1 {
                break;
            }
            l += 1
        }
        // find last idx
        loop {
            if disk[r] != -1 {
                break;
            }
            r -= 1
        }
    }

    let mut checksum = 0;
    for (i, v) in disk.iter().enumerate() {
        if *v == -1 {
            break;
        }
        checksum += i as i64 * *v;
    }

    println!("checksum: {}", checksum);
}

fn solve2(data: &[i64]) {
    let mut disk = BTreeMap::<i64, (i64, i64)>::new();
    let (mut current_idx, mut current_file_num) = (0, 0);
    for v in data.chunks(2) {
        let file_size = v[0];
        disk.insert(current_idx, (current_file_num, file_size));
        current_idx += file_size;
        current_file_num += 1;
        if v.len() == 2 {
            let free_size = v[1];
            disk.insert(current_idx, (-1, free_size));
            current_idx += free_size;
        }
    }

    let mut file_keys = vec![];
    for (k, v) in disk.iter() {
        if v.0 != -1 {
            file_keys.push(*k);
        }
    }

    for move_from in file_keys.iter().rev() {
        let (file_num, file_size) = match disk.get(move_from) {
            Some(v) => *v,
            None => {
                continue;
            }
        };

        // find empty space
        let mut move_to = -1;
        for (&k, (num, size)) in disk.iter() {
            if *num == file_num {
                break;
            }
            if *num != -1 {
                continue;
            }
            if *size < file_size {
                continue;
            }
            move_to = k;
            break;
        }
        // no avail space
        if move_to == -1 {
            continue;
        }

        let mut free_size_left = 0;
        let mut moved = false;
        match disk.get_mut(&move_to) {
            None => {}
            Some(v) => {
                v.0 = file_num;
                free_size_left = v.1 - file_size;
                v.1 = file_size;
                moved = true;
            }
        }
        if moved {
            match disk.get_mut(move_from) {
                None => {}
                Some(v) => {
                    v.0 = -1;
                }
            }
        }
        if free_size_left != 0 {
            disk.insert(move_to + file_size, (-1, free_size_left));
        }
    }

    let mut checksum = 0;
    for (k, v) in disk.iter() {
        if v.0 == -1 {
            continue;
        }
        for i in 0..v.1 {
            checksum += (k + i) * v.0
        }
    }

    println!("solve2: {}", checksum)
}
