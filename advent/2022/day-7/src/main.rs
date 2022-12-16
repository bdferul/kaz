#[derive(Debug, Clone, Copy)]
enum FileType {
    Dir,
    File(usize),
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
    let mut tree = std::collections::HashMap::new();
    tree.insert(
        "root".to_string(),
        FileData::new("root", "root", FileType::Dir),
    );
    let mut current_directory = tree.get(&"root".to_string()).unwrap().me.clone();
    let input = include_str!("test.txt");
    let mut commands = input.split("$ ");
    commands.next();
    for s in commands {
        let mut words = s.split_ascii_whitespace();

        match words.next().unwrap().trim() {
            "cd" => match words.next().unwrap().trim() {
                "/" => current_directory = tree.get(&"root".to_string()).unwrap().me.clone(),
                ".." => current_directory = tree.get(&current_directory).unwrap().parent.clone(),
                other => {
                    dbg!(other);
                    let full_dir = format!("{}/{}", current_directory, other);
                    println!("{}", full_dir);
                    let tmp = tree.entry(full_dir.clone()).or_insert(FileData::new(
                        &full_dir,
                        &current_directory,
                        FileType::Dir,
                    ));
                    current_directory = tmp.me.clone();
                }
            },
            "ls" => {
                while let Some(thing) = words.next() {
                    match thing {
                        "dir" => {words.next();},
                        size => {
                            let file_name = words.next().unwrap();
                            let full_dir = format!("{}/{}", current_directory, file_name);
                            dbg!(&size,&full_dir,&file_name);
                            tree.insert(
                                full_dir.clone(),
                                FileData::new(
                                    &full_dir,
                                    &current_directory,
                                    FileType::File(size.parse::<usize>().unwrap()),
                                ),
                            );
                        }
                    }
                }
            }
            other => panic!("{}", other),
        }

        dbg!(&tree);
    }
}
