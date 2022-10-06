use std::collections::HashMap;

pub struct UnionFind {
    nodes: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            nodes: (0..size).collect(),
        }
    }
    pub fn add(&mut self) -> usize {
        // Add a disconnected node to the UF and return its ID
        self.nodes.push(self.nodes.len());
        self.nodes.len() - 1
    }
    pub fn get_id(&self, node: usize) -> usize {
        let mut id = node;
        while id != self.nodes[id] {
            id = self.nodes[id];
        }
        id
    }
    pub fn get_id_mut(&mut self, node: usize) -> usize {
        if self.nodes[node] == node {
            node
        } else {
            let id = self.get_id(node);
            self.nodes[node] = id;
            id
        }
    }
    pub fn connect(&mut self, a: usize, b: usize) -> Option<usize> {
        let id_a = self.get_id_mut(a);
        let id_b = self.get_id_mut(b);
        self.nodes[id_a] = id_b;
        Some(id_b)
    }
    pub fn count(&self, node: usize) -> usize {
        let group_id = self.get_id(node);
        self.nodes
            .iter()
            .filter(|id| self.get_id(**id) == group_id)
            .count()
    }
}

pub struct NamedUnionFind<K> {
    labels: HashMap<K, usize>,
    uf: UnionFind,
}

impl<K> NamedUnionFind<K>
where
    K: Eq,
    K: std::hash::Hash,
{
    pub fn new() -> Self {
        Self {
            labels: HashMap::new(),
            uf: UnionFind::new(0),
        }
    }
    pub fn add(&mut self, label: K) {
        // Adds a node identified by `label`
        // If such a node already exists, this is a no-op
        if !self.labels.contains_key(&label) {
            self.labels.insert(label, self.uf.add());
        }
    }
    pub fn get_id(&self, label: K) -> Option<usize> {
        let id = *self.labels.get(&label)?;
        Some(self.uf.get_id(id))
    }
    pub fn get_labels(&self) -> std::collections::hash_map::Keys<'_, K, usize> {
        self.labels.keys()
    }
    pub fn connect(&mut self, a: K, b: K) -> Option<usize> {
        let id_a = *self.labels.get(&a)?;
        let id_b = *self.labels.get(&b)?;
        self.uf.connect(id_a, id_b)
    }
}
