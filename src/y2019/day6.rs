use std::{collections::HashMap, ops::Deref};

pub fn solve(input: &str) -> (impl ToString, impl ToString) {
    let orbits = input
        .lines()
        .map(|l| l.split_once(')').unwrap())
        .collect::<Vec<_>>();
    let tree = Tree::from_orbits(&orbits);

    let mut queue = vec![tree.find_node("COM")];
    let mut part1 = 0;
    let mut depth = 0;
    while !queue.is_empty() {
        let mut new_queue = Vec::new();
        for i in queue {
            part1 += depth;
            for j in tree.children(i) {
                new_queue.push(*j);
            }
        }
        queue = new_queue;
        depth += 1;
    }

    let you_path = tree.path(tree.find_node("YOU"));
    let san_path = tree.path(tree.find_node("SAN"));
    let last_common = you_path
        .iter()
        .zip(san_path.iter())
        .enumerate()
        .find(|(_, (a, b))| a != b)
        .unwrap()
        .0;
    assert_eq!(you_path[last_common - 1], san_path[last_common - 1]);
    assert_ne!(you_path[last_common], san_path[last_common]);
    let part2 = you_path.len() + san_path.len() - 2 * last_common;

    (part1, part2)
}

#[derive(Debug)]
struct Tree<'a> {
    nodes: Vec<Node<'a>>,
    nodes_by_name: HashMap<&'a str, usize>,
}

#[derive(Debug, Clone)]
struct Node<'a> {
    #[allow(dead_code)]
    name: &'a str,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<'a> Tree<'a> {
    fn from_orbits(orbits: &[(&'a str, &'a str)]) -> Self {
        let mut nodes = Vec::new();
        let mut nodes_by_name: HashMap<&'a str, usize> = HashMap::new();
        for (parent, child) in orbits {
            let parent_idx = *nodes_by_name
                .entry(parent)
                .or_insert_with(|| {
                    let node = Node {
                        name: parent,
                        parent: None,
                        children: Vec::new(),
                    };
                    nodes.push(node);
                    nodes.len() - 1
                })
                .deref();
            let child_idx = *nodes_by_name
                .entry(child)
                .or_insert_with(|| {
                    let node = Node {
                        name: child,
                        parent: None,
                        children: Vec::new(),
                    };
                    nodes.push(node);
                    nodes.len() - 1
                })
                .deref();
            nodes[parent_idx].children.push(child_idx);
            assert!(nodes[child_idx].parent.is_none());
            nodes[child_idx].parent = Some(parent_idx);
        }
        Tree {
            nodes,
            nodes_by_name,
        }
    }

    fn find_node(&self, v: &str) -> usize {
        *self.nodes_by_name.get(v).unwrap()
    }

    fn children(&self, index: usize) -> &[usize] {
        &self.nodes[index].children
    }

    fn path(&self, index: usize) -> Vec<usize> {
        let mut path = Vec::new();
        let mut current = index;
        while let Some(parent) = self.nodes[current].parent {
            path.push(parent);
            current = parent;
        }
        path.reverse();
        path
    }
}
