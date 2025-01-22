//{"name":"E. Triangle Tree","group":"Codeforces - Codeforces Round 1000 (Div. 2)","url":"https://codeforces.com/contest/2063/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n1 2\n1 3\n3\n1 2\n3 2\n5\n2 3\n1 5\n4 2\n1 2\n11\n2 1\n2 3\n2 4\n4 5\n6 5\n5 7\n4 8\n8 9\n7 10\n10 11\n","output":"1\n0\n4\n29\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETriangleTree"}}}

use std::mem::swap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut adj = vec![vec![]; n];
    for (u, v) in input.read_int_pair_vec(n - 1) {
        let u = u as usize - 1;
        let v = v as usize - 1;
        adj[u].push(v);
        adj[v].push(u);
    }
    out.print_line(doit(&adj, 0, n).0);
}

fn doit(adj: &Vec<Vec<usize>>, u: usize, p: usize) -> (i64, Vec<i32>) {
    let mut curr = (0 as i64, vec![0]);
    for v in &adj[u] {
        let v = *v;
        if v == p {
            continue;
        }
        curr = merge(curr, doit(adj, v, u));
    }
    curr.1.push(curr.1.last().cloned().unwrap() + 1);
    curr
}

fn merge(a: (i64, Vec<i32>), b: (i64, Vec<i32>)) -> (i64, Vec<i32>) {
    let mut result = a.0 + b.0;
    let mut aa = a.1;
    let mut bb = b.1;
    if aa.len() < bb.len() {
        swap(&mut aa, &mut bb);
    }
    let n_a = aa.len();
    let n_b = bb.len();
    let mut sum_lo = 0;
    for d in 1..n_b {
        let b_d = (bb[n_b - d] - bb[n_b - d - 1]) as i64;
        let a_d = (aa[n_a - d] - aa[n_a - d - 1]) as i64;
        let a_sum = aa[n_a - d] as i64;
        result += b_d * sum_lo;
        result += b_d * a_sum * (2 * d - 1) as i64;
        sum_lo += a_d * (2 * d - 1) as i64;
    }
    for d in 1..n_b {
        aa[n_a - n_b + d] += bb[d];
    }
    (result, aa)
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
