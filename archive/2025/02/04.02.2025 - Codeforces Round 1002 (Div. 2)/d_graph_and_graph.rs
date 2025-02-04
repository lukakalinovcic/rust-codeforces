//{"name":"D. Graph and Graph","group":"Codeforces - Codeforces Round 1002 (Div. 2)","url":"https://codeforces.com/contest/2059/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4 1 1\n4\n1 2\n2 3\n3 4\n4 1\n4\n1 2\n2 3\n3 4\n4 1\n4 1 2\n4\n1 2\n2 3\n3 4\n4 1\n4\n1 2\n2 3\n3 4\n4 1\n7 7 2\n7\n1 6\n2 1\n3 2\n3 4\n5 1\n7 3\n7 5\n6\n5 1\n5 6\n5 7\n6 3\n7 2\n7 4\n","output":"0\n-1\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DGraphAndGraph"}}}

use std::collections::BinaryHeap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let s1 = input.read_size() - 1;
    let s2 = input.read_size() - 1;
    let adj1 = read_graph(input, n);
    let adj2 = read_graph(input, n);
    let mut in_cycle = vec![false; n];
    for v in 0..n {
        for &u1 in &adj1[v] {
            for &u2 in &adj2[v] {
                if u1 == u2 {
                    in_cycle[v] = true;
                    in_cycle[u1] = true;
                }
            }
        }
    }

    const INF: i32 = 1000000000;
    let mut dist = vec![vec![INF; n]; n];
    dist[s1][s2] = 0;
    let mut pq = BinaryHeap::new();
    pq.push((0, (s1, s2)));
    while !pq.is_empty() {
        let (d, (v1, v2)) = pq.pop().unwrap();
        let d = -d;
        if d != dist[v1][v2] {
            continue;
        }
        if v1 == v2 && in_cycle[v1] {
            out.print_line(d);
            return;
        }

        for &u1 in &adj1[v1] {
            for &u2 in &adj2[v2] {
                let cost = (u1 as i32 - u2 as i32).abs();
                if d + cost < dist[u1][u2] {
                    dist[u1][u2] = d + cost;
                    pq.push((-dist[u1][u2], (u1, u2)));
                }
            }
        }
    }
    out.print_line(-1);
}

fn read_graph(input: &mut Input, n: usize) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    let m = input.read();
    for (u, v) in input.read_int_pair_vec(m) {
        let u = u as usize - 1;
        let v = v as usize - 1;
        adj[u].push(v);
        adj[v].push(u);
    }
    adj
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
