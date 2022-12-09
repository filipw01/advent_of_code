use itertools::Itertools;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

struct File {
    parent_dir: Option<DirHandle>,
    size: usize,
}

impl Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("File").field("size", &self.size).finish()
    }
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

type DirHandle = Rc<RefCell<Dir>>;

struct Dir {
    size: usize,
    parent_dir: Option<DirHandle>,
    name: String,
    files: Vec<File>,
    dirs: Vec<DirHandle>,
}

impl Debug for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dir")
            .field("size", &self.size)
            .field("name", &self.name)
            .field("files", &self.files)
            .field("dirs", &self.dirs)
            .finish()
    }
}

impl From<&str> for Dir {
    fn from(s: &str) -> Self {
        Dir {
            parent_dir: None,
            size: 0,
            name: s.to_string(),
            files: Vec::new(),
            dirs: Vec::new(),
        }
    }
}

impl Dir {
    fn add_size_to_parent(&mut self) {
        if let Some(parent) = self.parent_dir.as_ref() {
            parent.borrow_mut().size += self.size;
        }
    }
}

struct Cwd {
    path: Vec<DirHandle>,
}

impl Cwd {
    fn push(&mut self, dir: DirHandle) {
        self.path.push(dir);
    }

    fn pop(&mut self) {
        self.path.pop();
    }

    fn get_path(&self) -> String {
        self.path
            .iter()
            .map(|handle| {
                <Rc<RefCell<Dir>> as Borrow<RefCell<Dir>>>::borrow(handle)
                    .borrow()
                    .name
                    .clone()
            })
            .join("/")
            .split_off(1)
    }
}

impl From<DirHandle> for Cwd {
    fn from(dir: DirHandle) -> Self {
        Cwd { path: vec![dir] }
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
    let tree = Rc::from(RefCell::from(Dir::from("/")));
    let mut cwd = Cwd::from(tree.clone());
    let mut dir_handles = HashMap::from([(cwd.get_path(), tree.clone())]);
    for line in input.lines() {
        println!("dir_handles: {:?}", dir_handles);
        let mut split = line.split(' ');
        let action_type = split.next().unwrap();
        match action_type {
            "$" => {
                if split.next().unwrap() == "cd" {
                    let cd_to = split.next().unwrap();
                    match cd_to {
                        "/" => cwd = Cwd::from(tree.clone()),
                        ".." => cwd.pop(),
                        _ => {
                            let dir_name = format!("{}/{}", cwd.get_path(), cd_to);
                            if let Some(dir) = dir_handles.get(dir_name.as_str()) {
                                cwd.push(dir.clone());
                            } else {
                                panic!("Unable to find dir: {}", cwd.get_path());
                            }
                        }
                    }
                }
            }
            "dir" => {
                let dir_name = split.next().unwrap();
                let full_dir_name = format!("{}/{}", cwd.get_path(), dir_name);
                let dir = Rc::from(RefCell::from(Dir::from(dir_name)));
                dir_handles.insert(full_dir_name, dir.clone());
                if let Some(parent_dir) = dir_handles.get(&*cwd.get_path()) {
                    dir.borrow_mut().parent_dir = Some(parent_dir.clone());
                    parent_dir.borrow_mut().dirs.push(dir.clone());
                } else {
                    panic!("Unable to find dir: {}", cwd.get_path());
                }
            }
            _ => {
                let mut file = File::from(line);
                if let Some(parent_dir) = dir_handles.get(&*cwd.get_path()) {
                    file.parent_dir = Some(parent_dir.clone());
                    parent_dir.borrow_mut().size += file.size;
                    parent_dir.borrow_mut().files.push(file);
                } else {
                    panic!("Unable to find dir: {}", cwd.get_path());
                }
            }
        }
    }
    dir_handles
        .iter()
        .sorted_by(|(name_a, _), (name_b, _)| {
            name_b
                .chars()
                .filter(|c| *c == '/')
                .count()
                .cmp(&name_a.chars().filter(|c| *c == '/').count())
        })
        .for_each(|(_, dir)| {
            dir.borrow_mut().add_size_to_parent();
        });

    dir_handles
        .into_iter()
        .map(|(name, dir)| (name, dir.borrow_mut().size))
        .collect()
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
