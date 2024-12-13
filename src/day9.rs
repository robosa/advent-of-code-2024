use aoc_runner_derive::aoc;

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    let mut data: Vec<_> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
    let mut i = 0;
    let mut j = data.len() - 1;
    let mut empty = false;
    let mut res = 0;
    for k in 0.. {
        if empty {
            if data[j] == 0 {
                j -= 2;
                if j < i {
                    return res;
                }
            }
            res += k * (j / 2);
            data[j] -= 1;
            data[i] -= 1;
            if data[i] == 0 {
                empty = false;
                i += 1;
            }
        } else {
            res += k * (i / 2);
            data[i] -= 1;
            while data[i] == 0 {
                empty = !empty;
                i += 1;
            }
        }
    }
    panic!("Unreachable")
}

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    let mut files = Vec::new();
    let mut spaces = Vec::new();
    let mut index = 0;
    let mut res = 0;
    for (i, c) in input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
    {
        if i % 2 == 0 {
            files.push((index, c, i / 2));
        } else {
            spaces.push((index, c));
        }
        index += c;
    }
    for &(mut file_index, file_len, file_id) in files.iter().rev() {
        for space in spaces.iter_mut() {
            let (space_index, space_len) = *space;
            if space_index > file_index {
                break;
            }
            if file_len <= space_len {
                file_index = space_index;
                *space = (space_index + file_len, space_len - file_len);
                break;
            }
        }
        res += file_id * (file_index * file_len + file_len * (file_len - 1) / 2);
    }
    res
}
