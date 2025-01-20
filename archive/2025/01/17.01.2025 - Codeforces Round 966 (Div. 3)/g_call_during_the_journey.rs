//{"name":"G. Call During the Journey","group":"Codeforces - Codeforces Round 966 (Div. 3)","url":"https://codeforces.com/contest/2000/problem/G","interactive":false,"timeLimit":4000,"tests":[{"input":"7\n5 5\n100 20 80\n1 5 30 100\n1 2 20 50\n2 3 20 50\n3 4 20 50\n4 5 20 50\n2 1\n100 50 60\n1 2 55 110\n4 4\n100 40 60\n1 2 30 100\n2 4 30 100\n1 3 20 50\n3 4 20 50\n3 3\n100 80 90\n1 2 1 10\n2 3 10 50\n1 3 20 21\n3 2\n58 55 57\n2 1 1 3\n2 3 3 4\n2 1\n12 9 10\n2 1 6 10\n5 5\n8 5 6\n2 1 1 8\n2 3 4 8\n4 2 2 4\n5 3 3 4\n4 5 2 6\n","output":"0\n-1\n60\n80\n53\n3\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GCallDuringTheJourney"}}}

use std::collections::BinaryHeap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i64 = 1000000000000000000;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let (t_final, t_call_begin, t_call_end) =
        (input.read_long(), input.read_long(), input.read_long());
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let (u, v) = (input.read_int() as usize - 1, input.read_int() as usize - 1);
        let (t_bus, t_walk) = (input.read_long(), input.read_long());
        adj[u].push((v, t_bus, t_walk));
        adj[v].push((u, t_bus, t_walk));
    }

    let mut pq = BinaryHeap::new();
    let mut t_latest = vec![-INF; n];
    t_latest[n - 1] = t_final;
    pq.push((t_final, n - 1));
    while !pq.is_empty() {
        let (t_u, u) = pq.pop().unwrap();
        if t_u < t_latest[u] {
            continue;
        }
        for edge in &adj[u] {
            let (v, t_bus, t_walk) = *edge;
            // Try walking.
            let t_v_walking = t_u - t_walk;
            if t_v_walking > t_latest[v] {
                t_latest[v] = t_v_walking;
                pq.push((t_v_walking, v));
            }
            // Try the bus.
            let can_take_bus_now = t_u <= t_call_begin || t_u - t_bus >= t_call_end;
            let t_v_bus = if can_take_bus_now { t_u } else { t_call_begin } - t_bus;
            if t_v_bus > t_latest[v] {
                t_latest[v] = t_v_bus;
                pq.push((t_v_bus, v));
            }
        }
    }

    if t_latest[0] < 0 {
        out.print_line(-1);
    } else {
        out.print_line(t_latest[0]);
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
