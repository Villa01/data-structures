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
    println!("NULL");
}

