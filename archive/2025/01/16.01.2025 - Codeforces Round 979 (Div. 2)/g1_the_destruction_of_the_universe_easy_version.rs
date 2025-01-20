//{"name":"G1. The Destruction of the Universe (Easy Version)","group":"Codeforces - Codeforces Round 979 (Div. 2)","url":"https://codeforces.com/contest/2030/problem/G1","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n3\n1 1\n2 3\n3 3\n4\n1 4\n2 3\n2 4\n1 1\n5\n1 2\n2 3\n3 4\n4 5\n1 5\n","output":"5\n6\n24\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"G1TheDestructionOfTheUniverseEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

const MAXN: usize = 5000;
const MOD: i32 = 998244353;

fn mult(a: i32, b: i32) -> i32 {
    let (a, b) = (a as i64, b as i64);
    ((a * b) % (MOD as i64)) as i32
}

fn add(a: i32, b: i32) -> i32 {
    (a + b) % MOD
}

fn sub(a: i32, b: i32) -> i32 {
    (a + MOD - b) % MOD
}

struct PreCalc {
    inv: Vec<i32>,
    fact: Vec<i32>,
    inv_fact: Vec<i32>,
}

fn choose(p: &PreCalc, n: usize, k: usize) -> i32 {
    if k > n {
        0
    } else {
        mult(p.fact[n], mult(p.inv_fact[k], p.inv_fact[n - k]))
    }
}

#[derive(Clone, Debug)]
struct Interval {
    l: i32,
    r: i32,
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, precalc: &mut PreCalc) {
    let n = input.read();
    let lr = input.read_int_pair_vec(n);
    let result = doit(lr, precalc);
    out.print_line(result);
}

fn doit(lr: Vec<(i32, i32)>, precalc: &PreCalc) -> i32 {
    let n = lr.len();
    let mut intervals = lr
        .into_iter()
        .map(|(l, r)| Interval { l, r })
        .collect::<Vec<_>>();
    intervals.push(Interval { l: 10000001, r: 0 });
    let mut asc_order = intervals.clone();
    let mut desc_order = intervals.clone();
    asc_order.sort_by_key(|interval| (interval.r, interval.l));
    desc_order.sort_by_key(|interval| (interval.l, -interval.r));
    desc_order.reverse();

    let mut pow2 = vec![0; n + 2];
    pow2[0] = 1;
    for i in 1..=n + 1 {
        pow2[i] = mult(pow2[i - 1], 2);
    }
    let mut result = 0;
    let mut j = n;
    for i in 1..=n {
        while j > 0 && !(asc_order[i].r < desc_order[j].l) {
            j -= 1;
        }
        for j in 1..=j {
            let sum_choose_prods = choose(precalc, j - 1 + i - 1, i - 1);
            let sum = mult(
                mult(sub(desc_order[j].l, asc_order[i].r), sum_choose_prods),
                pow2[n - i - j],
            );
            result = add(result, sum);
        }
    }

    result
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

fn precalc() -> PreCalc {
    let mut inv = vec![0; MAXN + 2];
    let mut fact = vec![0; MAXN + 2];
    let mut inv_fact = vec![0; MAXN + 2];
    inv[1] = 1;
    fact[0] = 1;
    fact[1] = 1;
    inv_fact[0] = 1;
    inv_fact[1] = 1;
    for i in 2..=MAXN + 1 {
        inv[i] = (MOD - mult(inv[(MOD % i as i32) as usize], MOD / i as i32)) % MOD;
        fact[i] = mult(fact[i - 1], i as i32);
        inv_fact[i] = mult(inv_fact[i - 1], inv[i]);
    }
    PreCalc {
        inv,
        fact,
        inv_fact,
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = precalc();
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

fn brute(lr: Vec<(i32, i32)>) -> i32 {
    let n = lr.len();
    let mut minx = 1000000;
    let mut maxx = 0;
    for (l, r) in &lr {
        let (l, r) = (*l, *r);
        minx = minx.min(l.min(r));
        maxx = maxx.max(l.max(r));
    }

    let mut result = 0;
    for mask in 1..(1 << n) {
        let mut best_result = 1000000000;
        for x in minx..=maxx as i32 {
            let mut curr_result = 0;
            for i in 0..n {
                if (mask >> i) & 1 == 1 {
                    let (l, r) = lr[i];
                    curr_result += if x < l {
                        l - x
                    } else if x > r {
                        x - r
                    } else {
                        0
                    };
                }
            }
            best_result = best_result.min(curr_result);
        }
        result = add(result, best_result);
    }
    result
}

fn main() {
    tester::run_tests();

    let pre_calc = precalc();
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for test_num in 1.. {
        if test_num % 100 == 0 {
            eprintln!("test num: {test_num}");
        }
        let n = rng.gen_range(1..=12);
        let lr: Vec<_> = (0..n)
            .map(|_| {
                let l = rng.gen_range(1..=n as i32);
                let r = rng.gen_range(1..=n as i32);
                (l.min(r), l.max(r))
            })
            .collect::<Vec<_>>();
        let a = doit(lr.clone(), &pre_calc);
        let b = brute(lr.clone());
        if a != b {
            eprintln!("Mismatch at {n} {lr:?}   {a} != {b}");
            break;
        }
    }
}
//END MAIN
