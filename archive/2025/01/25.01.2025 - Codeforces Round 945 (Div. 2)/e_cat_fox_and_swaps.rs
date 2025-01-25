//{"name":"E. Cat, Fox and Swaps","group":"Codeforces - Codeforces Round 945 (Div. 2)","url":"https://codeforces.com/contest/1973/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n2\n2 1\n3\n3 1 2\n4\n3 2 1 4\n5\n5 3 1 2 4\n5\n1 2 3 4 5\n6\n3 2 1 5 4 6\n6\n1 3 2 4 5 6\n","output":"6\n11\n23\n29\n55\n46\n58\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ECatFoxAndSwaps"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let p = input.read_int_vec(n);
    let mut single_cands = vec![];
    let mut l = 2 * n;
    let mut r = 1;
    for i in 0..n {
        let x = p[i] as usize;
        let y = p[x as usize - 1] as usize;
        if x == y {
            continue;
        }
        single_cands.push(x + y);
        l = l.min(x.min(y) + n);
        r = r.max(x.max(y) + 1);
    }
    single_cands.sort();
    single_cands.dedup();

    let n = n as i64;
    let l = l as i64;
    let r = r as i64;
    let result = (4 * n - l - r) * (l - r + 1) / 2
        + (r - 1) * (2 * n - r + 1)
        + if single_cands.len() == 0 {
            2 * n
        } else if single_cands.len() == 1 {
            1
        } else {
            0
        };
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
