//{"name":"E. Graph Composition","group":"Codeforces - Codeforces Round 998 (Div. 3)","url":"https://codeforces.com/contest/2060/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 2 1\n1 2\n2 3\n1 3\n2 1 1\n1 2\n1 2\n3 2 0\n3 2\n1 2\n1 0 0\n3 3 1\n1 2\n1 3\n2 3\n1 2\n","output":"3\n0\n2\n0\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EGraphComposition"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn read_graph(input: &mut Input, n: usize, m: usize) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for (u, v) in input.read_int_pair_vec(m) {
        let (u, v) = (u as usize - 1, v as usize - 1);
        adj[u].push(v);
        adj[v].push(u);
    }
    adj
}

fn find_components(adj: &Vec<Vec<usize>>) -> (usize, Vec<usize>) {
    let n = adj.len();
    let mut num_components = 0;
    let mut component = vec![n; n];
    let mut s = vec![];
    for i in 0..n {
        if component[i] == n {
            component[i] = num_components;
            s.push(i);
            while !s.is_empty() {
                let u = s.pop().unwrap();
                for v in &adj[u] {
                    let v = *v;
                    if component[v] == n {
                        component[v] = num_components;
                        s.push(v);
                    }
                }
            }
            num_components += 1;
        }
    }
    (num_components, component)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m1 = input.read();
    let m2 = input.read();
    let mut adj1 = read_graph(input, n, m1);
    let adj2 = read_graph(input, n, m2);

    let (num_components2, components2) = find_components(&adj2);
    let mut num_removed = 0;
    for u in 0..n {
        let old_len = adj1[u].len();
        adj1[u] = adj1[u]
            .drain(..)
            .filter(|v| components2[u] == components2[*v])
            .collect();
        num_removed += old_len - adj1[u].len();
    }
    num_removed /= 2;
    let (num_components1, _) = find_components(&adj1);
    let num_added = num_components1 - num_components2;
    out.print_line(num_removed + num_added);
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
