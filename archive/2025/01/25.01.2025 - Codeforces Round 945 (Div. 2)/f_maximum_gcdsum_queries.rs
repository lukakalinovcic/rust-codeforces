//{"name":"F. Maximum GCD Sum Queries","group":"Codeforces - Codeforces Round 945 (Div. 2)","url":"https://codeforces.com/contest/1973/problem/F","interactive":false,"timeLimit":5000,"tests":[{"input":"3 4\n1 2 3\n4 5 6\n1 1 1\n0 1 2 3\n","output":"2 3 3 3\n"},{"input":"5 5\n3 4 6 8 4\n8 3 4 9 3\n10 20 30 40 50\n5 55 13 1000 113\n","output":"2 7 3 7 7\n"},{"input":"1 1\n3\n4\n5\n0\n","output":"7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FMaximumGCDSumQueries"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let q = input.read();

    let a = input.read_int_vec(n);
    let b = input.read_int_vec(n);
    let c = input.read_int_vec(n);
    let cands = find_cands(&a, &b, &c);
    let mut result = vec![0; q];
    let mut qs = input
        .read_long_vec(q)
        .into_iter()
        .enumerate()
        .map(|(i, q)| (q, i))
        .collect::<Vec<_>>();
    qs.sort();
    let mut j = 0;
    let mut best = 0;
    for i in 0..qs.len() {
        while j < cands.len() && cands[j].0 <= qs[i].0 {
            best = best.max(cands[j].1);
            j += 1;
        }
        result[qs[i].1] = best;
    }

    out.print_iter(result.into_iter());
    out.print_line("");
}

fn add_to(a: &mut (i32, i64), b: (i32, i64)) {
    a.0 += b.0;
    a.1 += b.1;
}

fn find_cands(a: &Vec<i32>, b: &Vec<i32>, c: &Vec<i32>) -> Vec<(i64, i32)> {
    let n = a.len();
    let mut cands = vec![];
    for iter in 0..2 {
        let (x, y) = if iter == 0 {
            (a[0], b[0])
        } else {
            (b[0], a[0])
        };
        let (x_mapping, x_divs, x_primes_and_shifts) = init_divisors(x);
        let (y_mapping, y_divs, y_primes_and_shifts) = init_divisors(y);
        let n_x = x_divs.len();
        let n_y = y_divs.len();

        let mut cnt_and_cost = vec![vec![(0, 0); n_y]; n_x];
        for i in 1..n {
            let u = *x_mapping.get(&gcd(x, a[i])).unwrap();
            let v = *y_mapping.get(&gcd(y, b[i])).unwrap();
            add_to(&mut cnt_and_cost[u][v], (1, 0));

            let u = *x_mapping.get(&gcd(x, b[i])).unwrap();
            let v = *y_mapping.get(&gcd(y, a[i])).unwrap();
            add_to(&mut cnt_and_cost[u][v], (1, c[i] as i64));

            let g = gcd(a[i], b[i]);
            let u = *x_mapping.get(&gcd(x, g)).unwrap();
            let v = *y_mapping.get(&gcd(y, g)).unwrap();
            add_to(&mut cnt_and_cost[u][v], (-1, -c[i] as i64));
        }

        for (prime, shift) in x_primes_and_shifts {
            for u in (0..n_x).rev() {
                if x_divs[u] % prime == 0 {
                    for v in 0..n_y {
                        let t = cnt_and_cost[u][v];
                        add_to(&mut cnt_and_cost[u - shift][v], t);
                    }
                }
            }
        }
        for (prime, shift) in y_primes_and_shifts {
            for v in (0..n_y).rev() {
                if y_divs[v] % prime == 0 {
                    for u in 0..n_x {
                        let t = cnt_and_cost[u][v];
                        add_to(&mut cnt_and_cost[u][v - shift], t);
                    }
                }
            }
        }
        let base_cost = if iter == 0 { 0 } else { c[0] } as i64;
        for u in (0..n_x).rev() {
            for v in (0..n_y).rev() {
                let (cnt, cost) = cnt_and_cost[u][v];
                if cnt == n as i32 - 1 {
                    cands.push((base_cost + cost, x_divs[u] + y_divs[v]));
                }
            }
        }
    }
    cands.sort();
    cands
}

fn init_divisors(mut a: i32) -> (HashMap<i32, usize>, Vec<i32>, Vec<(i32, usize)>) {
    let mut prime_and_count = vec![];
    let mut div = 2;
    while div * div <= a {
        let mut count = 0;
        while a % div == 0 {
            count += 1;
            a /= div;
        }
        if count > 0 {
            prime_and_count.push((div, count));
        }
        div += 1;
    }
    if a > 1 {
        prime_and_count.push((a, 1));
    }
    let mut prime_and_shift = vec![];
    let mut mapping = HashMap::new();
    let mut divisors = vec![1];
    mapping.insert(1, 0);
    for (prime, count) in prime_and_count {
        let shift = divisors.len();
        for j in 0..shift * count {
            let next = divisors[j] * prime;
            divisors.push(next);
            mapping.insert(next, j + shift);
        }
        prime_and_shift.push((prime, shift));
    }
    (mapping, divisors, prime_and_shift)
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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

    use rand::Rng;
    let mut rng = rand::thread_rng();
    let hcn = vec![
        735134400, 698377680, 551350800, 367567200, 294053760, 245044800, 183783600, 147026880,
        122522400, 110270160, 73513440,
    ];
    let mut get_highly_composite = |_i| {
        let t = hcn[rng.gen_range(0..hcn.len())];
        t
    };
    let n = 500000;
    let a = (0..n).map(|i| get_highly_composite(i)).collect::<Vec<_>>();
    let b = (0..n).map(|i| get_highly_composite(i)).collect::<Vec<_>>();
    let c = (0..n).map(|_| rng.gen_range(1..=100)).collect::<Vec<_>>();

    let t0 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    _ = find_cands(&a, &b, &c);
    let t1 = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    eprintln!("{:?}", t1 - t0);
}
//END MAIN
