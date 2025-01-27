//{"name":"D. Balanced Tree","group":"Codeforces - Ethflow Round 1 (Codeforces Round 1001, Div. 1 + Div. 2)","url":"https://codeforces.com/contest/2062/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n4\n0 11\n6 6\n0 0\n5 5\n2 1\n3 1\n4 3\n7\n1 1\n0 5\n0 5\n2 2\n2 2\n2 2\n2 2\n1 2\n1 3\n2 4\n2 5\n3 6\n3 7\n4\n1 1\n1 1\n1 1\n0 0\n1 4\n2 4\n3 4\n7\n0 20\n0 20\n0 20\n0 20\n3 3\n4 4\n5 5\n1 2\n1 3\n1 4\n2 5\n3 6\n4 7\n5\n1000000000 1000000000\n0 0\n1000000000 1000000000\n0 0\n1000000000 1000000000\n3 2\n2 1\n1 4\n4 5\n6\n21 88\n57 81\n98 99\n61 76\n15 50\n23 67\n2 1\n3 2\n4 3\n5 3\n6 4\n","output":"11\n3\n3\n5\n3000000000\n98\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBalancedTree"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let lr = input.read_int_pair_vec(n);
    let mut adj = vec![vec![]; n];
    for (u, v) in input.read_int_pair_vec(n - 1) {
        let u = u as usize - 1;
        let v = v as usize - 1;
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut root = 0;
    while adj[root].len() > 1 {
        root += 1;
    }
    let (root_value, push_up) = doit(&adj, root, n, &lr);
    out.print_line(root_value + push_up);
}

fn doit(adj: &Vec<Vec<usize>>, u: usize, p: usize, lr: &[(i32, i32)]) -> (i64, i64) {
    let mut push_up = 0;
    let mut pick_value = 0;
    let mut child_result = vec![];
    for v in &adj[u] {
        let v = *v;
        if v == p {
            continue;
        }
        let (x, y) = doit(adj, v, u, lr);
        child_result.push(x);
        pick_value = pick_value.max(x);
        push_up += y;
    }
    pick_value = pick_value.clamp(lr[u].0 as i64, lr[u].1 as i64);

    for x in child_result {
        if x > pick_value {
            push_up += x - pick_value;
        }
    }
    (pick_value, push_up)
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
