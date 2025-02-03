//{"name":"E. Carousel of Combinations","group":"Codeforces - Codeforces Round 940 (Div. 2) and CodeCraft-23","url":"https://codeforces.com/contest/1957/problem/E","interactive":false,"timeLimit":2500,"tests":[{"input":"4\n1\n3\n6\n314159\n","output":"0\n4\n24\n78926217\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ECarouselOfCombinations"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::modulo::modint::ModInt;

const MOD: u32 = 1000000007;

struct PreCalc {
    result: Vec<u32>,
}

impl PreCalc {
    fn new() -> Self {
        const MAX: usize = 1000000;
        let mut not_prime = vec![false; MAX + 1];
        not_prime[1] = true;
        for i in 2..=MAX {
            if not_prime[i] {
                continue;
            }
            let mut j = i * i;
            while j <= MAX {
                not_prime[j] = true;
                j += i;
            }
        }

        let mut delta_diff: Vec<ModInt<MOD>> = vec![0.into(); MAX + 1];
        for j in (0..=MAX).step_by(8) {
            if j + 4 <= MAX {
                delta_diff[j + 4] += 2.into();
            }
            if j + 8 <= MAX {
                delta_diff[j + 8] -= 2.into();
            }
        }
        for j in 2..=MAX {
            if not_prime[j] {
                continue;
            }
            for k in (j..=MAX).step_by(j) {
                if (k / j) % j == 1 {
                    delta_diff[k] += (j - 1).into();
                } else {
                    delta_diff[k] -= 1.into();
                }
            }
        }
        let mut delta: ModInt<MOD> = 0.into();
        let mut sum: ModInt<MOD> = 0.into();
        let mut result = vec![];
        for i in 0..=MAX {
            delta += delta_diff[i];
            sum += delta;
            result.push(sum.get());
        }

        Self { result }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, precalc: &mut PreCalc) {
    let n = input.read_size();
    out.print_line(precalc.result[n]);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = PreCalc::new();

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
