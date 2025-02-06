//{"name":"C. Turtle and Good Pairs","group":"Codeforces - Codeforces Round 968 (Div. 2)","url":"https://codeforces.com/contest/2003/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3\nabc\n5\nedddf\n6\nturtle\n8\npppppppp\n10\ncodeforces\n","output":"acb\nddedf\nurtlet\npppppppp\ncodeforces\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTurtleAndGoodPairs"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut s = input.read_line().into_bytes();
    let mut cnt = [0; 26];
    let mut prev = 26;
    for i in 0..n {
        cnt[(s[i] - b'a') as usize] += 1;
        prev = (s[i] - b'a') as usize;
    }
    for i in 0..n {
        let mut m = 26;
        for j in 0..26 {
            if j == prev || cnt[j] == 0 {
                continue;
            }
            if m == 26 || cnt[j] > cnt[m] {
                m = j;
            }
        }
        if m == 26 {
            s[i] = (prev as u8) + b'a';
        } else {
            s[i] = (m as u8) + b'a';
            prev = m;
        }
        cnt[prev] -= 1;
    }
    out.print_line(String::from_utf8(s).unwrap());
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
