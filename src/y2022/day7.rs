use std::{collections::HashMap, str::FromStr};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let commands = CommandList::from_str(input).unwrap().0;

    let mut filesystem = FileSystem::default();
    for command in commands.iter() {
        filesystem.run_command(command);
    }

    let folder_sizes = filesystem.calculate_folder_sizes();

    let part1 = folder_sizes.values().filter(|v| **v <= 100000).sum::<u64>();

    let total_size = *folder_sizes.get(&filesystem.root).unwrap();
    let minimum_to_delete = total_size - (70000000 - 30000000);
    let mut candidates = folder_sizes
        .values()
        .filter(|v| **v >= minimum_to_delete)
        .collect::<Vec<_>>();
    candidates.sort();
    let part2 = *candidates[0];

    (part1, part2)
}

#[derive(Debug)]
struct FileSystem {
    folders: Vec<Folder>,
    root: FolderRef,
    cwd: FolderRef,
}

#[derive(Debug)]
struct Folder {
    name: String,
    files: Vec<File>,
    folders: Vec<FolderRef>,
    parent: Option<FolderRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct FolderRef(usize);

#[derive(Debug)]
struct File {
    size: u64,
    _name: String,
}

impl Default for FileSystem {
    fn default() -> Self {
        let root_folder = Folder::new("".to_string(), None);
        let root_folder_ref = FolderRef(0);
        let folders = vec![root_folder];
        Self {
            folders,
            root: root_folder_ref,
            cwd: root_folder_ref,
        }
    }
}

impl FileSystem {
    fn run_command(&mut self, command: &Command) {
        match command {
            Command::Up => self.cwd = self.cwd().parent.unwrap(),
            Command::Enter(name) => {
                self.cwd = *self
                    .cwd()
                    .folders
                    .iter()
                    .map(|fr| (fr, self.folder(*fr)))
                    .find(|(_, f)| &f.name == name)
                    .unwrap()
                    .0
            }
            Command::Root => self.cwd = self.root,
            Command::List(result) => {
                let mut new_files = Vec::new();
                let mut new_folders = Vec::new();
                for line in result {
                    let parts = line.split(' ').collect::<Vec<_>>();
                    match parts[0] {
                        "dir" => {
                            new_folders.push(self.create_folder(parts[1].to_string(), self.cwd))
                        }
                        _ => new_files.push(File {
                            size: parts[0].parse().unwrap(),
                            _name: parts[1].to_string(),
                        }),
                    }
                }
                let folder = self.cwd_mut();
                folder.files.extend(new_files);
                folder.folders.extend(new_folders);
            }
        }
    }

    fn create_folder(&mut self, name: String, parent: FolderRef) -> FolderRef {
        let folder_ref = FolderRef(self.folders.len());
        let folder = Folder::new(name, Some(parent));
        self.folders.push(folder);
        folder_ref
    }

    fn cwd(&self) -> &Folder {
        self.folder(self.cwd)
    }

    fn cwd_mut(&mut self) -> &mut Folder {
        self.folder_mut(self.cwd)
    }

    fn folder(&self, folder_ref: FolderRef) -> &Folder {
        &self.folders[folder_ref.0]
    }

    fn folder_mut(&mut self, folder_ref: FolderRef) -> &mut Folder {
        &mut self.folders[folder_ref.0]
    }

    fn calculate_folder_sizes(&self) -> HashMap<FolderRef, u64> {
        let mut queue = vec![self.root];
        let mut search_queue = Vec::new();
        while let Some(folder_ref) = queue.pop() {
            search_queue.push(folder_ref);
            let folder = self.folder(folder_ref);
            queue.extend(&folder.folders);
        }

        let mut folder_sizes = HashMap::new();
        while let Some(folder_ref) = search_queue.pop() {
            let folder = self.folder(folder_ref);

            let file_size_sum = folder.files.iter().map(|f| f.size).sum::<u64>();
            let folder_size_sum = folder
                .folders
                .iter()
                .map(|fr| folder_sizes.get(fr).unwrap())
                .sum::<u64>();

            let total_size = file_size_sum + folder_size_sum;
            folder_sizes.insert(folder_ref, total_size);
        }

        folder_sizes
    }
}

impl Folder {
    fn new(name: String, parent: Option<FolderRef>) -> Folder {
        Folder {
            name,
            files: Vec::new(),
            folders: Vec::new(),
            parent,
        }
    }
}

#[derive(Debug)]
enum Command {
    List(Vec<String>),
    Up,
    Enter(String),
    Root,
}

struct CommandList(Vec<Command>);

impl FromStr for CommandList {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().peekable();
        let mut commands = Vec::new();

        while let Some(command) = lines.next() {
            assert!(command.starts_with("$ "));
            let parts = command.split(' ').collect::<Vec<_>>();
            let command = match parts[1] {
                "ls" => {
                    let mut results = Vec::new();
                    while lines.peek().is_some() && !lines.peek().unwrap().starts_with('$') {
                        results.push(lines.next().unwrap().to_string());
                    }
                    Command::List(results)
                }
                "cd" if parts[2] == ".." => Command::Up,
                "cd" if parts[2] == "/" => Command::Root,
                "cd" => Command::Enter(parts[2].to_string()),
                _ => unreachable!(),
            };
            commands.push(command)
        }

        Ok(CommandList(commands))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_known() {
        let input = "$ cd /
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
7214296 k";

        let result = solve(input);

        assert_eq!(result.0.to_string(), 95437.to_string());
        assert_eq!(result.1.to_string(), 24933642.to_string());
    }
}
