
#[derive(Debug)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value :T) -> Node<T>{
        Node { value, left: None, right: None }
    }
}

impl<T> From<Node<T>> for Option<Box<Node<T>>> {
    fn from(node: Node<T>) -> Self {
        Some(Box::new(node))
    }
}

#[derive(Debug)]
struct  Tree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T : std::cmp::PartialOrd  + std::fmt::Debug + std::marker::Copy> Tree<T> {
    fn new() -> Self{
        Tree {
            root: None
        }
    }

    fn insert(&mut self , value :T){
        match &mut self.root {
            None => {
                self.root = Node::new(value).into()
            },
            Some(node) => {
                // do recersuve
                Tree::insert_recursive(node, value)
            }
        }
    }


    fn insert_recursive(node :&mut Box<Node<T>> , value : T){
        if value > node.value {
          match &mut node.right {
              None => {
                  node.right = Node::new(value).into();
              },
              Some(n) => {
                  Tree::insert_recursive(n, value);
              }
          }
        }
        
        else if value < node.value{
          match &mut node.left {
              None => {
                  node.left = Node::new(value).into();
              },
              Some(n) => {
                  Tree::insert_recursive(n, value);
              }
          }
        }
  
      }

    fn is_empty(&self) -> bool {
        match self.root {
            None => true,
            Some(_) => false
        }
    }

    fn find(&self , value : T) -> bool {
        Tree::find_recursive(&self.root , value)
    }

    fn find_recursive(node :&Option<Box<Node<T>>> , value : T) -> bool {

        if let None = node {
            return false;
        }

        let curr_node = node.as_ref().unwrap();
        let node_value = curr_node.value;

        if node_value == value {
            return true;
        }


        if value > node_value {
            return Tree::find_recursive(&curr_node.right, value)
        }
          
        else {
            return Tree::find_recursive(&curr_node.left, value)
        }

    }

}

fn main() {
    let mut tree = Tree::new();
    assert_eq!(tree.is_empty() , true);

    tree.insert(12);
    tree.insert(9);
    tree.insert(13);
    tree.insert(3);

    assert_eq!(tree.is_empty() , false);
    assert_eq!(tree.find(1) , false);
    assert_eq!(tree.find(3) , true);

    dbg!(tree );
}