//{"name":"E. Tree Pruning","group":"Codeforces - Codeforces Round 975 (Div. 2)","url":"https://codeforces.com/contest/2019/problem/E","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n7\n1 2\n1 3\n2 4\n2 5\n4 6\n4 7\n7\n1 2\n1 3\n1 4\n2 5\n3 6\n5 7\n15\n12 9\n1 6\n6 14\n9 11\n8 7\n3 5\n13 5\n6 10\n13 15\n13 6\n14 12\n7 2\n8 1\n1 4\n","output":"2\n2\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETreePruning"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut adj = vec![vec![]; n + 1];
    for (u, v) in input.read_int_pair_vec(n - 1) {
        adj[u as usize].push(v);
        adj[v as usize].push(u);
    }
    let maxp = n.next_power_of_two().trailing_zeros() as usize;
    let mut p = vec![vec![0; maxp + 1]; n + 1];
    p[1][0] = 1;
    let mut depth: usize = 0;
    let mut curr_depth = vec![1];
    let mut result = n;
    while !curr_depth.is_empty() {
        let mut next_depth = vec![];
        depth += 1;
        let maxk = (depth + 1).next_power_of_two().trailing_zeros() as usize;
        for u in curr_depth {
            for &v in &adj[u as usize] {
                let v = v as usize;
                if p[v][0] != 0 {
                    continue;
                }
                p[v][0] = u as i32;
                for k in 1..maxk {
                    p[v][k] = p[p[v][k - 1] as usize][k - 1];
                }
                next_depth.push(v);
            }
        }
        let mut keep = 1;
        for i in 0..next_depth.len() {
            if i > 0 {
                let mut u = next_depth[i - 1];
                let mut v = next_depth[i];
                for k in (0..maxk).rev() {
                    if p[u][k] != p[v][k] {
                        keep += 1 << k;
                        u = p[u][k] as usize;
                        v = p[v][k] as usize;
                    }
                }
                keep += 1;
            } else {
                keep += depth;
            }
        }
        result = result.min(n - keep);

        curr_depth = next_depth;
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
