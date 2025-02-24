enum Color {
    Red,
    Black,
}

struct Node<T: Ord> {
    color: Color,
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

struct RedBlackTree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree { root: None }
    }

    // call the recursive insert box function to perform
    // a generic self balancing insertion to the RBTree
    pub fn insert(&mut self, value: T) {
        self.root = Self::insert_box(self.root, value);
        if let Some(ref mut n) = self.root {
            n.color = Color::Black;
        }
    }

    // takes in a node option and a value and scans for the
    // value and inserts it to specific locations in the box
    // depending on the min.max value of the value to the node
    pub fn insert_box(node: Option<Box<Node<T>>>, value: T) {
        match node {
            Some(mut n) => {
                if value < n.value {
                    n.left = Self::insert_box(n.left.take(), value);
                } else if value > n.value {
                    n.right = Self::insert_box(n.right.take(), value);
                }
                Some(n)
            }
            None => Some(Box::new(Node {
                color: Color::Red,
                value: value,
                left: None,
                right: None,
            })),
        }
    }

    // if the tree leans towards the right mostly then perform
    // a left rotation to actually self balance the tree aptly
    pub fn rotate_left(mut node: Box<Node<T>>) -> Box<Node<T>>{
        if let Some(mut right_child) = node.right.take(){
            node.right = right_child.left.take();
            right_child.left = Some(node);
            return right_child;
        }
        node
    }

    // perform a right rotation of the tree to ensure that 
    // the tree self balances itself based on values and leaning
    pub fn rotate_right(mut node: Box<Node<T>>) -> Box<Node<T>> {
        if let Some(mut left_child) = node.left.take(){
            node.left = left_child.right.take();
            left_child.right = Some(node);
            return left_child;
        }
        node
    }

    pub fn fviolations(&mut self, node: &mut Box<Node<T>>) {
        let mut main_left = node.left.take();
        let mut main_right = node.right.take();
        let mut main_left_color = main_left.take().unwrap().color;
        let mut main_right_color = node.right.take().unwrap().color;
        if main_left_color == Color::Red && main_right_color == Color::Red{
            node.color = Color::Red;
            main_left_color = Color::Black;
            main_right_color = Color::Black;
        } else {
            if let Some(ref mut left_left) = main_left.take().unwrap().left{
                if left_left.color == Color::Black {
                    *node = Color::Black;
                    main_left.take().unwrap().color = Color::Black;
                    main_right.take().unwrap().color = Color::Red;
                }
            }
        }
    }

    // fix some violations that might occur in the course of
    // creating the tree such as consecutive red uncles and 
    // children nodes being all red
    pub fn fix_violations<T>(&mut self, node:&mut Box<Node<T>>){
        if let Some(ref mut left) = node.left {
            if left.color == Color::Red{
                // apply recoloring or rotations as needed
                // if left and right is red, then recolor them
                // to black and make the parent to be black
                if let Some(ref mut right) = node.right{
                    // uncle is red here in this case
                    if right.color == Color::Red {
                        node.color = Color::Red;
                        left.color = Color::Black;
                        right.color = Color::Black;
                    } else {
                        // uncle is black in this case
                        // incomplete implementation
                        if let Some(ref mut left_left) = left.left {
                            if left_left.color == Color::Black{
                                left.color = Color::Red;
                            }
                        }
                    }
                }
            }
        }
    }
}
