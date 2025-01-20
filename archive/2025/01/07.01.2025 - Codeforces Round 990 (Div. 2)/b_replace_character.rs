//{"name":"B. Replace Character","group":"Codeforces - Codeforces Round 990 (Div. 2)","url":"https://codeforces.com/contest/2047/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n3\nabc\n4\nxyyx\n8\nalphabet\n1\nk\n10\naabbccddee\n6\nttbddq\n","output":"cbc\nyyyx\nalphaaet\nk\neabbccddee\ntttddq\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BReplaceCharacter"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let mut s = input.read_line().into_bytes();
    let mut cnt = vec![0; 26];
    for i in 0..n {
        let x = (s[i] - b'a') as usize;
        cnt[x] += 1;
    }
    let mut maxi = 0;
    let mut mini = 0;
    for i in 0..26 {
        if cnt[i] > cnt[maxi] {
            maxi = i;
        }
        if cnt[i] > 0 && (cnt[mini] == 0 || cnt[i] <= cnt[mini]) {
            mini = i;
        }
    }
    for i in 0..n {
        let x = (s[i] - b'a') as usize;
        if x == mini {
            s[i] = maxi as u8 + b'a';
            break;
        }
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
