//{"name":"C. Sums on Segments","group":"Codeforces - Educational Codeforces Round 173 (Rated for Div. 2)","url":"https://codeforces.com/contest/2043/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5\n1 -1 10 1 1\n5\n-1 -1 -1 -1 -1\n2\n-1 2\n2\n7 1\n3\n1 4 -1\n","output":"8\n-1 0 1 2 9 10 11 12\n6\n-5 -4 -3 -2 -1 0\n4\n-1 0 1 2\n4\n0 1 7 8\n6\n-1 0 1 3 4 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSumsOnSegments"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn process_subarray<'a, I>(it: I) -> (i32, i32)
where
    I: Iterator<Item = &'a i32>,
{
    let (mut best_lo, mut best_hi) = (0, 0);
    let (mut curr_lo, mut curr_hi) = (0, 0);
    for x in it {
        curr_lo = (curr_lo + x).min(0);
        curr_hi = (curr_hi + x).max(0);
        best_lo = best_lo.min(curr_lo);
        best_hi = best_hi.max(curr_hi);
    }
    (best_lo, best_hi)
}

fn process_prefix<'a, I>(it: I) -> (i32, i32)
where
    I: Iterator<Item = &'a i32>,
{
    let (mut best_lo, mut best_hi) = (0, 0);
    let mut curr = 0;
    for x in it {
        curr += x;
        best_lo = best_lo.min(curr);
        best_hi = best_hi.max(curr);
    }
    (best_lo, best_hi)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int() as usize;
    let a: Vec<_> = (0..n).map(|_| input.read_int()).collect();
    let mut pos = None;
    for i in 0..n {
        if a[i] != -1 && a[i] != 1 {
            pos = Some(i);
        }
    }
    let pos = pos.unwrap_or(0);

    let before = process_subarray(a[0..pos].iter());
    let after = process_subarray(a[pos + 1..n].iter());
    let rest = (before.0.min(after.0), before.1.max(after.1));

    let before = process_prefix(a[0..pos].iter().rev());
    let after = process_prefix(a[pos + 1..n].iter());
    let cross = (a[pos] + before.0 + after.0, a[pos] + before.1 + after.1);

    let mut result = vec![];
    for a in rest.0..=rest.1 {
        result.push(a);
    }
    for a in cross.0..=cross.1 {
        if a < rest.0 || a > rest.1 {
            result.push(a);
        }
    }
    result.sort();
    out.print_line(result.len());
    out.print_iter(result.into_iter());
    out.put(b'\n');
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
