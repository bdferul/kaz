use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum FileType {
    Dir,
    Doc(usize),
}

#[derive(Debug, Clone)]
struct FileData {
    pub me: String,
    pub parent: String,
    pub file_type: FileType,
}

impl FileData {
    fn new(me: &str, parent: &str, file_type: FileType) -> Self {
        Self {
            me: me.to_string(),
            parent: parent.to_string(),
            file_type,
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let tree = get_dir_hash_map(input);

    let mut dirs = tree
        .iter()
        .filter(|(_, x)| x.file_type == FileType::Dir)
        .map(|(s, _)| s)
        .collect::<Vec<&String>>();
    dirs.sort_by_key(|s| 0 - s.chars().filter(|c| c == &'/').count() as i32);

    let mut dir_sizes = HashMap::new();

    for s in dirs {
        let size = tree
            .iter()
            .filter(|(_, x)| x.parent == *s && x.me != *s)
            .fold(0, |acc, (_, x)| match x.file_type {
                FileType::Doc(n) => n + acc,
                FileType::Dir => {
                    acc + match dir_sizes.get(&x.me) {
                        Some(n) => n,
                        _ => panic!("{}", x.me),
                    }
                }
            });
        if dir_sizes.insert(s, size).is_some() {
            panic!("{s}")
        };
        //println!("{s}: {size}");
    }
    println!("{:?}", dir_sizes.get(&"root".to_string()).unwrap());
}

fn print_tree(tree: HashMap<String,FileData>) {

}

fn get_dir_hash_map(input: &str) -> HashMap<String, FileData> {
    let mut tree = std::collections::HashMap::new();
    tree.insert(
        "root".to_string(),
        FileData::new("root", "root", FileType::Dir),
    );
    let mut current_directory = tree.get(&"root".to_string()).unwrap().me.clone();
    let mut commands = input.split("$ ");
    commands.next();
    for s in commands {
        let mut words = s.split_ascii_whitespace();

        match words.next().unwrap().trim() {
            "cd" => match words.next().unwrap().trim() {
                "/" => current_directory = tree.get(&"root".to_string()).unwrap().me.clone(),
                ".." => current_directory = tree.get(&current_directory).unwrap().parent.clone(),
                other => {
                    let full_dir = format!("{}/{}", current_directory, other);
                    let tmp = tree.entry(full_dir.clone()).or_insert_with(|| {
                        FileData::new(&full_dir, &current_directory, FileType::Dir)
                    });
                    current_directory = tmp.me.clone();
                }
            },
            "ls" => {
                while let Some(thing) = words.next() {
                    match thing {
                        "dir" => {
                            words.next();
                        }
                        size => {
                            let file_name = words.next().unwrap();
                            let full_dir = format!("{}/{}", current_directory, file_name);
                            tree.insert(
                                full_dir.clone(),
                                FileData::new(
                                    &full_dir,
                                    &current_directory,
                                    FileType::Doc(size.parse::<usize>().unwrap()),
                                ),
                            );
                        }
                    }
                }
            }
            other => panic!("{}", other),
        }
        if s.split_ascii_whitespace().next().unwrap().trim() == "cd" {
            println!("{s}: {current_directory}");
        }
    }

    tree
}
