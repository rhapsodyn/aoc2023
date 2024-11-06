use std::{
    collections::{HashMap, VecDeque},
    str::Split,
};

#[derive(Debug)]
enum State {
    On,
    Off,
}

#[derive(Debug, PartialEq, Clone)]
enum Pulse {
    High,
    Low,
}

// Flip-flop modules (prefix %) are either on or off;
// they are initially off. If a flip-flop module receives a high pulse,
// it is ignored and nothing happens.
// However, if a flip-flop module receives a low pulse, it flips between on and off.
// If it was off, it turns on and sends a high pulse.
// If it was on, it turns off and sends a low pulse.
#[derive(Debug)]
struct FlipFlop {
    name: String,
    state: State,
    output_to: Vec<String>,
}

// Conjunction modules (prefix &)
// remember the type of the most recent pulse received
// from each of their connected input modules;
// they initially default to remembering a low pulse for each input.
// When a pulse is received, the conjunction module first updates its memory for that input.
// Then, if it remembers high pulses for all inputs, it sends a low pulse;
// otherwise, it sends a high pulse.
#[derive(Debug)]
struct Conjunction {
    name: String,
    inputs: HashMap<String, Pulse>,
    output_to: Vec<String>,
}

// There is a single broadcast module (named broadcaster).
// When it receives a pulse, it sends the same pulse to all of its destination modules.
// When you push the button, a single low pulse is sent directly to the broadcaster module.
#[derive(Debug)]
struct Broadcaster {
    output_to: Vec<String>,
}

#[derive(Debug)]
enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster(Broadcaster),
}

const INIT_NAME: &'static str = "broadcaster";

// (from, to, pulse)
type Signal = (String, String, Pulse);

impl Module {
    fn get_output(&self) -> Vec<String> {
        match self {
            Module::FlipFlop(f) => f.output_to.clone(),
            Module::Conjunction(c) => c.output_to.clone(),
            Module::Broadcaster(b) => b.output_to.clone(),
        }
    }

    fn add_input(&mut self, input: &str) {
        match self {
            Module::Conjunction(c) => {
                c.inputs.insert(input.to_owned(), Pulse::Low);
            }
            _ => unreachable!("{:#?} have no inputs", self),
        }
    }
}

fn parse(input: &str) -> HashMap<String, Module> {
    let mut module_map = HashMap::new();
    let lines = input.lines();
    let mut conj_names = vec![];
    for l in lines {
        if l.len() > 0 {
            let mut seps = l.split(" ");
            let name = seps.next().unwrap();

            let m = if name.starts_with("&") {
                let conj_name: String = name.chars().skip(1).collect();
                conj_names.push(conj_name.clone());
                Module::Conjunction(Conjunction {
                    name: conj_name,
                    inputs: HashMap::new(),
                    output_to: make_output(&mut seps),
                })
            } else if name.starts_with("%") {
                Module::FlipFlop(FlipFlop {
                    name: name.chars().skip(1).collect(),
                    state: State::Off,
                    output_to: make_output(&mut seps),
                })
            } else {
                assert_eq!(name, INIT_NAME);
                Module::Broadcaster(Broadcaster {
                    output_to: make_output(&mut seps),
                })
            };

            let key = if name.starts_with("&") || name.starts_with("%") {
                name.chars().skip(1).collect()
            } else {
                name.to_owned()
            };
            module_map.insert(key, m);
        }
    }

    let mut output_map = HashMap::new();
    for (name, module) in module_map.iter() {
        output_map.insert(name.to_owned(), module.get_output());
    }

    for (iname, output) in output_map.iter() {
        for oname in output {
            if conj_names.contains(oname) {
                module_map.get_mut(oname).unwrap().add_input(iname)
            }
        }
    }

    dbg!(&module_map);
    module_map
}

fn make_output(seps: &mut Split<&str>) -> Vec<String> {
    assert_eq!(seps.next().unwrap(), "->");
    let mut outputs = vec![];
    for out in seps {
        let out_name = if out.ends_with(",") {
            out.chars().take(out.len() - 1).collect()
        } else {
            out.to_owned()
        };

        outputs.push(out_name)
    }
    outputs
}

pub fn ans1(input: &str) -> u64 {
    let mut mods = parse(input);
    // init
    let mut signals: VecDeque<Signal> = VecDeque::new();
    let mut low_counter = 0;
    let mut high_counter = 0;

    for _ in 0..1000 {
        // button pressed
        signals.push_back((String::from("button"), String::from(INIT_NAME), Pulse::Low));
        while signals.len() > 0 {
            // one round
            let (from, to, pulse) = signals.pop_front().unwrap();
            //println!("{:?} -{:?} -> {:?}", &from, &pulse, &to);
            match pulse {
                Pulse::High => high_counter += 1,
                Pulse::Low => low_counter += 1,
            }

            match mods.get_mut(&to) {
                Some(target) => {
                    match target {
                        Module::FlipFlop(f) => {
                            if pulse == Pulse::High {
                                // ignored
                                continue;
                            } else {
                                match f.state {
                                    State::On => {
                                        f.state = State::Off;
                                        // send low out
                                        for next in &f.output_to {
                                            signals.push_back((
                                                f.name.clone(),
                                                next.to_owned(),
                                                Pulse::Low,
                                            ))
                                        }
                                    }
                                    State::Off => {
                                        f.state = State::On;
                                        // send high out
                                        for next in &f.output_to {
                                            signals.push_back((
                                                f.name.clone(),
                                                next.to_owned(),
                                                Pulse::High,
                                            ))
                                        }
                                    }
                                }
                            }
                        }
                        Module::Conjunction(c) => {
                            // update first
                            for i in c.inputs.iter_mut() {
                                if i.0 == &from {
                                    *i.1 = pulse.clone();
                                }
                            }

                            let next = if c.inputs.iter().all(|(_, p)| p == &Pulse::High) {
                                // all high
                                // send low
                                Pulse::Low
                            } else {
                                // send high
                                Pulse::High
                            };

                            for o in &c.output_to {
                                signals.push_back((c.name.clone(), o.to_owned(), next.clone()))
                            }
                        }
                        Module::Broadcaster(b) => {
                            for o in &b.output_to {
                                signals.push_back((
                                    String::from(INIT_NAME),
                                    o.to_owned(),
                                    Pulse::Low,
                                ));
                            }
                        }
                    }
                }
                None => {
                    // (for testing purposes)
                    continue;
                }
            }
        }
    }

    dbg!(high_counter);
    dbg!(low_counter);
    high_counter * low_counter
}

#[test]
fn test1() {
    assert_eq!(
        ans1(
            r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#
        ),
        32000000
    );

    assert_eq!(
        ans1(
            r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#
        ),
        11687500
    )
}

#[test]
fn run1() {
    let q = include_str!("../inputs/input20.txt");
    println!("{}", ans1(&q));
}

pub fn ans2(input: &str) -> u64 {
    return todo!("tooooo slow, maybe some gcd work");

    let mut mods = parse(input);
    let mut signals: VecDeque<Signal> = VecDeque::new();
    let mut counter = 0;
    loop {
        // button pressed
        counter += 1;
        signals.push_back((String::from("button"), String::from(INIT_NAME), Pulse::Low));
        while signals.len() > 0 {
            // one round
            let (from, to, pulse) = signals.pop_front().unwrap();
            //println!("{:?} -{:?} -> {:?}", &from, &pulse, &to);
            if to == "rx" && pulse == Pulse::Low {
                return counter;
            }

            match mods.get_mut(&to) {
                Some(target) => {
                    match target {
                        Module::FlipFlop(f) => {
                            if pulse == Pulse::High {
                                // ignored
                                continue;
                            } else {
                                match f.state {
                                    State::On => {
                                        f.state = State::Off;
                                        // send low out
                                        for next in &f.output_to {
                                            signals.push_back((
                                                f.name.clone(),
                                                next.to_owned(),
                                                Pulse::Low,
                                            ))
                                        }
                                    }
                                    State::Off => {
                                        f.state = State::On;
                                        // send high out
                                        for next in &f.output_to {
                                            signals.push_back((
                                                f.name.clone(),
                                                next.to_owned(),
                                                Pulse::High,
                                            ))
                                        }
                                    }
                                }
                            }
                        }
                        Module::Conjunction(c) => {
                            // update first
                            for i in c.inputs.iter_mut() {
                                if i.0 == &from {
                                    *i.1 = pulse.clone();
                                }
                            }

                            let next = if c.inputs.iter().all(|(_, p)| p == &Pulse::High) {
                                // all high
                                // send low
                                Pulse::Low
                            } else {
                                // send high
                                Pulse::High
                            };

                            for o in &c.output_to {
                                signals.push_back((c.name.clone(), o.to_owned(), next.clone()))
                            }
                        }
                        Module::Broadcaster(b) => {
                            for o in &b.output_to {
                                signals.push_back((
                                    String::from(INIT_NAME),
                                    o.to_owned(),
                                    Pulse::Low,
                                ));
                            }
                        }
                    }
                }
                None => {
                    // (for testing purposes)
                    continue;
                }
            }
        }
    }
}

#[test]
fn test2() {}

#[test]
fn run2() {
     let q = include_str!("../inputs/input20.txt");
     println!("{}", ans2(&q));
}
