
enum Color {
    Red,
    Black
}

struct Node<T:Ord>{
    color:Color,
    value:T,
    left:Option<Box<Node<T>>>,
    right:Option<Box<Node<T>>>
}

struct RedBlackTree<T:Ord>{
    root: Option<Box<Node<T>>>
}

impl <T:Ord> RedBlackTree<T> {
    pub fn new() -> Self {
        RedBlackTree {root: None}
    }

    pub fn insert(&mut self, value:T) {
        self.root = Self::insert_box(self.root, value);
        if let Some(ref mut n) = self.root {
            n.color = Color::Black;
        }
    }

    pub fn insert_box(node: Option<Box<Node<T>>>, value:T){
        match node {
            Some(mut n) => {
                if value < n.value {
                    n.left = Self::insert_box(n.left.take(), value);
                } else if value > n.value {
                    n.right = Self::insert_box(n.right.take(), value);
                }
                Some(n)
            },
            None => {
                Some(Box::new(Node{
                    color:Color::Red,
                    value:value,
                    left:None,
                    right:None
                }))
            }
        }
    }
}