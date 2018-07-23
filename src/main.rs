#[derive(Debug, PartialEq)]
struct Node<'a> {
    val: &'a str,
    l: Option<Box<Node<'a>>>,
    r: Option<Box<Node<'a>>>,
}

impl<'a> Node<'a> {
    pub fn insert(&mut self, new_val: &'a str) {
        if self.val == new_val {
            return;
        }
            
        let target_node = if new_val < self.val {&mut self.l} else {&mut self.r};
        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(new_val),
            &mut None => {
                let new_node = Node {val: new_val, l: None, r: None};
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            },
        }
    }

    fn print(&self) {
        println!("{}", self.val);
    }

    // Print binary tree in a preorder way
    //   Preorder  (Root, Left, Right)
    //   Inorder   (Left, Root, Right)
    //   Postorder (Left, Right, Root)
    pub fn traverse(&self) {
        let mut vec = Vec::new();
        vec.push(self);

        while !vec.is_empty() {
            let temp_node = vec.pop();

            // Unpacks Some(Node{...})
            match temp_node {
                Some(ref node) => {
                    node.print();

                    // Avoiding moving 'node' by using reference pattern
                    match node.r {
                        Some(ref subnode) => vec.push(subnode),
                        None => (),
                    }

                    match node.l {
                        Some(ref subnode) => vec.push(subnode),
                        None => (),
                    }
                },
                None => (),
            }
        }
    }
}

fn main() {
    let mut x = Node {val: "H", l: None, r: None};
    x.insert("e");
    x.insert("n");
    x.insert("r");
    x.insert("i");
    x.insert("k");

    //println!("{:?}", x);
    x.traverse();
}
