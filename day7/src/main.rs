use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::time::Instant;

lazy_static! {
    static ref RE_LS: Regex = Regex::new(r"^\$ ls$").unwrap();
    static ref RE_LS_DIR: Regex = Regex::new(r"^dir (?P<name>[a-z]+)$").unwrap();
    static ref RE_LS_FILE: Regex = Regex::new(r"^(?P<size>[0-9]+) (?P<name>.+)$").unwrap();
    static ref RE_CD_X: Regex = Regex::new(r"^\$ cd (?P<name>[a-z]+)$").unwrap();
    static ref RE_CD_UP: Regex = Regex::new(r"^\$ cd ..$").unwrap();
    static ref RE_CD_ROOT: Regex = Regex::new(r"^\$ cd /$").unwrap();
}

// https://fasterthanli.me/series/advent-of-code-2022/part-7
type NodeHandle = Rc<RefCell<Node>>;

struct Node {
    name: String,
    size: usize,
    children: HashMap<String, NodeHandle>,
    parent: Option<NodeHandle>,
}

impl fmt::Debug for Node {
    // make sure we don't print the parent
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("name", &self.name)
            .field("size", &self.size)
            .field("children", &self.children)
            .finish()
    }
}

impl Node {
    fn new(name: &str, size: usize, parent: Option<NodeHandle>) -> Self {
        Node {
            name: name.to_string(),
            size: size,
            children: HashMap::new(),
            parent: parent,
        }
    }

    fn is_dir(&self) -> bool {
        self.size == 0 && !self.children.is_empty()
    }

    fn get_total_size(&self) -> usize {
        // is a file
        if self.size != 0 {
            return self.size;
        }
        // is an empty dir
        if self.children.len() == 0 {
            return 0;
        } else {
            // a dir with files
            return self
                .children
                .iter()
                .fold(0, |acc, (_, node)| node.borrow_mut().get_total_size() + acc);
        }
    }
}

fn all_dirs(n: NodeHandle) -> Box<dyn Iterator<Item = NodeHandle>> {
    let children: Vec<_> = n.borrow().children.values().cloned().collect();
    let it = std::iter::once(n).chain(
        children
            .into_iter()
            .filter(|child| Node::is_dir(&child.borrow()))
            .map(|child| all_dirs(child))
            .flatten(),
    );
    Box::new(it)
}

fn main() {
    let input = include_str!("input.txt");

    let filesystem = Rc::new(RefCell::new(Node::new("/", 0, None)));
    let mut current = filesystem.clone();

    // parse input
    for line in input.lines() {
        if RE_LS.is_match(line) {
            // do nothing
        }
        if RE_LS_DIR.is_match(line) {
            let cap = RE_LS_DIR.captures(line).unwrap();
            let name = cap.name("name").map(|name| name.as_str()).unwrap();

            let new_dir = Node::new(name, 0, Some(current.clone()));
            current
                .borrow_mut()
                .children
                .insert(name.to_owned(), Rc::new(RefCell::new(new_dir)));
        }
        if RE_LS_FILE.is_match(line) {
            let cap = RE_LS_FILE.captures(line).unwrap();
            let name = cap.name("name").map(|name| name.as_str()).unwrap();
            let size = cap.name("size").map(|name| name.as_str()).unwrap();

            let new_file = Node::new(name, size.parse().unwrap(), Some(current.clone()));
            current
                .borrow_mut()
                .children
                .insert(name.to_owned(), Rc::new(RefCell::new(new_file)));
        }
        if RE_CD_X.is_match(line) {
            let cap = RE_CD_X.captures(line).unwrap();
            let name = cap.name("name").map(|name| name.as_str()).unwrap();

            let new_current = current.borrow_mut().children.get(name).unwrap().clone();
            current = new_current;
        }
        if RE_CD_UP.is_match(line) {
            let new_current = current.borrow_mut().parent.clone().unwrap();
            current = new_current;
        }
        if RE_CD_ROOT.is_match(line) {
            current = filesystem.clone();
        }
    }

    // println!("{:#?}", filesystem);
    // println!("{}", filesystem.borrow_mut().get_total_size());

    let start = Instant::now();
    println!(
        "answer 1: {} {:?}",
        all_dirs(filesystem.clone())
            .map(|node| node.borrow().get_total_size())
            .filter(|size| size < &100_000)
            .sum::<usize>(),
        start.elapsed()
    );
}
