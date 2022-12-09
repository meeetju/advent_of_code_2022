use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use lazy_regex::{regex, Lazy, Regex};
use std::cell::RefCell;
use std::rc::Rc;

static CD_COMMAND: &Lazy<Regex> = regex!(r"^\$ cd ([.a-z/]*)$");
const GO_DIR_UP: &str = "..";
const GO_TO_ROOT: &str = "/";
const NONE: &str = "";
const LIST_DIRECTORY: &str = "$ ls";
const IS_DIRECTORY: bool = true;
const IS_FILE: bool = false;
const PART_1_MAX_LIMIT: u32 = 100000;
const TOTAL_DISK_SPACE: u32 = 70000000;
const REQUIRED_EMPTY_DISK_SPACE: u32 = 30000000;

#[derive(PartialEq, Debug)]
struct TreeNode {
  pub size: u32,
  pub name: Option<String>,
  pub children: Vec<Rc<RefCell<TreeNode>>>,
  pub parent: Option<Rc<RefCell<TreeNode>>>,
  pub is_directory: bool,
}

impl TreeNode {
    pub fn new(name: String, is_directory: bool) -> TreeNode {
      return TreeNode {
        size: 0,
        name: Some(name),
        children: vec![],
        parent: None,
        is_directory : is_directory,
      };
    }

    pub fn print(&self) -> String {
        if self.is_directory.clone() {
          return self.name.as_ref().unwrap().to_string().clone() + "[" 
            + &self
              .children
              .iter()
              .map(|tn| tn.borrow().print())
              .collect::<Vec<String>>()
              .join(",")
            + "]";
        } else {
            return self.name.as_ref().unwrap().to_string().clone() + " : " + &self.size.to_string();
        }
    }
}

#[warn(unused_variables)]
fn main() {

    let mut tree_nodes: Vec<Rc<RefCell<TreeNode>>> = vec![];

    let root_node = Rc::new(RefCell::new(TreeNode::new("/".to_string(), IS_DIRECTORY)));
    tree_nodes.push(root_node.clone());
    let mut current_directory: Rc<RefCell<TreeNode>> = root_node.clone();

    if let Ok(lines) = read_lines("./commands_2.txt") {
        for line in lines {
            if let Ok(value) = line {
                // Check if listing dir only
                match value.as_str() {
                    LIST_DIRECTORY => {continue;},
                    _ => {}
                }
                // Check command was executed
                match get_cd_command(&value).as_str(){
                    GO_DIR_UP => {
                        let old_current_directory = Rc::clone(&current_directory);
                        current_directory = Rc::clone(old_current_directory.borrow().parent.as_ref().unwrap());
                        continue;},
                    GO_TO_ROOT => {
                        current_directory = root_node.clone();
                        continue;},
                    NONE => {},
                    directory_name => {
                        // println!("Adding directory: {directory_name}");
                        let new_node = Rc::new(RefCell::new(TreeNode::new(directory_name.to_string(), IS_DIRECTORY)));
                        tree_nodes.push(new_node.clone());
                        new_node.as_ref().borrow_mut().parent = Some(current_directory.clone());
                        current_directory.as_ref().borrow_mut().children.push(new_node.clone());
                        current_directory = new_node;
                        // dbg!(&current_directory);
                        continue;}
                }
                // Element size and name
                let mut split = value.split(" ");
                let (size, name) = (split.next().unwrap(), split.next().unwrap());
                if !is_directory(size) {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(name.to_string(), IS_FILE)));
                    new_node.as_ref().borrow_mut().size = size.parse::<u32>().unwrap();
                    current_directory.as_ref().borrow_mut().children.push(new_node.clone());
                } 
            }
        }
    }
    for node in tree_nodes.iter() {
        node.as_ref().borrow_mut().size = dir_size_recursive(node.clone());
    }

    // for node in tree_nodes.iter() {
    //     println!("{:#?}", &node.borrow().print());
    // }

    let mut total: u32 = 0;
    for node in tree_nodes.iter() {
        // println!("{:#?}", &node.as_ref().borrow().name);
        // println!("{:#?}", &node.as_ref().borrow().size);
        if node.as_ref().borrow().size <= PART_1_MAX_LIMIT {
            total += &node.as_ref().borrow().size;
        }
    }
    println!("Answer part 1");
    println!("{}", total);
}

fn dir_size_recursive(dir: Rc<RefCell<TreeNode>>) -> u32 {
    let mut total_size: u32 = 0;
    for node in dir.as_ref().borrow_mut().children.iter() {
        if !node.as_ref().borrow().is_directory {
            total_size += node.as_ref().borrow().size;
        } else if node.as_ref().borrow().size != 0{
            total_size = node.as_ref().borrow().size;
        } else {
            total_size += dir_size_recursive(node.clone());
        }
    }
    total_size
}


fn is_directory(size: &str) -> bool {
    size == "dir"
}


fn get_cd_command(value: &str) -> String {
    let cd_command_result = CD_COMMAND.captures_iter(&value).collect::<Vec<regex::Captures>>();
    let cd_patameter = match cd_command_result.get(0) {
        Some(captures) => match captures.get(1) {
            Some(content) => Ok(value[content.start()..content.end()].to_string()),
            None => Err("Not found".to_string()),
        },
        None => Err("Not found".to_string()),
    };
    cd_patameter.unwrap_or_default()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
