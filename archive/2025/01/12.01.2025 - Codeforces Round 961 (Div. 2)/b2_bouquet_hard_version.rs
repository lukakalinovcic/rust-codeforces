//{"name":"B2. Bouquet (Hard Version)","group":"Codeforces - Codeforces Round 961 (Div. 2)","url":"https://codeforces.com/contest/1995/problem/B2","interactive":false,"timeLimit":1500,"tests":[{"input":"7\n3 10\n1 2 3\n2 2 1\n3 1033\n206 207 1000\n3 4 1\n6 20\n4 2 7 5 6 1\n1 2 1 3 1 7\n8 100000\n239 30 610 122 24 40 8 2\n12 13123 112 1456 124 100 123 10982\n6 13\n2 4 11 1 3 5\n2 2 1 2 2 1\n8 10330\n206 210 200 201 198 199 222 1000\n9 10 11 12 13 14 15 16\n2 10000000000\n11 12\n87312315 753297050\n","output":"7\n1033\n19\n99990\n13\n10000\n9999999999\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"B2BouquetHardVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve1(m: i64, a: i64, c: i64) -> i64 {
    let k = c.min(m / a);
    k * a
}

fn solve2(m: i64, a: i64, c1: i64, c0: i64) -> i64 {
    let k0 = c0.min(m / (a - 1));
    let k1 = c1.min((m - k0 * (a - 1)) / a);

    let rm = m - k0 * (a - 1) - k1 * a;
    let adjust = rm.min(k0).min(c1 - k1);
    let k0 = k0 - adjust;
    let k1 = k1 + adjust;
    k0 * (a - 1) + k1 * a
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let m = input.read_long();
    let a = input.read_long_vec(n);
    let c = input.read_long_vec(n);
    let mut ac = a.into_iter().zip(c.into_iter()).collect::<Vec<_>>();
    ac.sort();
    let mut result = 0;
    for i in 0..n {
        let (a1, c1) = ac[i];
        result = result.max(solve1(m, a1, c1));
        if i > 0 {
            let (a0, c0) = ac[i - 1];
            if a0 + 1 == a1 {
                result = result.max(solve2(m, a1, c1, c0));
            }
        }
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
