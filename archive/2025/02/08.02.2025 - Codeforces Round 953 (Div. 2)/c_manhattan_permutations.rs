//{"name":"C. Manhattan Permutations","group":"Codeforces - Codeforces Round 953 (Div. 2)","url":"https://codeforces.com/contest/1978/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n3 4\n4 5\n7 0\n1 1000000000000\n8 14\n112 777\n5 12\n5 2\n","output":"Yes\n3 1 2\nNo\nYes\n1 2 3 4 5 6 7\nNo\nYes\n8 2 3 4 5 6 1 7\nNo\nYes\n5 4 3 1 2\nYes\n2 1 3 4 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CManhattanPermutations"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let k = input.read_long();
    let p = doit(n, k);
    match p {
        None => out.print_line("No"),
        Some(p) => {
            out.print_line("Yes");
            out.print_line(p);
        }
    }
}

fn doit(n: usize, k: i64) -> Option<Vec<i32>> {
    if k % 2 == 1 {
        return None;
    }
    let mut p = vec![0; n];
    for i in 0..n {
        p[i] = i as i32 + 1;
    }
    if k == 0 {
        return Some(p);
    }
    let mut k = k;
    for i in 0..n / 2 {
        let t = (n - 2 * i - 1).min(k as usize / 2);
        k -= 2 * t as i64;
        p[i] = (i + t) as i32 + 1;
        p[i + t] = i as i32 + 1;
        if k == 0 {
            return Some(p);
        }
    }
    None
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
