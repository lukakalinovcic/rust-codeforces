//{"name":"D. Iris and Game on the Tree","group":"Codeforces - Codeforces Round 969 (Div. 2)","url":"https://codeforces.com/contest/2007/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n4\n1 2\n1 3\n4 1\n0101\n4\n1 2\n3 2\n2 4\n???0\n5\n1 2\n1 3\n2 4\n2 5\n?1?01\n6\n1 2\n2 3\n3 4\n5 3\n3 6\n?0????\n5\n1 2\n1 3\n1 4\n1 5\n11?1?\n2\n2 1\n??\n","output":"2\n1\n1\n2\n1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DIrisAndGameOnTheTree"}}}

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
    let s = input.read_line().into_bytes();
    let (zeros, ones, undecided_leaf, undecided_internal) = dfs(&adj, 0, n, &s);

    let result = match s[0] {
        b'0' => ones + (undecided_leaf + 1) / 2,
        b'1' => zeros + (undecided_leaf + 1) / 2,
        b'?' => {
            if ones == zeros && undecided_internal % 2 == 0 {
                ones.max(zeros) + (undecided_leaf + 1) / 2
            } else {
                ones.max(zeros) + undecided_leaf / 2
            }
        }
        _ => panic!("unexpected char"),
    };
    out.print_line(result);
}

fn dfs(adj: &Vec<Vec<usize>>, u: usize, p: usize, s: &Vec<u8>) -> (i32, i32, i32, i32) {
    let mut leaf = true;
    let mut zeros = 0;
    let mut ones = 0;
    let mut undecided_leaf = 0;
    let mut undecided_internal = 0;
    for &v in &adj[u] {
        if v == p {
            continue;
        }
        let (z, o, ul, ui) = dfs(adj, v, u, s);
        zeros += z;
        ones += o;
        undecided_leaf += ul;
        undecided_internal += ui;
        leaf = false;
    }
    if leaf {
        match s[u] {
            b'0' => zeros += 1,
            b'1' => ones += 1,
            b'?' => undecided_leaf += 1,
            _ => panic!("unexpected char"),
        }
    } else {
        if s[u] == b'?' {
            undecided_internal += 1;
        }
    }

    (zeros, ones, undecided_leaf, undecided_internal)
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
