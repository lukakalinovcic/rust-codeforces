//{"name":"C. Lazy Narek","group":"Codeforces - Codeforces Round 972 (Div. 2)","url":"https://codeforces.com/contest/2005/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5 2\nnn\naa\nrr\nee\nkk\n1 5\nnarek\n1 4\nnare\n5 7\nnrrarek\nnrnekan\nuuuuuuu\nppppppp\nnkarekz\n","output":"0\n5\n0\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CLazyNarek"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i32 = 1000000000;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let s = (0..n)
        .map(|_| input.read_line().into_bytes())
        .collect::<Vec<_>>();
    let narek = "narek".bytes().collect::<Vec<_>>();
    let mut curr = vec![-INF; 5];
    curr[0] = 0;
    for i in 0..n {
        let mut next = curr.clone();

        for start_offset in 0..5 {
            let mut offset = start_offset;
            let mut score = 0;
            for j in 0..m {
                if s[i][j] == narek[offset] {
                    offset = (offset + 1) % 5;
                    if offset == 0 {
                        score += 5;
                    }
                } else {
                    for k in 0..5 {
                        if s[i][j] == narek[k] {
                            score -= 1;
                        }
                    }
                }
            }
            next[offset] = next[offset].max(curr[start_offset] + score);
        }

        curr = next;
    }
    let mut result = 0;
    for offset in 0..5 {
        result = result.max(curr[offset] - offset as i32);
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
