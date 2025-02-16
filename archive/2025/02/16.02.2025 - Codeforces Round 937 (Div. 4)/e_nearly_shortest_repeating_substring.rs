//{"name":"E. Nearly Shortest Repeating Substring","group":"Codeforces - Codeforces Round 937 (Div. 4)","url":"https://codeforces.com/contest/1950/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4\nabaa\n4\nabba\n13\nslavicgslavic\n8\nhshahaha\n20\nstormflamestornflame\n","output":"1\n4\n13\n2\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ENearlyShortestRepeatingSubstring"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let s = input.read_line().into_bytes();
    for d in 1..=n {
        let k = n / d;
        if k * d != n {
            continue;
        }
        let mut fixes = 0;
        let mut ok = true;
        for i in 0..d {
            let mut cnt = vec![0; 26];
            for j in 0..k {
                let a = j * d + i;
                cnt[(s[a] - b'a') as usize] += 1;
            }
            let mut nonzero = vec![];
            for j in 0..26 {
                if cnt[j] > 0 {
                    nonzero.push(cnt[j]);
                }
            }
            if nonzero.len() > 2 || nonzero.len() == 2 && (nonzero[0] != 1 && nonzero[1] != 1) {
                ok = false;
                break;
            }
            if nonzero.len() == 2 {
                fixes += 1;
            }
        }
        if ok && fixes < 2 {
            out.print_line(d);
            break;
        }
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
