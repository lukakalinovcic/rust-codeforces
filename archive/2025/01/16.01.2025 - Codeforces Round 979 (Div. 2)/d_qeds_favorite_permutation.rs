//{"name":"D. QED's Favorite Permutation","group":"Codeforces - Codeforces Round 979 (Div. 2)","url":"https://codeforces.com/contest/2030/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5 3\n1 4 2 5 3\nRLRLL\n2\n4\n3\n8 5\n1 5 2 4 8 3 6 7\nRRLLRRRL\n4\n3\n5\n3\n4\n6 2\n1 2 3 4 5 6\nRLRLRL\n4\n5\n","output":"YES\nYES\nNO\nNO\nYES\nNO\nNO\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DQEDsFavoritePermutation"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let q = input.read();
    let p = input.read_int_vec(n);
    let mut s = input.read_line().into_bytes();

    let mut irrelevant = vec![false; n];
    let mut mx = 0;
    for i in 0..n - 1 {
        mx = mx.max(p[i]);
        if mx == i as i32 + 1 {
            irrelevant[i + 1] = true;
        }
    }

    let mut num_uncool = 0;
    for i in 1..n {
        if irrelevant[i] {
            continue;
        }
        let is_cool = s[i - 1] == b'R' || s[i] == b'L';
        if !is_cool {
            num_uncool += 1;
        }
    }

    for i in input.read_int_vec(q) {
        let i = i as usize - 1;
        let was_cool = (0..2)
            .map(|offset| s[i + offset - 1] == b'R' || s[i + offset] == b'L')
            .collect::<Vec<_>>();
        s[i] = if s[i] == b'L' { b'R' } else { b'L' };
        let is_cool = (0..2)
            .map(|offset| s[i + offset - 1] == b'R' || s[i + offset] == b'L')
            .collect::<Vec<_>>();
        let irrelevant = irrelevant[i..i + 2].to_vec();
        for (irrelevant, (was_cool, is_cool)) in irrelevant
            .into_iter()
            .zip(was_cool.into_iter().zip(is_cool.into_iter()))
        {
            if !irrelevant {
                if !was_cool && is_cool {
                    num_uncool -= 1;
                }
                if was_cool && !is_cool {
                    num_uncool += 1;
                }
            }
        }
        if num_uncool == 0 {
            out.print_line("YES");
        } else {
            out.print_line("NO");
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
