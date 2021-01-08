use std::collections::HashSet;
use std::io::{self, BufRead as _};

type BoxError = Box<dyn std::error::Error>;

#[derive(Debug)]
struct Edge<T> {
    from: usize,
    to: usize,
    data: T,
}

#[derive(Debug)]
struct Graph<T> {
    vertices: Vec<String>,
    edges: Vec<Edge<T>>,
}

impl<T> Graph<T> {
    fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn insert_vertex(&mut self, vertex: &str) {
        let vertex = vertex.into();
        if !self.vertices.contains(&vertex) {
            self.vertices.push(vertex);
        }
    }

    fn insert_edge(&mut self, from: &str, to: &str, data: T) -> Result<(), BoxError> {
        let from_i = self
            .vertices
            .iter()
            .position(|x| x == from)
            .ok_or("from vertex not found")?;
        let to_i = self
            .vertices
            .iter()
            .position(|x| x == to)
            .ok_or("to vertex not found")?;
        self.edges.push(Edge {
            from: from_i,
            to: to_i,
            data,
        });
        Ok(())
    }

    fn find_ancestors(&self, vertex: &str) -> Vec<&str> {
        if let Some(vertex_i) = self.vertices.iter().position(|x| x == vertex) {
            self.edges
                .iter()
                .filter(|edge| edge.to == vertex_i)
                .map(|edge| self.vertices[edge.from].as_str())
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug)]
struct BagData {
    contain_count: usize,
}

fn parse_rule(rule: &str) -> Result<(String, Vec<(usize, String)>), BoxError> {
    let parts = rule
        .strip_suffix(".")
        .ok_or("missing full stop")?
        .split(" bags contain ")
        .collect::<Vec<_>>();
    match parts[..] {
        [bag, contains] => {
            if contains == "no other bags" {
                Ok((bag.into(), Vec::new()))
            } else {
                let parts = contains
                    .split(", ")
                    .map(|part| {
                        let parts = part.splitn(2, ' ').collect::<Vec<_>>();
                        let count = parts[0].parse().unwrap();
                        let name = if count == 1 {
                            parts[1].strip_suffix(" bag")
                        } else {
                            parts[1].strip_suffix(" bags")
                        }
                        .unwrap();
                        (count, name.into())
                    })
                    .collect::<Vec<_>>();
                Ok((bag.into(), parts))
            }
        }
        _ => Err("invalid rule".into()),
    }
}

fn main() -> Result<(), BoxError> {
    let mut graph = Graph::new();
    for line in io::stdin().lock().lines() {
        let line = line?;
        let rule = parse_rule(&line)?;
        graph.insert_vertex(&rule.0);
        for contains in rule.1 {
            graph.insert_vertex(&contains.1);
            graph.insert_edge(
                &rule.0,
                &contains.1,
                BagData {
                    contain_count: contains.0,
                },
            )?;
        }
    }

    let mut ancestors = HashSet::new();
    let mut queue = Vec::new();
    queue.extend(graph.find_ancestors("shiny gold"));
    while let Some(bag) = queue.pop() {
        if ancestors.insert(bag) {
            queue.extend(graph.find_ancestors(bag));
        }
    }

    println!("{:?}", ancestors.len());
    Ok(())
}
