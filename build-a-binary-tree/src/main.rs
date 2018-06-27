struct Node<'a> {
    val: &'a str,
    l: Option<Box<Node<'a>>>,
    r: Option<Box<Node<'a>>>,
}

impl Node {
    pub fn insert(&self, new_val: &str) {
        if self.val == new_val { return; }

        let target_node = if new_val < self.val { &self.l } else { &self.r };

        if let &Some(sub_node) = target_node { sub_node.insert(new_val); } else {
            let new_node = Node {
                val: new_val,
                l: None,
                r: None,
            };
            let boxed_node = Some(Box::new(new_node));

            *target_node = boxed_node;
        }
    }
}
