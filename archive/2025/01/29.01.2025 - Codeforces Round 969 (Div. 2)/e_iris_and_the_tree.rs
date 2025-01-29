//{"name":"E. Iris and the Tree","group":"Codeforces - Codeforces Round 969 (Div. 2)","url":"https://codeforces.com/contest/2007/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n2 1000000000000\n1\n2 1000000000000\n4 9\n1 1 1\n2 2\n4 4\n3 3\n6 100\n1 2 3 2 1\n6 17\n3 32\n2 4\n4 26\n5 21\n10 511\n1 2 2 4 2 1 1 8 8\n3 2\n6 16\n10 256\n9 128\n2 1\n5 8\n8 64\n4 4\n7 32\n","output":"2000000000000\n25 18 18\n449 302 247 200 200\n4585 4473 2681 1567 1454 1322 1094 1022 1022\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EIrisAndTheTree"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let w = input.read_long();
    let mut p = vec![0; n];
    let mut d = vec![0; n];
    for i in 1..n {
        p[i] = input.read::<usize>() - 1;
        d[i] = d[p[i]] + 1;
    }
    let mut alive_edges = vec![0; n];
    let mut paths = vec![[n, n]; n];
    let mut assign_to_path = |u: usize, i: usize| {
        alive_edges[i] += 1;
        if paths[u][0] == n {
            paths[u][0] = i;
        } else {
            paths[u][1] = i;
        }
    };

    for i in 0..n {
        let mut u = i;
        let mut v = (i + 1) % n;
        while d[u] > d[v] {
            assign_to_path(u, i);
            u = p[u];
        }
        while d[v] > d[u] {
            assign_to_path(v, i);
            v = p[v];
        }
        while u != v {
            assign_to_path(u, i);
            assign_to_path(v, i);
            u = p[u];
            v = p[v];
        }
    }

    let mut results = vec![];
    let mut sum = 0;
    let mut alive_paths = n;
    for _ in 1..n {
        let x = input.read_size() - 1;
        let y = input.read_long();
        sum += y;
        for &i in &paths[x] {
            alive_edges[i] -= 1;
            if alive_edges[i] == 0 {
                alive_paths -= 1;
            }
        }
        results.push(alive_paths as i64 * (w - sum) + sum * 2);
    }
    out.print_line(results);
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
