//{"name":"D. Skipping","group":"Codeforces - Codeforces Round 980 (Div. 2)","url":"https://codeforces.com/contest/2024/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n15 16\n2 1\n5\n10 10 100 100 1000\n3 4 1 1 1\n3\n100 49 50\n3 2 2\n4\n100 200 300 1000\n2 3 4 1\n","output":"16\n200\n100\n1000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSkipping"}}}

use std::collections::BinaryHeap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i64 = 1000000000000000000;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_long_vec(n);
    let b = input.read_long_vec(n);
    let b: Vec<usize> = b.into_iter().map(|b| b as usize - 1).collect();
    let mut prefix_sum = vec![0; n + 1];
    for i in 0..n {
        prefix_sum[i + 1] = a[i] + prefix_sum[i];
    }

    let mut dist = vec![INF; n];
    let mut pq = BinaryHeap::new();
    let mut result = 0;
    dist[0] = 0;
    pq.push((0, 0));
    while !pq.is_empty() {
        let (d, u) = pq.pop().unwrap();
        let d = -d;
        if d > dist[u] {
            continue;
        }
        result = result.max(prefix_sum[u + 1] - d);

        let mut relax = |d: i64, v: usize| {
            if d < dist[v] {
                dist[v] = d;
                pq.push((-d, v))
            }
        };
        if b[u] > u {
            relax(d + a[u], b[u]);
        }
        if u > 0 {
            relax(d, u - 1);
        }
    }
    out.print_line(result);
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
