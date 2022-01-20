#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            data: value,
            next: None,
        }
    }
}

fn get_list<T>(vals: Vec<T>) -> Option<Box<Node<T>>>
where
    T: Copy,
{
    if vals.is_empty() {
        return None;
    }
    let mut head: Option<Box<Node<T>>> = Some(Box::new(Node::new(vals[0])));
    for val in vals.iter().skip(1) {
        let mut new_head = Node::new(*val);
        new_head.next = Some(head.unwrap());
        head = Some(Box::new(new_head));
    }
    head
}

fn main() {
    // implementing linked list
    let mut node1 = Node::new(20);
    node1.next = None;
    node1.next = Some(Box::new(Node::new(21)));

    dbg!(node1);

    let linked_list = get_list(vec!["nachiket", "kanore", "hello", "world"]);
    dbg!(linked_list);

    let linked_list = get_list(vec![1, 2, 3, 4, 5, 6]);
    dbg!(linked_list);
}
