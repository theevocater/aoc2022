use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::BufRead;

enum Filesystem {
    File {
        name: String,
        size: usize,
    },
    Directory {
        children: HashMap<String, Filesystem>,
        parent: Option<Box<Filesystem>>,
    },
}

fn main() -> anyhow::Result<()> {
    let tokens = Regex::new(
        &[
            r"(?P<cd>\$ cd (?P<cdir>.*))",
            r"(?P<ls>\$ ls)", // Included but does nothing
            r"(?P<dir>dir (?P<dir_name>.*))",
            r"(?P<file>(?P<size>\d+) (?P<name>.+))",
        ]
        .join("|"),
    )?;
    let mut fs_root = io::stdin().lock().lines().flatten().fold(
        Filesystem::Directory {
            children: HashMap::new(),
            parent: None,
        },
        |fs, input| {
            // Build the directory tree
            let captures = tokens.captures(input.as_str()).unwrap();
            // / => ignore
            // .. => return fs.parent
            // _ => return fs.children[_]
            match captures.name("cd") {
                Some(_) => println!("{:?}", captures.name("cdir").unwrap().as_str(),),
                None => (),
            };
            // Create a new empty dir subchild of 'fs' with parent = fs
            match captures.name("dir") {
                Some(_) => println!("{:?}", captures.name("dir_name").unwrap().as_str(),),
                None => (),
            };
            // Create a new file child
            match captures.name("file") {
                Some(_) => println!(
                    "{:?} {:?}",
                    captures.name("name").unwrap().as_str(),
                    captures.name("size").unwrap().as_str(),
                ),
                None => (),
            };
            fs
        },
    );
    Ok(())
}
