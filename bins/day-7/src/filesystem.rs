use core::panic;
use std::{collections::{HashMap, HashSet}, sync::atomic::{AtomicU32, Ordering}, hash::Hash};

static IdGen: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Id {
    File(u32),
    Dir(u32),
}

#[derive(Debug)]
pub struct File {
    id: Id,
    name: String,
    size: usize,
}

impl File {
    fn new(name: String, size: usize) -> Self {
        return File {
            id: Id::File(IdGen.fetch_add(1, Ordering::SeqCst)),
            name,
            size,
        };
    }
}

#[derive(Debug)]
pub struct Dir {
    id: Id,
    name: String,
    parent: Option<Id>,
    files: HashSet<Id>,
    dirs: HashSet<Id>,
    pub size: usize,
}

impl Dir {
    fn new(parent: Option<Id>, name: String) -> Self {
        return Dir {
            id: Id::Dir(IdGen.fetch_add(1, Ordering::SeqCst)),
            parent,
            name,
            files: HashSet::new(),
            dirs: HashSet::new(),
            size: 0,
        };
    } 
}

#[derive(Debug)]
pub struct FileSystem {
    pub files: HashMap<Id, File>,
    pub dirs: HashMap<Id, Dir>,
    cwd: Id,
}

impl FileSystem {
    pub fn new() -> Self {        
        let mut rootRoot = Dir::new(None, String::from(""));
        let root = Dir::new(Some(rootRoot.id), String::from("/"));
        rootRoot.dirs.insert(root.id);
        
        let mut fs = FileSystem {
            files: HashMap::new(),
            dirs: HashMap::new(),
            cwd: rootRoot.id,
        };

        fs.dirs.insert(rootRoot.id, rootRoot);
        fs.dirs.insert(root.id, root);
        return fs;
    }

    fn getDirFromId(&self, id: &Id) -> &Dir {
        return self.dirs.get(id).expect(&format!("Failed to find dir with id: {:?}", id));
    }

    fn getDirFromIdMut(&mut self, id: &Id) -> &mut Dir {
        return self.dirs.get_mut(id).expect(&format!("Failed to find dir with id: {:?}", id));
    }

    fn getFileFromId(&self, id: &Id) -> &File {
        return self.files.get(id).expect(&format!("Failed to find file with id: {:?}", id));
    }

    pub fn getDirSize(&self, name: &str) -> usize {
        return self.dirs.iter().find(|(_, dir)| dir.name == name).unwrap().1.size;
    }

    pub fn ensureDir(&mut self, name: &str) {
        let id = self.cwd;
        let cwd = self.getDirFromId(&id);

        if let None = cwd.dirs.iter().find(|child| self.getDirFromId(*child).name == name) {
            let child = Dir::new(Some(cwd.id), String::from(name));
            println!("Created directory {}", child.name);
            let mutCwd = self.getDirFromIdMut(&id);
            mutCwd.dirs.insert(child.id);
            self.dirs.insert(child.id, child);
        }
    }

    pub fn ensureFile(&mut self, name: &str, size: usize) {
        let id = self.cwd;
        let cwd = self.getDirFromId(&id);

        if let None = cwd.files.iter().find(|child| self.getFileFromId(*child).name == name) {
            let child = File::new(String::from(name), size);
            println!("Created file {}", child.name);
            let mutCwd = self.getDirFromIdMut(&id);
            mutCwd.files.insert(child.id);
            mutCwd.size += size;
            
            let mut id = mutCwd.parent;
            while let Some(parentId) = id {
                let mut parent = self.getDirFromIdMut(&parentId);
                parent.size += size;
                id = parent.parent;
            }

            self.files.insert(child.id, child);
        }
    }

    pub fn changeDir(&mut self, name: &str) {
        let dir = self.getDirFromId(&self.cwd);

        match name {
            ".." => self.cwd = match dir.parent { Some(id) => id, None => panic!("Cannot get to parent form root!")},
            childName => {
                match dir.dirs.iter().find(|childId| self.getDirFromId(*childId).name == childName) {
                    Some(childId) => self.cwd = *childId,
                    None => panic!("Failed to find child with name {name}"), 
                };
            }
        }
        println!("Changed directory to {}", self.getDirFromId(&self.cwd).name);
    } 

}