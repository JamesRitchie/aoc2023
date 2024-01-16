use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    error::Error,
    fs,
    path::PathBuf,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Vertical,
    Horizontal,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    i: usize,
    j: usize,
    dir: Direction,
}

struct QueueEntry {
    node: Node,
    heat_loss: i64,
}

impl Ord for QueueEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse standard ordering so our queue behaves as a MinHeap
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for QueueEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for QueueEntry {}

impl PartialEq for QueueEntry {
    fn eq(&self, other: &Self) -> bool {
        self.heat_loss.eq(&other.heat_loss)
    }
}

fn get_neighbours(node: Node, grid: &Vec<Vec<i64>>, part_two: bool) -> Vec<(Node, i64)> {
    let n_rows = grid.len() as i64;
    let n_cols = grid[0].len() as i64;
    let r_lower; // Minimum number of blocks to traverse before turning.
    let r_upper; // Maximum

    if part_two {
        r_lower = 4;
        r_upper = 11;
    } else {
        r_lower = 1;
        r_upper = 4;
    }
    let mut offsets = vec![];

    let new_dir = match node.dir {
        Direction::Horizontal => Direction::Vertical,
        Direction::Vertical => Direction::Horizontal,
    };

    for z in [-1, 1] {
        let mut heat_loss = 0;
        // Need to start from 1 rather than r_lower in order to compute the total heat loss.
        for r in 1..r_upper {
            let mut i = node.i as i64;
            let mut j = node.j as i64;
            match new_dir {
                Direction::Horizontal => j += z * r,
                Direction::Vertical => i += z * r,
            }
            if j >= 0 && j < n_cols && i >= 0 && i < n_rows {
                heat_loss += grid[i as usize][j as usize];
                if r >= r_lower {
                    offsets.push((
                        Node {
                            i: i as usize,
                            j: j as usize,
                            dir: new_dir,
                        },
                        heat_loss,
                    ))
                }
            }
        }
    }
    offsets
}

fn compute_answer(puzzle_input: &str, part_two: bool) -> i64 {
    // Implements Dijkstra's algorithm, where every node is an entry in the grid augmented with the
    // orientation of the direction of travel where the node was arrived at.
    // Neighbours of each node are defined by the maximum and minimum travel distance defined in
    // for both problem parts and the arrival orientation.

    let grid = puzzle_input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let n_rows = grid.len();
    let n_cols = grid[0].len();

    let mut queue = BinaryHeap::new();
    let mut heat_losses = HashMap::new();
    let mut visited = HashSet::new();

    for d in [Direction::Horizontal, Direction::Vertical] {
        let n = Node { i: 0, j: 0, dir: d };
        heat_losses.insert(n, 0);
        queue.push(QueueEntry {
            node: n,
            heat_loss: 0,
        });
    }

    while let Some(entry) = queue.pop() {
        if entry.node.i == n_rows - 1 && entry.node.j == n_cols - 1 {
            break;
        }
        if !visited.insert(entry.node) {
            continue;
        }

        let neighbours = get_neighbours(entry.node, &grid, part_two);

        for (node, heat_loss) in neighbours.iter() {
            let new_heat_loss = entry.heat_loss + heat_loss;

            let lower = match heat_losses.get(node) {
                None => true,
                Some(old_heat_loss) => new_heat_loss < *old_heat_loss,
            };

            if lower && !visited.contains(node) {
                heat_losses.insert(*node, new_heat_loss);
                queue.push(QueueEntry {
                    node: *node,
                    heat_loss: new_heat_loss,
                });
            }
        }
    }

    [Direction::Horizontal, Direction::Vertical]
        .iter()
        .filter_map(|d| {
            heat_losses.get(&Node {
                i: n_rows - 1,
                j: n_cols - 1,
                dir: *d,
            })
        })
        .map(|v| *v)
        .min()
        .unwrap()
}

pub fn run(input_path: PathBuf, part_two: bool) -> Result<i64, Box<dyn Error>> {
    let puzzle_input = fs::read_to_string(input_path)?;
    let answer = compute_answer(&puzzle_input, part_two);
    println!("The answer is {answer}");
    Ok(answer)
}
