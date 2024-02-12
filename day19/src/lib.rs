use std::{
    cmp::{max, min},
    collections::HashMap,
    error::Error,
    fs,
    path::PathBuf,
    str::FromStr,
};

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum PartAttribute {
    X,
    M,
    A,
    S,
}

impl FromStr for PartAttribute {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::X),
            "m" => Ok(Self::M),
            "a" => Ok(Self::A),
            "s" => Ok(Self::S),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
enum RuleType {
    GreaterThan(i64),
    LessThan(i64),
}

#[derive(Debug)]
struct Rule {
    destination: String,
    attribute: Option<PartAttribute>,
    rule_type: Option<RuleType>,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(':') {
            None => Ok(Self {
                destination: s.to_string(),
                attribute: None,
                rule_type: None,
            }),
            Some((op, dst)) => {
                let (atr, op_val) = op.split_at(1);
                let (op, val_str) = op_val.split_at(1);
                let val = val_str.parse::<i64>().unwrap();
                let rule_type = match op {
                    ">" => RuleType::GreaterThan(val),
                    "<" => RuleType::LessThan(val),
                    _ => panic!(),
                };
                Ok(Self {
                    destination: dst.to_string(),
                    attribute: Some(PartAttribute::from_str(atr).unwrap()),
                    rule_type: Some(rule_type),
                })
            }
        }
    }
}

fn process_part(
    part: &HashMap<PartAttribute, i64>,
    workflow_id: &str,
    workflows: &HashMap<&str, Vec<Rule>>,
) -> i64 {
    let mut destination = "R";

    for rule in workflows.get(workflow_id).unwrap() {
        if let Some(rule_type) = &rule.rule_type {
            let attribute = &rule.attribute;
            let part_value = part.get(attribute.as_ref().unwrap()).unwrap();
            let passes = match rule_type {
                RuleType::GreaterThan(v) => part_value > v,
                RuleType::LessThan(v) => part_value < v,
            };
            if passes {
                destination = &rule.destination;
                break;
            }
        } else {
            destination = &rule.destination;
            break;
        }
    }

    match destination {
        "A" => part.values().sum(),
        "R" => 0,
        _ => process_part(part, destination, workflows),
    }
}

fn process_range(
    part_range: &HashMap<PartAttribute, (i64, i64)>,
    workflow_id: &str,
    workflows: &HashMap<&str, Vec<Rule>>,
) -> i64 {
    // Recursively push ranges through the workflows rather than individual parts, splitting them
    // if necessary, similar to Day 5.
    match workflow_id {
        // Base cases
        "A" => return part_range.values().map(|(l, u)| *u - *l + 1).product(),
        "R" => return 0,
        _ => (),
    }

    let mut counter = 0;

    let mut current_range = (*part_range).clone();

    for rule in workflows.get(workflow_id).unwrap() {
        if let Some(rule_type) = &rule.rule_type {
            let attribute = &rule.attribute.unwrap();
            let (l, u) = current_range.get(attribute).unwrap();
            match rule_type {
                RuleType::GreaterThan(v) => {
                    if u > v {
                        // Split the range.
                        let mut new_range = current_range.clone();
                        new_range.insert(*attribute, (max(*v + 1, *l), *u));
                        counter += process_range(&new_range, &rule.destination, workflows);
                    }

                    current_range.insert(*attribute, (*l, min(*v, *u)));
                }
                RuleType::LessThan(v) => {
                    if l < v {
                        // Split the range.
                        let mut new_range = current_range.clone();
                        new_range.insert(*attribute, (*l, min(*v - 1, *u)));
                        counter += process_range(&new_range, &rule.destination, workflows);
                    }
                    current_range.insert(*attribute, (max(*v, *l), *u));
                }
            }
        } else {
            counter += process_range(&current_range, &rule.destination, workflows)
        }
    }

    counter
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> i64 {
    let (workflow_definitions, part_definitions) = puzzle_input.split_once("\n\n").unwrap();

    let workflows = parse_workflows(workflow_definitions);

    if part_two {
        // Treat ranges as closed.
        let part_range: HashMap<_, (i64, i64)> = HashMap::from([
            (PartAttribute::X, (1, 4000)),
            (PartAttribute::M, (1, 4000)),
            (PartAttribute::A, (1, 4000)),
            (PartAttribute::S, (1, 4000)),
        ]);
        process_range(&part_range, "in", &workflows)
    } else {
        let parts = parse_parts(part_definitions);
        parts.map(|p| process_part(&p, "in", &workflows)).sum()
    }
}

fn parse_parts(part_definitions: &str) -> impl Iterator<Item = HashMap<PartAttribute, i64>> + '_ {
    part_definitions.lines().map(|l| {
        l.trim_matches(&['{', '}'][..])
            .split(',')
            .map(|r| {
                let (atr, val) = r.split_once('=').unwrap();
                (
                    PartAttribute::from_str(atr).unwrap(),
                    val.parse::<i64>().unwrap(),
                )
            })
            .collect::<HashMap<_, _>>()
    })
}

fn parse_workflows(workflow_definitions: &str) -> HashMap<&str, Vec<Rule>> {
    workflow_definitions
        .lines()
        .map(|l| {
            let (id, rules_str) = l.split_once('{').unwrap();
            let rules = rules_str
                .trim_end_matches('}')
                .split(',')
                .map(|s| Rule::from_str(s).unwrap())
                .collect::<Vec<_>>();
            (id, rules)
        })
        .collect::<HashMap<_, _>>()
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
