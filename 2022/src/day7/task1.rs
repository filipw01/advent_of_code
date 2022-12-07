use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct File {
    parent_dir: Option<String>,
    size: usize,
}

impl From<&str> for File {
    fn from(s: &str) -> Self {
        let size = s
            .split(' ')
            .next()
            .unwrap_or_else(|| panic!("Unable to parse file from: {}", s));
        File {
            parent_dir: None,
            size: size
                .parse()
                .unwrap_or_else(|_| panic!("Failed to parse size: {}", size)),
        }
    }
}

#[derive(Debug)]
struct Dir {
    parent_dir: Option<String>,
    name: String,
    files: Vec<File>,
    dirs: Vec<Dir>,
}

impl From<&str> for Dir {
    fn from(s: &str) -> Self {
        Dir {
            parent_dir: None,
            name: s.to_string(),
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }
}

struct Cwd {
    path: Vec<String>,
}

impl Cwd {
    fn new() -> Self {
        Cwd {
            path: vec!["/".to_string()],
        }
    }

    fn push(&mut self, dir: &str) {
        self.path.push(dir.to_string());
    }

    fn pop(&mut self) {
        self.path.pop();
    }

    fn get_path(&self) -> String {
        self.path.join("/").split_off(1)
    }

    fn get_current_dir<'a, 'b>(&'b self, tree: &'a mut Dir) -> &'a mut Dir {
        let mut current_dir = tree;
        for dir in self.path.iter().skip(1) {
            current_dir = current_dir
                .dirs
                .iter_mut()
                .find(|d| d.name == *dir)
                .unwrap();
        }
        current_dir
    }
}

pub fn solution(input: &str) -> usize {
    let dir_sizes = get_dir_sizes(input);
    dir_sizes
        .iter()
        .filter(|(_, size)| **size <= 100000)
        .map(|(_, s)| s)
        .sum()
}

pub fn get_dir_sizes(input: &str) -> HashMap<String, usize> {
    let mut cwd = Cwd::new();
    let mut tree = Dir::from("/");
    for line in input.lines() {
        let mut split = line.split(' ');
        let action_type = split.next().unwrap();
        match action_type {
            "$" => {
                if split.next().unwrap() == "cd" {
                    let cd_to = split.next().unwrap();
                    match cd_to {
                        "/" => cwd = Cwd::new(),
                        ".." => cwd.pop(),
                        _ => cwd.push(cd_to),
                    }
                }
            }
            "dir" => {
                let dir_name = split.next().unwrap();
                let mut dir = Dir::from(dir_name);
                dir.parent_dir = Some(cwd.get_path());
                let parent_dir = cwd.get_current_dir(&mut tree);
                parent_dir.dirs.push(dir);
            }
            _ => {
                let mut file = File::from(line);
                file.parent_dir = Some(cwd.get_path());
                let parent_dir = cwd.get_current_dir(&mut tree);
                parent_dir.files.push(file);
            }
        }
    }
    let mut dir_sizes = HashMap::new();
    let mut dirs_to_traverse = VecDeque::from([&tree]);
    while !dirs_to_traverse.is_empty() {
        let dir = dirs_to_traverse.pop_front().unwrap();
        let dir_path = if dir.name == "/" {
            "/".to_string()
        } else {
            format!(
                "{}/{}",
                dir.parent_dir.as_ref().unwrap_or(&"".to_string()),
                dir.name
            )
        };
        let dir_size = dir.files.iter().map(|f| f.size).sum::<usize>();
        dir_sizes.insert(dir_path, dir_size);
        for dir in dir.dirs.iter() {
            dirs_to_traverse.push_back(dir);
        }
    }

    let mut new_dir_sizes = dir_sizes.clone();
    dir_sizes
        .iter()
        .sorted_by(|(name_a, _), (name_b, _)| {
            name_b
                .chars()
                .filter(|c| *c == '/')
                .count()
                .cmp(&name_a.chars().filter(|c| *c == '/').count())
        })
        .for_each(|(subdir_name, subdir_size)| {
            dir_sizes
                .iter()
                .filter(|(name, _)| subdir_name.starts_with(*name) && *name != subdir_name)
                .for_each(|(name, _)| {
                    if let Some(new_dir_size) = new_dir_sizes.get_mut(name) {
                        *new_dir_size += subdir_size;
                    }
                });
        });
    new_dir_sizes
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
            95437
        );
    }
}
