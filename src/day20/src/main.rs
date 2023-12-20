use std::collections::{HashMap, VecDeque};
use std::io;

#[derive(Clone, PartialEq, Debug)]
enum SignalType {
    LOW,
    HIGH,
}

#[derive(Clone, Debug)]
struct Pulse {
    target_node: String,
    origin_node: String,
    signal: SignalType,
}

trait ReceiveSignal {
    fn process_signal(
        &mut self,
        pulse: Pulse,
        successors: &HashMap<String, Vec<String>>,
    ) -> Vec<Pulse>;
}

trait InstalledModule {
    fn add_predecessor(&mut self, s: String);
}

#[derive(Debug)]
struct BroadCasterModule {}

#[derive(Debug)]
struct FlipFlopModule {
    state: bool,
}

#[derive(Debug)]
struct ConjunctionModule {
    last_pred_state: HashMap<String, SignalType>,
}

#[derive(Debug)]
enum Module {
    BroadCasterModule(BroadCasterModule),
    FlipFlopModule(FlipFlopModule),
    ConjunctionModule(ConjunctionModule),
}

impl InstalledModule for Module {
    fn add_predecessor(&mut self, s: String) {
        match self {
            Self::ConjunctionModule(c) => {
                c.last_pred_state.insert(s, SignalType::LOW);
            }
            _ => (),
        };
    }
}

impl ReceiveSignal for Module {
    fn process_signal(
        &mut self,
        pulse: Pulse,
        successors: &HashMap<String, Vec<String>>,
    ) -> Vec<Pulse> {
        let binding = Vec::new();
        let succs = successors.get(&pulse.target_node).unwrap_or(&binding);
        match self {
            Module::BroadCasterModule(_b) => {
                return succs
                    .iter()
                    .map(|s| Pulse {
                        target_node: s.clone(),
                        origin_node: pulse.target_node.clone(),
                        signal: pulse.signal.clone(),
                    })
                    .collect();
            }
            Module::FlipFlopModule(f) => {
                if pulse.signal == SignalType::HIGH {
                    return vec![];
                }
                f.state = !f.state;

                return succs
                    .iter()
                    .map(|s| Pulse {
                        target_node: s.clone(),
                        origin_node: pulse.target_node.clone(),
                        signal: if f.state {
                            SignalType::HIGH
                        } else {
                            SignalType::LOW
                        },
                    })
                    .collect();
            }
            Module::ConjunctionModule(c) => {
                c.last_pred_state
                    .insert(pulse.origin_node, pulse.signal.clone());

                return succs
                    .iter()
                    .map(|s| Pulse {
                        target_node: s.clone(),
                        origin_node: pulse.target_node.clone(),
                        signal: if c.last_pred_state.values().all(|f| *f == SignalType::HIGH) {
                            SignalType::LOW
                        } else {
                            SignalType::HIGH
                        },
                    })
                    .collect();
            }
        }
    }
}

fn main() {
    let lines: Vec<String> = io::stdin().lines().map(Result::unwrap).collect();

    let mut modules: HashMap<String, Module> = HashMap::new();

    for line in lines.iter() {
        let side_separation: Vec<String> = line.split("-").map(|x| x.to_string()).collect();
        let mut left_node = side_separation.get(0).unwrap().trim().to_string();
        if left_node == String::from("broadcaster") {
            modules.insert(
                left_node.clone(),
                Module::BroadCasterModule(BroadCasterModule {}),
            );
        } else {
            let mut characters = left_node.chars();
            let first_char = characters.next().unwrap();
            left_node = characters.as_str().to_string();
            if first_char == '%' {
                modules.insert(
                    left_node.clone(),
                    Module::FlipFlopModule(FlipFlopModule { state: false }),
                );
            } else if first_char == '&' {
                modules.insert(
                    left_node.clone(),
                    Module::ConjunctionModule(ConjunctionModule {
                        last_pred_state: HashMap::new(),
                    }),
                );
            } else {
                panic!()
            }
        }
    }

    let mut connections: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines.iter() {
        let side_separation: Vec<String> = line.split("-").map(|x| x.to_string()).collect();
        let mut left_node = side_separation.get(0).unwrap().trim().to_string();
        if left_node != String::from("broadcaster") {
            let mut characters = left_node.chars();
            characters.next().unwrap();
            left_node = characters.as_str().to_string();
        }

        let mut right_nodes = side_separation.get(1).unwrap().trim().to_string();
        let mut connection = vec![];
        for node in right_nodes.trim_start_matches(|x| x == '>').split(",") {
            let node = node.trim().to_string();
            let mut opt_module: Option<&mut Module> = modules.get_mut(&node);
            match opt_module {
                None => {
                    modules.insert(
                        node.clone(),
                        Module::BroadCasterModule(BroadCasterModule {}),
                    );
                }
                Some(module) => {
                    module.add_predecessor(left_node.clone());
                }
            }
            connection.push(node);
        }
        connections.insert(left_node, connection);
    }

    let mut deque = VecDeque::new();
    let mut count_low = 0i64;
    let mut count_high = 0i64;

    for i in 0..1000 {
        deque.push_back(Pulse {
            target_node: String::from("broadcaster"),
            origin_node: String::from("button"),
            signal: SignalType::LOW,
        });

        while !deque.is_empty() {
            let nxt = deque.pop_front().unwrap();

            if nxt.signal == SignalType::LOW {
                count_low += 1
            } else {
                count_high += 1
            }

            let mut module = modules.get_mut(&nxt.target_node).unwrap();
            let result = module.process_signal(nxt, &connections);
            deque.extend(result);
        }
    }

    println!("{}, {}", count_low, count_high);
    println!("{}", count_low * count_high);
}
