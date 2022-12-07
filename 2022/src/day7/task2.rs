use crate::day7::task1::get_dir_sizes;
use itertools::Itertools;

pub fn solution(input: &str) -> usize {
    let dir_sizes = get_dir_sizes(input);
    let total_occupied_space = dir_sizes.get("/").unwrap();
    let fs_size = 70000000;
    let space_needed = 30000000;
    let space_to_free = space_needed + total_occupied_space - fs_size;
    *dir_sizes
        .iter()
        .sorted_by(|(_, size_a), (_, size_b)| size_a.cmp(size_b))
        .find(|(_, s)| **s >= space_to_free)
        .unwrap()
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            solution(
                "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"
            ),
            24933642
        );
    }
}
