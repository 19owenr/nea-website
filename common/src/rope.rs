#[derive(Clone)]
struct RopeNode {
    left: Option<Box<RopeNode>>,
    right: Option<Box<RopeNode>>,
    weight: usize,
    value: Option<String>,
}

#[derive(Clone)]
struct Rope {
    root: Option<Box<RopeNode>>,
}

impl Rope {
    const LEAF_SIZE: usize = 2;
    
    pub fn new(s: &str) -> Self {
        Self {
            root: Self::build_rope(s),
        }
    }

    fn build_rope(s: &str) -> Option<Box<RopeNode>> {
        if s.len() <= Self::LEAF_SIZE {
            return Some(Box::new(RopeNode {
                left: None,
                right: None,
                weight: s.len(),
                value: Some(s.to_string()),
            }));
        }

        let mid = s.len() / 2;
        let left_node = Self::build_rope(&s[..mid]);
        let right_node = Self::build_rope(&s[mid..]);

        Some(Box::new(RopeNode {
            left: left_node,
            right: right_node,
            weight: Self::rope_length_node(&left_node),
            value: None,
        }))
    }

    fn rope_length_node(node: &Option<Box<RopeNode>>) -> usize {
        match node {
            None => 0,
            Some(n) => {
                if let Some(ref v) = n.value {
                    v.len()
                } else {
                    Self::rope_length_node(&n.left)
                        + Self::rope_length_node(&n.right)
                }
            }
        }
    }

    pub fn length(&self) -> usize {
        Self::rope_length_node(&self.root)
    }

    pub fn char_at(&self, index: usize) -> Option<char> {
        Self::char_at_node(&self.root, index)
    }

    fn char_at_node(node: &Option<Box<RopeNode>>, index: usize) -> Option<char> {
        let n = node.as_ref()?;

        if let Some(ref v) = n.value {
            return v.chars().nth(index);
        }

        if index < n.weight {
            Self::char_at_node(&n.left, index)
        } else {
            Self::char_at_node(&n.right, index - n.weight)
        }
    }

    pub fn concat(self, other: Rope) -> Rope {
        let new_root = RopeNode {
            weight: Self::rope_length_node(&self.root),
            left: self.root,
            right: other.root,
            value: None,
        };

        Rope {
            root: Some(Box::new(new_root)),
        }
    }

    pub fn split(self, index: usize) -> (Rope, Rope) {
        let (left_node, right_node) = Self::split_node(self.root, index);
        (
            Rope { root: left_node },
            Rope { root: right_node },
        )
    }

    fn split_node(
        node: Option<Box<RopeNode>>,
        index: usize,
    ) -> (Option<Box<RopeNode>>, Option<Box<RopeNode>>) {
        let n = match node {
            None => return (None, None),
            Some(n) => n,
        };

        if let Some(value) = n.value {
            let left_str = value[..index.min(value.len())].to_string();
            let right_str = value[index.min(value.len())..].to_string();

            return (
                Self::build_rope(&left_str),
                Self::build_rope(&right_str),
            );
        }

        if index < n.weight {
            let (l, r) = Self::split_node(n.left, index);

            let new_right = RopeNode {
                left: r,
                right: n.right,
                weight: Self::rope_length_node(&r),
                value: None,
            };

            (l, Some(Box::new(new_right)))
        } else {
            let (l, r) =
                Self::split_node(n.right, index - n.weight);

            let new_left = RopeNode {
                left: n.left,
                right: l,
                weight: Self::rope_length_node(&n.left),
                value: None,
            };

            (Some(Box::new(new_left)), r)
        }
    }

    pub fn insert(&mut self, index: usize, s: &str) {
        let rope_clone = self.clone();
        let (left, right) = rope_clone.split(index);
        let middle = Rope::new(s);
        let result = left.concat(middle).concat(right);
        self.root = result.root;
    }

    pub fn delete(&mut self, start: usize, len: usize) {
        let rope_clone = self.clone();
        let (left, rest) = rope_clone.split(start);
        let ( _, right) = rest.split(len);
        let result = left.concat(right);
        self.root = result.root;
    }
}
