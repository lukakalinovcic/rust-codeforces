//{"name":"E. Reverse the Rivers","group":"Codeforces - Codeforces Round 984 (Div. 3)","url":"https://codeforces.com/contest/2036/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4 4\n1 3 5 9\n4 6 5 3\n2 1 2 7\n3\n1 > 4\n2 < 8\n1 < 6\n2\n1 < 8\n2 > 8\n1\n3 > 5\n2\n4 > 8\n1 < 8\n","output":"2\n-1\n3\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EReverseTheRivers"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let k = input.read();
    let q = input.read();
    let mut lower_bounds = vec![vec![]; k];
    let mut curr = vec![0; k];
    for i in 0..n {
        for (j, a) in input.read_int_vec(k).into_iter().enumerate() {
            if curr[j] != curr[j] | a {
                curr[j] |= a;
                lower_bounds[j].push((curr[j], i));
            }
        }
    }
    for _ in 0..q {
        let m = input.read();
        let mut lo = 0;
        let mut hi = n;
        for _ in 0..m {
            let r = (input.read_int() - 1) as usize;
            let o = input.read_char();
            let c = input.read_int();
            if o == '>' {
                let (Ok(i) | Err(i)) = lower_bounds[r].binary_search_by(|(x, _)| x.cmp(&(c + 1)));
                lo = lo.max(lower_bounds[r].get(i).unwrap_or(&(0, n)).1);
            } else {
                let (Ok(i) | Err(i)) = lower_bounds[r].binary_search_by(|(x, _)| x.cmp(&c));
                hi = hi.min(lower_bounds[r].get(i).unwrap_or(&(0, n)).1);
            }
        }
        if lo < hi {
            out.print_line(lo + 1);
        } else {
            out.print_line(-1);
        }
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
