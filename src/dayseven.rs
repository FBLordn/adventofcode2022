use std::fs;

fn main(){

}

fn part(){
    let _con = fs::read_to_string("numbers.txt").unwrap();
    let _path:Vec<char> = Vec::new();
}

struct TreeNode<'a>{
    pub size: i32,
    pub name: &'a str,
    pub children: Vec<TreeNode<'a>>,
}

impl <'a> TreeNode <'a>{
    pub fn new(psize: i32, pname: &str) -> TreeNode{
        return TreeNode{
            size: psize,
            name: pname,
            children: vec![],
        }
    }

    pub fn new_file(psize: i32, pname: &str) -> TreeNode{
        return TreeNode{
            size: psize,
            name: pname,
            children: vec![],
        }
    }

    pub fn add_dir(mut self, pname:&str) -> TreeNode{
        let new_node = TreeNode::new(0, pname);
        self.children.push(new_node);
        return new_node;
    }

    pub fn add_file(mut self, size: i32, pname:&'a str) -> TreeNode{
        let new_node = TreeNode::new_file(size, pname);
        self.children.push(new_node);
        return new_node;
    }

    pub fn get_with_name(self, name: &'a str) -> TreeNode{
        let knot = self;
        let kids = self.children;
        for child in kids{
            if name == child.name{
                return child;
            }
        }
        return knot;
    }

    pub fn get_from_path(self, path: Vec<&str>) -> TreeNode{
        let mut node:TreeNode = self;
        for chr in path{
            node = node.get_with_name(&chr);
        }
        return node;
    }
}