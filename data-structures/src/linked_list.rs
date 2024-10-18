pub struct ListNode {
    data: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(value: i32) -> ListNode {
        ListNode {
            data: value,
            next: None,
        }
    }

    pub fn to_string(&self) -> String {
        format!("[{}]", self.data)
    }
}

pub fn print(head: &ListNode) {
    let mut cur = Some(head);

    while let Some(node) = cur {
        print!("{} -> ", node.to_string());
        cur = node.next.as_deref();
    }
    println!("None");
}

pub fn length(head: &ListNode) -> i32 {
    let mut cur = Some(head);
    let mut count = 0;
    while let Some(node) = cur {
        count += 1;
        cur = node.next.as_deref();
    }
    count
}

// Receives an option because the List could be empty
pub fn insert_at_beginning(head: Option<Box<ListNode>>, data: i32) -> Box<ListNode> {
    let mut new_node = Box::new(ListNode::new(data));

    match head {
        Some(node) => {
            new_node.next = Some(node);
            return new_node;
        }
        None => new_node,
    }
}

pub fn insert_at_end(mut head: Option<Box<ListNode>>, data: i32) -> Box<ListNode> {
    let new_node = Box::new(ListNode::new(data));

    match head {
        Some(ref mut node) => {
            let mut cur = node;
            while let Some(ref mut temp) = cur.next {
                cur = temp;
            }
            cur.next = Some(new_node);
            return head.unwrap();
        }
        None => new_node,
    }
}
