use std::fmt::Display;

type NodePointer<T> = Box<Node<T>>;

struct NodeValue<T> {
    value: T,
    next: NodePointer<T>,
}

impl<T> NodeValue<T> {
    fn new(value: T, next: NodePointer<T>) -> NodeValue<T> {
        NodeValue {
            value: value,
            next: next,
        }
    }
}

enum Node<T> {
    Empty,
    NotEmpty(NodeValue<T>),
}

impl<T> Node<T> {
    fn new(value: T, next: NodePointer<T>) -> Node<T> {
        Node::NotEmpty(NodeValue::new(value, next))
    }

    fn get_node(&mut self) -> Self {
        let mut cur = Self::Empty;
        std::mem::swap(&mut cur, self);
        cur
    }
}

pub struct LinkedList<T> {
    head: NodePointer<T>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: Box::new(Node::Empty),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn add_at_beginning(&mut self, value: T) {
        let curr_head = self.head.get_node();
        let new_node = Node::new(value, Box::new(curr_head));
        self.head = Box::new(new_node);
        self.len += 1;
    }

    pub fn remove_at_beginning(&mut self) -> Option<T> {
        let curr_head = self.head.get_node();
        if let Node::NotEmpty(curr_head) = curr_head {
            self.head = curr_head.next;
            self.len -= 1;
            Some(curr_head.value)
        } else {
            None
        }
    }
}

// Consumes the existing list
impl<T> Iterator for LinkedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let curr_head = self.remove_at_beginning();

        curr_head
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut curr_node = self.head.as_ref();
        let mut linked_list_string = String::new();

        loop {
            match curr_node {
                Node::Empty => break,
                Node::NotEmpty(node) => {
                    // If next node is empty just print without comma
                    if let Node::NotEmpty(_) = node.next.as_ref() {
                        linked_list_string.push_str(&format!("{}, ", node.value));
                    } else {
                        linked_list_string.push_str(&format!("{}", node.value));
                    }
                    curr_node = &node.next;
                }
            }
        }

        write!(f, "LinkedList[{linked_list_string}]")
    }
}
