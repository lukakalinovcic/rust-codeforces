//{"name":"G. D-Function","group":"Codeforces - Codeforces Round 952 (Div. 4)","url":"https://codeforces.com/contest/1985/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n0 1 4\n0 2 7\n1 2 1\n1 2 3\n582 74663 3\n0 3 1\n","output":"2\n3\n90\n12\n974995667\n999\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GDFunction"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::modulo::modint::ModInt;

type PreCalc = ();

const MOD: u32 = 1000000007;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let l = input.read_int();
    let r = input.read_int();
    let k = input.read_int();
    out.print_line((solve1(r, k) - solve1(l, k)).unwrap());
}

fn sum_powers(a: ModInt<MOD>, n: i32) -> ModInt<MOD> {
    if n == 1 {
        1.into()
    } else if n % 2 == 1 {
        sum_powers(a, n - 1) * a + 1.into()
    } else {
        let x = sum_powers(a, n / 2);
        x * a.power((n / 2) as u32) + x
    }
}

fn solve1(n: i32, k: i32) -> ModInt<MOD> {
    if n == 0 {
        return 0.into();
    }
    let digits: ModInt<MOD> = (9 / k).into();
    let x = sum_powers(digits + 1.into(), n);
    x * digits
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
