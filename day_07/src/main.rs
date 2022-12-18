use slab::Slab;

struct FileSystemObject {
    name: String, // Debugging purposes only
    size: usize,
    children: Option<Vec<usize>>, // Only present for directories. Items are children's IDs.
    parent: Option<usize>,        // Only present for non-root objects
}

impl FileSystemObject {
    fn new_file(name: String, size: usize, parent: usize) -> Self {
        Self {
            name,
            size,
            children: None,
            parent: Some(parent),
        }
    }

    fn new_directory(name: String, parent: usize) -> Self {
        Self {
            name,
            size: 0,
            children: Some(Vec::new()),
            parent: Some(parent),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");
    let mut input_lines = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>());

    let mut file_system_objects = Slab::new();

    // Create root directory and skip first line
    let _ = input_lines.next();
    let root_directory = file_system_objects.insert(FileSystemObject {
        name: String::from("/"),
        size: 0,
        children: Some(Vec::new()),
        parent: None,
    });

    let mut current_directory = root_directory;

    for line in input_lines {
        match line[..] {
            ["$", "cd", ".."] => {
                current_directory = file_system_objects[current_directory]
                    .parent
                    .expect("Cannot `cd ..` from root directory");
            }
            ["$", "cd", dir] => {
                let new_directory = file_system_objects.insert(FileSystemObject::new_directory(
                    dir.to_string(),
                    current_directory,
                ));

                // Add new directory to current directory's children
                file_system_objects[current_directory]
                    .children
                    .as_mut()
                    .unwrap()
                    .push(new_directory);

                current_directory = new_directory;
            }
            ["$", "ls"] => {}
            ["dir", _dir_name] => {}
            [size, name] => {
                let size = size.parse::<usize>().expect("Could not parse size");

                // Create new file
                let new_file = file_system_objects.insert(FileSystemObject::new_file(
                    name.to_string(),
                    size,
                    current_directory,
                ));

                // Add new file to current directory's children
                file_system_objects[current_directory]
                    .children
                    .as_mut()
                    .unwrap()
                    .push(new_file);
            }
            _ => unreachable!("Invalid input line: {:?}", line),
        }
    }

    // Resolve directory sizes
    resolve_file_system_object_size(root_directory, &mut file_system_objects);

    // Part 1
    // Find all directories with size <= 100_000
    let mut part_1 = 0;
    let mut file_system_object_stack = vec![root_directory];
    while let Some(file_system_object) = file_system_object_stack.pop() {
        let Some(children) = &file_system_objects[file_system_object].children else {
            // Not a directory, skip
            continue;
        };

        file_system_object_stack.extend(children);

        let size = file_system_objects[file_system_object].size;

        if size <= 100_000 {
            part_1 += size;
        }
    }

    println!("Part 1: {part_1}");

    // Part 2
    // Find the smallest directory to delete to get 30_000_000 unused space assuming 70_000_000 total space
    let size_to_delete = 30_000_000 - (70_000_000 - file_system_objects[root_directory].size);

    let mut part_2 = usize::MAX;
    let mut file_system_object_stack = vec![root_directory];

    while let Some(file_system_object) = file_system_object_stack.pop() {
        let Some(children) = &file_system_objects[file_system_object].children else {
            // Not a directory, skip
            continue;
        };

        file_system_object_stack.extend(children);

        let size = file_system_objects[file_system_object].size;

        if size >= size_to_delete && size < part_2 {
            part_2 = size;
        }
    }

    println!("Part 2: {part_2}");

    Ok(())
}

fn resolve_file_system_object_size(root: usize, file_system_objects: &mut Slab<FileSystemObject>) {
    if let Some(children) = file_system_objects[root].children.clone()
    /* Make borrow checker happy */
    {
        for child in children {
            resolve_file_system_object_size(child, file_system_objects);
            file_system_objects[root].size += file_system_objects[child].size;
        }
    }
}
