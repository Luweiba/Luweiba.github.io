#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Display> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            elem,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkList<T> {
    head: Option<Box<Node<T>>>,
    cnt: usize,
    index: usize,
}

impl<T: std::fmt::Display> LinkList<T> {
    fn new() -> Self {
        Self {
            head: None,
            cnt: 0,
        }
    }
    
    fn push(&mut self, elem: T) {
        self.cnt += 1;
        let node = Box::new(Node::new(elem));
        if self.head.is_none() {
            self.head = Some(node);
            return;
        }
        let mut curr_node = self.head.as_mut().unwrap();
        loop {
            if curr_node.next.is_none() {
                curr_node.next = Some(node); 
                return;
            } else {
                curr_node = curr_node.next.as_mut().unwrap();
            }
        }
    }
    
    fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        let mut front = self.head.take().unwrap();
        self.head = front.next.take();
        self.cnt -= 1;
        return Some(front.elem);
    }
}

fn main() {
    let mut list = LinkList::new();
    for i in 0..10 {
        list.push(i.to_string());
        println!("{:?}", list);
    }
    while let Some(val) = list.pop() {
        println!("Pop: {}", val);
    }
}
