use std::fs::read_to_string;
use std::path::Path;

#[derive(PartialEq, Clone, Copy, Debug)]
struct File {
    id: i64,
    size: i64,
}

fn print_file_arr(files: &[File]) {
    for file in files {
        if file.id == -1 {
            print!("{}", ".".repeat(file.size as usize));
        } else {
            print!("{}", format!("{}", file.id).repeat(file.size as usize));
        }
    }
    println!()
}

pub fn solve<P>(input_file: P) -> u64
where
    P: AsRef<Path>,
{
    let input = read_to_string(input_file).unwrap();
    let mut expanded = input
        .trim()
        .char_indices()
        .map(|(idx, c)| File {
            id: if idx % 2 == 1 { -1i64 } else { idx as i64 / 2 },
            size: c.to_digit(10).unwrap() as i64,
        })
        .collect::<Vec<File>>();

    print_file_arr(&expanded);

    let mut right = expanded.len() - 1;
    while right > 0 {
        while expanded[right].id == -1 && right > 0 {
            right -= 1;
        }

        let mut left = 0;

        while left < right
            && (expanded[left].id != -1 || expanded[left].size < expanded[right].size)
        {
            left += 1;
        }
        let sizediff = expanded[left].size - expanded[right].size;
        if left < right {
            expanded.swap(left, right);
            // Add file with size diff back in
            if sizediff > 0 {
                expanded[right].size -= sizediff;
                expanded.insert(
                    left + 1,
                    File {
                        id: -1,
                        size: sizediff,
                    },
                );
                right += 1;
            }
        }
        right -= 1;
    }
    print_file_arr(&expanded);

    let mut block_id = 0;
    let mut res = 0;
    for file in expanded {
        if file.id != -1 {
            for i in 0..file.size {
                res += ((block_id + i) * file.id) as u64;
            }
        }
        block_id += file.size;
    }
    res
}
