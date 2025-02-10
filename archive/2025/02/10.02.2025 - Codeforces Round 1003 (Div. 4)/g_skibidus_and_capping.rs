//{"name":"G. Skibidus and Capping","group":"Codeforces - Codeforces Round 1003 (Div. 4)","url":"https://codeforces.com/contest/2065/problem/G","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4\n2 2 3 4\n6\n2 2 3 4 5 6\n9\n2 2 4 5 7 8 9 3 5\n","output":"5\n12\n18\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GSkibidusAndCapping"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

struct PreCalc {
    primes: Vec<i32>,
}

impl PreCalc {
    fn new() -> Self {
        const MAX: usize = 200000;
        let mut not_prime = vec![false; MAX];
        let mut primes = vec![];
        for i in 2..MAX {
            if not_prime[i] {
                continue;
            }
            primes.push(i as i32);
            let mut j = i * i;
            while j < MAX {
                not_prime[j] = true;
                j += i;
            }
        }
        Self { primes }
    }
}
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, precalc: &mut PreCalc) {
    let n = input.read();
    let mut cnt = vec![0; n + 1];
    for a in input.read_int_vec(n) {
        cnt[a as usize] += 1;
    }
    let p = &precalc.primes;
    let mut result: i64 = 0;
    for j in 0..p.len() {
        if p[j] > n as i32 {
            break;
        }
        let pj = p[j] as usize;
        let cj = cnt[pj] as i64;
        for i in 0..j {
            let pi = p[i] as usize;
            let ci = cnt[pi] as i64;
            result += ci * cj;
            let pij = pi * pj;
            if pij <= n {
                let cij = cnt[pi * pj] as i64;
                result += ci * cij;
                result += cj * cij;
                result += cij * (cij + 1) / 2;
            }
        }

        if pj * pj <= n {
            let cjj = cnt[pj * pj] as i64;
            result += cj * cjj;
            result += cjj * (cjj + 1) / 2;
        }
    }
    out.print_line(result);
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
