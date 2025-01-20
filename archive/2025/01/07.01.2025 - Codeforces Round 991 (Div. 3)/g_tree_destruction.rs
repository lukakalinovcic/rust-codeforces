//{"name":"G. Tree Destruction","group":"Codeforces - Codeforces Round 991 (Div. 3)","url":"https://codeforces.com/contest/2050/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n2\n1 2\n5\n1 2\n2 3\n3 4\n3 5\n4\n1 2\n2 3\n3 4\n5\n2 1\n3 1\n4 1\n5 4\n6\n2 1\n3 1\n4 1\n5 3\n6 3\n6\n2 1\n3 2\n4 2\n5 3\n6 4\n","output":"1\n3\n2\n3\n4\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GTreeDestruction"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn doit(
    adj: &Vec<Vec<i32>>,
    root: i32,
    parent: i32,
    path_active: bool,
    memo: &mut Vec<i32>,
) -> i32 {
    let key = (root * 2 + if path_active { 1 } else { 0 }) as usize;
    if memo[key] != -1 {
        return memo[key];
    }
    let mut n_children = 0;
    let (mut active1, mut active2) = (0, 0);
    let mut inactive = 0;
    for neigh in adj[root as usize].iter() {
        let neigh = *neigh;
        if neigh == parent {
            continue;
        }
        n_children += 1;
        let x = doit(adj, neigh, root, true, memo);
        if x > active1 {
            active2 = active1;
            active1 = x;
        } else if x > active2 {
            active2 = x;
        }
        let x = doit(adj, neigh, root, false, memo);
        if x > inactive {
            inactive = x;
        }
    }
    let result = if path_active {
        let mut result = n_children;
        result = result.max(active1 + (n_children - 1).max(0));
        result
    } else {
        let mut result = inactive;
        let top = if parent == -1 { 0 } else { 1 };
        result = result.max(top + n_children);
        result = result.max(top + active1 + (n_children - 1).max(0));
        result = result.max(top + active1 + active2 + (n_children - 2).max(0));
        result
    };
    memo[key] = result;
    result
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let mut adj = vec![vec![]; n];
    for (a, b) in input.read_int_pair_vec(n - 1) {
        let (a, b) = (a - 1, b - 1);
        adj[a as usize].push(b);
        adj[b as usize].push(a);
    }
    let mut memo = vec![-1; 2 * n];
    out.print_line(doit(&adj, 0, -1, false, &mut memo));
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
