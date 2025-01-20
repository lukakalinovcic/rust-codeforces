//{"name":"E. Rendez-vous de Marian et Robin","group":"Codeforces - Codeforces Round 974 (Div. 3)","url":"https://codeforces.com/contest/2014/problem/E","interactive":false,"timeLimit":5000,"tests":[{"input":"6\n2 1 1\n1\n1 2 10\n3 1 2\n2 3\n1 2 10\n3 3 1\n2\n1 2 4\n1 3 10\n2 3 6\n4 3 2\n2 3\n1 2 10\n2 3 18\n3 4 16\n3 2 1\n2\n1 2 4\n1 3 16\n7 7 1\n3\n1 5 2\n2 6 12\n1 2 12\n6 4 8\n7 3 4\n6 3 4\n7 6 4\n","output":"5\n-1\n6\n19\n14\n12\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ERendezVousDeMarianEtRobin"}}}

use std::collections::BinaryHeap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i64 = 1000000000000000000;

fn shortest_paths(adj: &Vec<Vec<(usize, i64)>>, source: usize, horse: &Vec<bool>) -> Vec<i64> {
    let n = adj.len();
    let mut dist = vec![[INF, INF]; n];
    dist[source] = [0, INF];
    let mut pq = BinaryHeap::new();
    pq.push((0, source, 0));
    while !pq.is_empty() {
        let (d, u, mut on_horse) = pq.pop().unwrap();
        let d = -d;
        if d > dist[u][on_horse] {
            continue;
        }
        if horse[u] {
            on_horse = 1;
        }
        for (v, w) in &adj[u] {
            let v = *v;
            let w = if on_horse == 1 { *w / 2 } else { *w };
            if dist[v][on_horse] > d + w {
                dist[v][on_horse] = d + w;
                pq.push((-dist[v][on_horse], v, on_horse));
            }
        }
    }
    dist.into_iter().map(|[d1, d2]| d1.min(d2)).collect()
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let h = input.read();
    let mut horse = vec![false; n];
    for i in input.read_int_vec(h) {
        horse[i as usize - 1] = true;
    }
    let mut adj = vec![vec![]; n];
    for _ in 0..m {
        let (u, v, w) = (
            input.read::<usize>() - 1,
            input.read::<usize>() - 1,
            input.read_long(),
        );
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    let mut result = INF;
    for (d1, d2) in shortest_paths(&adj, 0, &horse)
        .into_iter()
        .zip(shortest_paths(&adj, n - 1, &horse).into_iter())
    {
        result = result.min(d1.max(d2));
    }
    if result == INF {
        out.print_line(-1);
    } else {
        out.print_line(result);
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
