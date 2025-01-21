//{"name":"E. Nested Segments","group":"Codeforces - Codeforces Round 997 (Div. 2)","url":"https://codeforces.com/contest/2056/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1 0\n2 3\n1 1\n2 2\n1 2\n5 2\n1 3\n2 3\n4 1\n1 1\n6 2\n1 3\n4 6\n2300 0\n","output":"1\n1\n2\n5\n4\n187997613\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ENestedSegments"}}}

use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::modulo::modint::ModInt;

const MAXN: usize = 200000;
const MOD: u32 = 998244353;

struct PreCalc {
    fact: Vec<ModInt<MOD>>,
    inv_fact: Vec<ModInt<MOD>>,
}

fn catalan(precalc: &PreCalc, n: usize) -> ModInt<MOD> {
    precalc.fact[2 * n] * precalc.inv_fact[n + 1] * precalc.inv_fact[n]
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, precalc: &mut PreCalc) {
    let n = input.read();
    let m = input.read();
    let mut segments: HashSet<(i32, i32)> =
        HashSet::from_iter(input.read_int_pair_vec(m).into_iter());
    segments.insert((1, n as i32));
    for i in 1..=n {
        segments.insert((i as i32, i as i32));
    }

    let n = segments.len();
    let mut events = vec![];
    for (i, (l, r)) in segments.into_iter().enumerate() {
        events.push((l, 0, l - r, i));
        events.push((r, 1, r - l, i));
    }
    events.sort();
    let mut num_children = vec![0; n];
    let mut active = vec![];
    for (_x, t, _, i) in events {
        if t == 0 {
            active.push(i);
        } else {
            let ii = active.pop().unwrap();
            assert_eq!(ii, i);
            if !active.is_empty() {
                let p = active.last().cloned().unwrap();
                num_children[p] += 1;
            }
        }
    }

    let mut result: ModInt<MOD> = 1.into();
    for x in num_children {
        if x >= 1 {
            result *= catalan(precalc, x - 1);
        }
    }
    out.print_line(result.unwrap());
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let inv = ModInt::<MOD>::gen_inverses(2 * MAXN + 2);
    let mut fact = vec![ModInt::<MOD>::from(1)];
    let mut inv_fact = vec![ModInt::<MOD>::from(1)];
    for i in 1..2 * MAXN + 2 {
        fact.push(fact[i - 1] * i.into());
        inv_fact.push(inv_fact[i - 1] * inv[i]);
    }
    let mut pre_calc = PreCalc { fact, inv_fact };

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
