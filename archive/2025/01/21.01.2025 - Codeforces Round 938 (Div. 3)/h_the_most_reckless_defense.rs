//{"name":"H. The Most Reckless Defense","group":"Codeforces - Codeforces Round 938 (Div. 3)","url":"https://codeforces.com/contest/1955/problem/H","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n2 2 1\n#.\n##\n1 2 1\n2 2 1\n#.\n##\n1 2 2\n2 2 1\n#.\n##\n1 2 500\n3 3 2\n#..\n##.\n.##\n1 2 4\n3 1 3\n3 5 2\n#.###\n#.#.#\n###.#\n2 2 2\n2 4 2\n5 5 4\n#....\n#....\n#....\n#....\n#####\n3 2 142\n4 5 9\n2 5 79\n1 3 50\n","output":"0\n1\n1491\n11\n8\n1797\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"HTheMostRecklessDefense"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

const MAXN: usize = 50;
const MAXR: i32 = 11;

struct PreCalc {
    ceil_sqrt: Vec<i32>,
}

impl PreCalc {
    fn new(n: usize) -> Self {
        let mut ceil_sqrt = vec![0; n + 1];
        let mut i = 0;
        for j in 0..=n {
            ceil_sqrt[j] = i as i32;
            if i * i == j {
                i += 1;
            }
        }
        Self { ceil_sqrt }
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, precalc: &mut PreCalc) {
    let n = input.read_int();
    let m = input.read_int();
    let k: usize = input.read();
    let a = (0..n)
        .map(|_| input.read_line().into_bytes())
        .collect::<Vec<_>>();
    let mut damages = vec![];
    for _ in 0..k {
        let r = input.read_int() - 1;
        let c = input.read_int() - 1;
        let base_damage = input.read_int();
        let mut damage = vec![0; MAXR as usize + 1];
        for dr in -MAXR..=MAXR {
            let rr = r + dr;
            if rr < 0 || rr >= n {
                continue;
            }
            for dc in -MAXR..=MAXR {
                let cc = c + dc;
                if cc < 0 || cc >= m {
                    continue;
                }
                if a[rr as usize][cc as usize] != b'#' {
                    continue;
                }
                let dist_sq = dr * dr + dc * dc;
                let d = precalc.ceil_sqrt[dist_sq as usize];
                if d <= MAXR {
                    damage[d as usize] += base_damage;
                }
            }
        }
        for d in 1..=MAXR {
            damage[d as usize] += damage[d as usize - 1];
        }
        damages.push(damage);
    }
    let mut prev = vec![0; 1 << MAXR];
    for damage in damages {
        let mut curr = vec![0; 1 << MAXR];
        for mask in 0..(1 << MAXR) {
            curr[mask] = curr[mask].max(damage[0] + prev[mask]);
            for d in 0..MAXR {
                if (mask >> d) & 1 == 0 {
                    curr[mask | (1 << d)] =
                        curr[mask | (1 << d)].max(damage[d as usize + 1] + prev[mask]);
                }
            }
        }
        prev = curr;
    }
    let mut pow3 = vec![0; MAXR as usize + 1];
    pow3[0] = 1;
    for d in 1..=MAXR {
        pow3[d as usize] = pow3[d as usize - 1] * 3;
    }
    let mut result = 0;
    for mask in 0..(1 << MAXR) {
        let mut extra_health = 0;
        for d in 0..MAXR {
            if (mask >> d) & 1 == 1 {
                extra_health += pow3[d as usize + 1];
            }
        }
        result = result.max(prev[mask] - extra_health);
    }
    out.print_line(result);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = PreCalc::new(2 * MAXN * MAXN);

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
