use itertools::Itertools;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let size = input.lines().count();
    let mut uf = UnionFind::new(size);

    for (id, line) in input.lines().enumerate() {
        // Use the fact each id comes on its own line, so we can ignore the first part and use the line number instead
        let neighbours = line
            .split(" <-> ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|n| n.parse::<usize>().unwrap());
        for nb in neighbours {
            uf.connect(id, nb);
        }
    }
    let part1 = uf.count(0);
    let part2 = (0..size).map(|n| uf.get_id(n)).sorted().unique().count();

    (part1.to_string(), part2.to_string())
}

struct UnionFind {
    nodes: Vec<usize>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            nodes: (0..size).collect(),
        }
    }
    fn get_id(&self, node: usize) -> usize {
        let mut id = node;
        while id != self.nodes[id] {
            id = self.nodes[id];
        }
        id
    }
    fn get_id_mut(&mut self, node: usize) -> usize {
        if self.nodes[node] == node {
            node
        } else {
            let id = self.get_id(node);
            self.nodes[node] = id;
            id
        }
    }
    fn connect(&mut self, a: usize, b: usize) {
        let id_a = self.get_id_mut(a);
        let id_b = self.get_id_mut(b);
        self.nodes[id_a] = id_b;
    }
    fn count(&self, node: usize) -> usize {
        let group_id = self.get_id(node);
        self.nodes
            .iter()
            .filter(|id| self.get_id(**id) == group_id)
            .count()
    }
}
