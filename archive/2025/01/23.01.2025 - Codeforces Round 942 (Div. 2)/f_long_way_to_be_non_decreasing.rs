//{"name":"F. Long Way to be Non-decreasing","group":"Codeforces - Codeforces Round 942 (Div. 2)","url":"https://codeforces.com/contest/1972/problem/F","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n5 8\n1 6 3 7 1\n2 3 5 8 7 1 5 6\n3 3\n1 3 2\n2 1 3\n10 10\n2 8 5 4 8 4 1 5 10 10\n6 7 2 6 3 4 1 1 3 5\n","output":"3\n-1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FLongWayToBeNonDecreasing"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

#[derive(Clone, Debug)]
struct TreeNodeData {
    depth: i32,
    root: usize,
    enter: i32,
    exit: i32,
}

#[derive(Clone, Debug)]
struct CycleNodeData {
    cycle_id: i32,
    cycle_len: i32,
    index: i32,
}

#[derive(Clone, Debug)]
enum Node {
    Undecided,
    CycleNode(CycleNodeData),
    TreeNode(TreeNodeData),
}

#[derive(Debug)]
struct Pseudoforest {
    nodes: Vec<Node>,
}

impl Pseudoforest {
    fn new(next: Vec<i32>) -> Self {
        let n = next.len();
        let mut nodes = vec![Node::Undecided; n];
        discover_cycles(&next, &mut nodes);
        process_trees(&next, &mut nodes);
        Self { nodes }
    }

    fn dist(&self, s: usize, t: usize) -> Option<i32> {
        match (&self.nodes[s], &self.nodes[t]) {
            (Node::TreeNode(s), Node::TreeNode(t)) => {
                if s.enter >= t.enter && s.exit <= t.exit {
                    Some(s.depth - t.depth)
                } else {
                    None
                }
            }
            (Node::CycleNode(_), Node::TreeNode(_)) => None,
            (Node::TreeNode(s), Node::CycleNode(_)) => self.dist(s.root, t).map(|d| d + s.depth),
            (Node::CycleNode(s), Node::CycleNode(t)) => {
                if s.cycle_id != t.cycle_id {
                    None
                } else {
                    if t.index >= s.index {
                        Some(t.index - s.index)
                    } else {
                        Some(t.cycle_len + t.index - s.index)
                    }
                }
            }
            _ => panic!("Unexpected"),
        }
    }
}

fn discover_cycles(next: &Vec<i32>, nodes: &mut Vec<Node>) {
    let n = next.len();
    let mut t = 0;
    let mut discover = vec![-1; n];
    let mut num_cycles = 0;
    for i in 0..n {
        if discover[i] != -1 {
            continue;
        }
        let mut u = i;
        while discover[u] == -1 {
            discover[u] = t;
            t += 1;
            u = next[u] as usize;
        }
        if discover[u] >= discover[i] {
            let cycle_len = t - discover[u];
            for i in 0..cycle_len {
                nodes[u] = Node::CycleNode(CycleNodeData {
                    cycle_id: num_cycles,
                    cycle_len,
                    index: i,
                });
                u = next[u] as usize;
            }
            num_cycles += 1;
        }
    }
}

fn process_trees(next: &Vec<i32>, nodes: &mut Vec<Node>) {
    let n = next.len();
    let mut adj = vec![vec![]; n];
    for i in 0..n {
        if let Node::Undecided = nodes[i] {
            adj[next[i] as usize].push(i);
        }
    }
    let mut t = 0;
    for i in 0..n {
        if let Node::CycleNode(_) = nodes[i] {
            traverse(&adj, i, i, 0, nodes, &mut t);
        }
    }
}

fn traverse(
    adj: &Vec<Vec<usize>>,
    u: usize,
    root: usize,
    depth: i32,
    nodes: &mut Vec<Node>,
    t: &mut i32,
) {
    let t_enter = *t;
    *t += 1;

    for v in &adj[u] {
        let v = *v;
        traverse(adj, v, root, depth + 1, nodes, t);
    }

    let t_exit = *t;
    *t += 1;

    if u != root {
        nodes[u] = Node::TreeNode(TreeNodeData {
            root,
            depth,
            enter: t_enter,
            exit: t_exit,
        });
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let a = input
        .read_int_vec(n)
        .into_iter()
        .map(|x| x - 1)
        .collect::<Vec<_>>();
    let b = input
        .read_int_vec(m)
        .into_iter()
        .map(|x| x - 1)
        .collect::<Vec<_>>();

    let g = Pseudoforest::new(b.clone());

    let mut lo = 0;
    let mut hi = m;
    while lo < hi {
        let mid = (lo + hi) / 2;

        let mut ok = true;
        let mut target = 0;
        for i in 0..n {
            while target < m && g.dist(a[i] as usize, target).unwrap_or(m as i32) > mid as i32 {
                target += 1;
            }
            if target == m {
                ok = false;
                break;
            }
        }
        if ok {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    if lo == m {
        out.print_line(-1);
    } else {
        out.print_line(lo);
    }
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
