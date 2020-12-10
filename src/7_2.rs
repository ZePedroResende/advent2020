use petgraph::graphmap::GraphMap;
use petgraph::visit::{Bfs, DfsPostOrder};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Policy {
    graph: GraphMap<usize, usize, petgraph::Directed>,
    node_count: usize,
    count: usize,
    hash: HashMap<String, usize>,
}

impl Policy {
    fn new() -> Policy {
        Policy {
            hash: HashMap::new(),
            node_count: 0,
            graph: GraphMap::new(),
            count: 0,
        }
    }

    fn build_graph(&mut self) {
        let filename = "inputs/7.txt";

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

        /*
        lines
            .to_owned()
            .into_iter()
            .map(move |l| self.add_graph(l.trim()));
            */

        for l in lines {
            self.add_graph(l.trim());
        }
    }

    fn add_graph(&mut self, line: &str) {
        let key_re = Regex::new(r"(.*?) bags? contain (.*)").unwrap();
        let values_re = Regex::new(r"(\d) ([\w ]*) bags?").unwrap();

        let matched = key_re.captures(line).unwrap();

        let key = matched.get(1).unwrap().as_str().clone();
        let value = matched.get(2).unwrap().as_str().clone();

        if !self.hash.contains_key(key) {
            self.hash.insert(key.to_string(), self.node_count);
            self.graph.add_node(self.node_count);
            self.node_count += 1;
        }

        for m in values_re.captures_iter(value) {
            if !self.hash.contains_key(m.get(2).unwrap().as_str()) {
                self.hash
                    .insert(m.get(2).unwrap().as_str().into(), self.node_count);
                self.graph.add_node(self.node_count);
                self.node_count += 1;
            }

            self.graph.add_edge(
                *self.hash.get(key).unwrap(),
                *self.hash.get(m.get(2).unwrap().as_str()).unwrap(),
                m.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            );
        }
    }

    fn count(&mut self) -> usize {
        let id = self.hash.get("shiny gold".into()).unwrap();

        for n in self.graph.neighbors(*id) {
            self.count += self.graph.edge_weight(*id, n).unwrap() * self.count_id(&n);
        }

        self.count
    }

    fn count_id(&self, id: &usize) -> usize {
        let mut count = 1;
        for n in self.graph.neighbors(*id) {
            count += self.graph.edge_weight(*id, n).unwrap() * self.count_id(&n);
            //                   }
        }
        count
    }
}

fn main() {
    let mut graph = Policy::new();
    graph.build_graph();
    println!("{}", graph.count());
}
