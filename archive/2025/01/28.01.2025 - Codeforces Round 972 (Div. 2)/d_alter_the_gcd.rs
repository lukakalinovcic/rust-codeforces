//{"name":"D. Alter the GCD","group":"Codeforces - Codeforces Round 972 (Div. 2)","url":"https://codeforces.com/contest/2005/problem/D","interactive":false,"timeLimit":4000,"tests":[{"input":"5\n8\n11 4 16 17 3 24 25 8\n8 10 4 21 17 18 25 21\n4\n6 4 24 13\n15 3 1 14\n2\n13 14\n5 8\n8\n20 17 15 11 21 10 3 7\n9 9 4 20 14 9 13 1\n2\n18 13\n15 20\n","output":"2 36\n3 2\n2 3\n2 36\n6 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DAlterTheGCD"}}}

use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn prefix_gcds<'a, I>(mut iter: I) -> Vec<i32>
where
    I: Iterator<Item = &'a i32>,
{
    let mut result = vec![];
    let mut g = *iter.next().unwrap();
    for &x in iter {
        let gg = gcd(g, x);
        if gg != g {
            result.push(g);
        }
        g = gg;
    }
    result.push(g);
    result
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);
    out.print_line(doit(&a, &b));
}

fn doit(a: &[i32], b: &[i32]) -> (i32, i64) {
    let n = a.len();
    let mut a_cands = vec![prefix_gcds(a.iter())].concat();
    let mut b_cands = vec![prefix_gcds(b.iter())].concat();
    a_cands.sort();
    a_cands.dedup();
    b_cands.sort();
    b_cands.dedup();

    let mut suffix = vec![(0, 0); n + 1];
    for i in (0..n).rev() {
        suffix[i] = (gcd(suffix[i + 1].0, a[i]), gcd(suffix[i + 1].1, b[i]));
    }

    let mut best_sum = 0;
    let mut new_cands = HashSet::new();
    let mut prev = (0, 0);
    for &a_cand in &a_cands {
        for &b_cand in &b_cands {
            if a_cand + b_cand < best_sum {
                continue;
            }
            let mut i = 0;
            while i < n && (gcd(a_cand, a[i]), gcd(b_cand, b[i])) == (a_cand, b_cand) {
                i += 1;
            }
            let mut a_gcd_prefix = a_cand;
            let mut b_gcd_prefix = b_cand;
            for j in i..n {
                a_gcd_prefix = gcd(a_gcd_prefix, b[j]);
                b_gcd_prefix = gcd(b_gcd_prefix, a[j]);
                if a_gcd_prefix + b_gcd_prefix < best_sum {
                    break;
                }

                let (a_gcd_suffix, b_gcd_suffix) = suffix[j + 1];
                let a_gcd = gcd(a_gcd_prefix, a_gcd_suffix);
                let b_gcd = gcd(b_gcd_prefix, b_gcd_suffix);
                if a_gcd + b_gcd > best_sum {
                    best_sum = a_gcd + b_gcd;
                    new_cands.clear();
                }
                if a_gcd + b_gcd == best_sum && (a_gcd, b_gcd) != prev {
                    new_cands.insert((a_gcd, b_gcd));
                    new_cands.insert((b_gcd, a_gcd));
                    prev = (a_gcd, b_gcd);
                }
            }
            if i == n {
                let a_gcd = a_cand;
                let b_gcd = b_cand;
                if a_gcd + b_gcd > best_sum {
                    best_sum = a_gcd + b_gcd;
                    new_cands.clear();
                }
                if a_gcd + b_gcd == best_sum && (a_gcd, b_gcd) != prev {
                    new_cands.insert((a_gcd, b_gcd));
                    new_cands.insert((b_gcd, a_gcd));
                    prev = (a_gcd, b_gcd);
                }
            }
        }
    }

    let mut sum_ways = 0;
    for (a_gcd, b_gcd) in new_cands {
        let mut curr: [i64; 3] = [1, 0, 0];
        for i in 0..n {
            let mut next: [i64; 3] = [0, 0, 0];
            let can_keep = (gcd(a_gcd, a[i]), gcd(b_gcd, b[i])) == (a_gcd, b_gcd);
            let can_swap = (gcd(a_gcd, b[i]), gcd(b_gcd, a[i])) == (a_gcd, b_gcd);
            if can_keep {
                next[0] = curr[0];
                next[2] = curr[1] + curr[2];
            }
            if can_swap {
                next[1] = curr[0] + curr[1];
            }
            curr = next;
        }
        let ways = curr[1] + curr[2];
        if ways > 0 {
            sum_ways += ways;
        }
    }
    (best_sum, sum_ways)
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

fn brute(a: &[i32], b: &[i32]) -> (i32, i64) {
    let n = a.len();
    let mut best_sum = 0;
    let mut best_ways = 0;
    for l in 0..n {
        for r in l..n {
            let mut a_gcd = 0;
            let mut b_gcd = 0;
            for i in 0..n {
                if i >= l && i <= r {
                    a_gcd = gcd(a_gcd, b[i]);
                    b_gcd = gcd(b_gcd, a[i]);
                } else {
                    a_gcd = gcd(a_gcd, a[i]);
                    b_gcd = gcd(b_gcd, b[i]);
                }
            }
            if a_gcd + b_gcd > best_sum {
                best_sum = a_gcd + b_gcd;
                best_ways = 0;
            }
            if a_gcd + b_gcd == best_sum {
                best_ways += 1;
            }
        }
    }
    (best_sum, best_ways)
}

fn main() {
    tester::run_tests();

    use rand::Rng;
    let mut rng = rand::thread_rng();
    for test_num in 1.. {
        if test_num % 1000 == 0 {
            eprintln!("test num: {test_num}");
        }
        let n = rng.gen_range(2..=30);
        let a = (0..n).map(|_| rng.gen_range(1..=100)).collect::<Vec<_>>();
        let b = (0..n).map(|_| rng.gen_range(1..=100)).collect::<Vec<_>>();
        let x = doit(&a, &b);
        let y = brute(&a, &b);
        if x != y {
            eprintln!("Mismatch at {a:?} {b:?}   {x:?} != {y:?}");
            break;
        }
    }
}
//END MAIN
