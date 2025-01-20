//{"name":"E. Photoshoot for Gorillas","group":"Codeforces - Codeforces Round 966 (Div. 3)","url":"https://codeforces.com/contest/2000/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 4 2\n9\n1 1 1 1 1 1 1 1 1\n2 1 1\n2\n5 7\n20 15 7\n9\n4 1 4 5 6 1 1000000000 898 777\n1984 1 1\n4\n5 4 1499 2004\n9 5 5\n6\n6 7 14 16 16 6\n","output":"21\n12\n49000083104\n3512\n319\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPhotoshootForGorillas"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int();
    let m = input.read_int();
    let k = input.read_int();
    let w = input.read();
    let mut a = input.read_int_vec(w);
    a.sort();
    a.reverse();
    let mut b = vec![];
    for r in 0..n {
        for c in 0..m {
            let r_shifts = k.min(r + 1).min(n - r).min(n - k + 1);
            let c_shifts = k.min(c + 1).min(m - c).min(m - k + 1);
            b.push(r_shifts * c_shifts);
        }
    }
    b.sort();
    b.reverse();
    out.print_line(
        a.into_iter()
            .zip(b.into_iter())
            .map(|(a, b)| a as i64 * b as i64)
            .sum::<i64>(),
    );
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
