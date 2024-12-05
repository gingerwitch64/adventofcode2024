use std::io;
use std::io::Read;

fn main() {
    let mut buffer_string = String::new();
    io::stdin().read_to_string(&mut buffer_string).unwrap();

    println!("{}", buffer_string);

    let mut l_list: Vec<u64> = Vec::new();
    let mut r_list: Vec<u64> = Vec::new();

    for line in buffer_string.lines() {
        let str_parts: Vec<&str> = line.split_whitespace().collect();
        l_list.push(str_parts[0].parse().unwrap());
        r_list.push(str_parts[1].parse().unwrap());
    }

    println!("Part 1: {}", part_one(l_list.clone(), r_list.clone()));
    println!("Part 2: {}", part_two(l_list.clone(), r_list.clone()));
}

fn part_one(list_one: Vec<u64>, list_two: Vec<u64>) -> u64 {
    let mut list_a = list_one.clone();
    let mut list_b = list_two.clone();

    list_a.sort();
    list_b.sort();

    assert_eq!(list_a.len(), list_b.len());

    let mut diff_list: Vec<u64> = Vec::new();

    for i in 0..list_a.len() {
        let abs_diff = list_a[i].abs_diff(list_b[i]);
        diff_list.push(abs_diff);
    }

    let mut diff_sum: u64 = 0;
    for num in diff_list { diff_sum += num; }

    return diff_sum;
}

fn part_two(left_list: Vec<u64>, right_list: Vec<u64>) -> u64 {

    let mut similarity: u64 = 0;

    for l_num in left_list {
        let mut occur: u64 = 0;
        for r_num in &right_list {
            if l_num.eq(r_num) {
                occur += 1;
            }
        }
        similarity += l_num * occur;
    }

    return similarity;
}
