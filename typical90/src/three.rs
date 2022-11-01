use proconio::input;

pub fn main() {
    input! {
        n: usize,
        a: [(usize,usize); n-1],
    }
    let mut adjascent = AdjacencyList::new(n);
    for i in a {
        adjascent.add_edge((i.0) - 1, (i.1) - 1);
    }
    println!("{}", adjascent.diameter() + 1);
}

pub struct AdjacencyList {
    dimension: usize,
    list: Vec<Vec<usize>>,
}

impl AdjacencyList {
    pub fn new(dimension: usize) -> Self {
        AdjacencyList {
            dimension,
            list: vec![vec![]; dimension],
        }
    }
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from >= self.dimension || to >= self.dimension {
            panic!("invalid vertex");
        }
        match self.list.get_mut(from) {
            Some(list) => list.push(to),
            None => self.list.insert(from, vec![to]),
        }
        match self.list.get_mut(to) {
            Some(list) => list.push(from),
            None => self.list.insert(to, vec![from]),
        }
    }
    fn distance(&self, pos: usize) -> Vec<isize> {
        let mut distance = vec![-1; self.dimension];
        let mut stack = vec![pos];
        distance[pos] = 0;
        while let Some(v) = stack.pop() {
            for &w in self.list[v].iter() {
                if distance[w] == -1 {
                    distance[w] = distance[v] + 1;
                    stack.push(w);
                }
            }
        }
        distance
    }
    pub fn diameter(&self) -> isize {
        let distance = self.distance(0);
        let (max_index, _) =
            distance
                .iter()
                .enumerate()
                .fold((0, 0), |(max_index, max), (index, &v)| {
                    if v > max {
                        (index, v)
                    } else {
                        (max_index, max)
                    }
                });

        let temp_distance = self.distance(max_index);
        let mut diameter = 0;
        for i in temp_distance {
            diameter = std::cmp::max(diameter, i);
        }
        diameter
    }
}
