use sscanf::sscanf;
use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let network = Network::from_stdin();
    println!("{}", network.get_human_steps());
    println!("{}", network.get_ghost_steps());
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

struct Network {
    instruction: String,
    nodes: HashMap<String, Node>,
}

impl Network {
    fn from_stdin() -> Network {
        let mut iter = std::io::stdin().lock().lines();
        let Ok(instruction) = iter.next().unwrap() else {
            todo!()
        };
        let _ = iter.next(); // Empty
        let mut nodes: HashMap<String, Node> = HashMap::new();
        for line in iter {
            let line = line.unwrap();
            let Ok((id, left, right)) = sscanf!(line, "{} = ({}, {})", String, String, String)
            else {
                todo!()
            };
            nodes.insert(id, Node { left, right });
        }
        eprintln!("{}", &instruction);
        eprintln!("{:?}", &nodes);
        Network { instruction, nodes }
    }

    fn nth_instruction(&self, n: usize) -> char {
        self.instruction
            .chars()
            .nth(n % self.instruction.len())
            .unwrap()
    }

    fn get_human_steps(&self) -> i64 {
        let mut n = 0;
        let mut id = "AAA".to_string();
        while id != "ZZZ" {
            let node = &self.nodes[&id];
            id = match self.nth_instruction(n) {
                'L' => node.left.clone(),
                'R' => node.right.clone(),
                _ => "".to_string(),
            };
            n += 1;
        }
        n as i64
    }

    fn get_ghost_steps(&self) -> i64 {
        let ids: Vec<String> = self
            .nodes
            .keys()
            .filter(|x| x.ends_with("A"))
            .map(|x| x.clone())
            .collect();
        let loops: Vec<_> = ids.iter().map(|id| self.ghost_goal_steps(&id)).collect();
        0
    }

    fn ghost_goal_steps(&self, id: &str) -> (i64, i64) {
        (0, 0)
    }
}
