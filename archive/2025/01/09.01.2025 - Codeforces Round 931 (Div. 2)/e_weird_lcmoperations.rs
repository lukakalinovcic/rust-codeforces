//{"name":"E. Weird LCM Operations","group":"Codeforces - Codeforces Round 931 (Div. 2)","url":"https://codeforces.com/contest/1934/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n4\n7\n","output":"1\n1 2 3\n1\n1 3 4\n3\n3 5 7\n5 6 7\n2 3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EWeirdLCMOperations"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn gen(f: &[(i64, i32)], i: usize, mut divisor: i64, mut factors: i32, out: &mut Vec<(i64, i32)>) {
    if i == f.len() {
        out.push((divisor, factors));
    } else {
        for _ in 0..=f[i].1 {
            gen(f, i + 1, divisor, factors, out);
            divisor *= f[i].0;
            factors += 1;
        }
    }
}

fn divisors(mut a: i64) -> Vec<(i64, i32)> {
    let mut f = vec![];
    for d in 2.. {
        if d * d > a {
            break;
        }
        let mut cnt = 0;
        while a % d == 0 {
            cnt += 1;
            a /= d;
        }
        f.push((d, cnt));
    }
    if a > 1 {
        f.push((a, 1))
    }
    let mut divisors = vec![];
    gen(&f, 0, 1, 0, &mut divisors);
    divisors
}

fn doit(n: usize) -> Vec<(usize, usize, usize)> {
    if n == 6 {
        return vec![(3, 4, 5)];
    } else if n == 8 {
        return vec![(5, 6, 7), (1, 3, 8)];
    } else if n == 14 {
        return vec![(14, 11, 5), (12, 13, 7), (10, 9, 1), (8, 2, 3)];
    }
    let mut used = vec![false; n + 1];
    let mut ops = vec![];
    for a in ((n + 1) / 2..=n).rev() {
        if used[a] {
            continue;
        }
        for c in (1..a - 1).rev() {
            if used[c] || gcd(a, c) != 1 {
                continue;
            }
            for b in (c + 1..a).rev() {
                if used[b] || gcd(a, b) != 1 || gcd(b, c) != 1 {
                    continue;
                }
                used[a] = true;
                used[b] = true;
                used[c] = true;
                ops.push((a, b, c));
                for x in [a * b, b * c, a * c] {
                    for (d, _) in divisors(x as i64) {
                        if d >= (n + 1) as i64 / 2 && d <= n as i64 {
                            used[d as usize] = true;
                        }
                    }
                }
                break;
            }
            if used[c] {
                break;
            }
        }
    }
    ops
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let ops = doit(n);
    out.print_line(ops.len());
    for (a, b, c) in ops {
        out.print_line(format!("{a} {b} {c}"));
    }
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

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn check(n: usize, ops: Vec<(usize, usize, usize)>) {
    let mut a: Vec<_> = (0..=n).collect();
    for (i, j, k) in ops {
        let x = a[i];
        let y = a[j];
        let z = a[k];
        a[i] = lcm(y, z);
        a[j] = lcm(x, z);
        a[k] = lcm(x, y);
    }
    let mut seen = vec![false; n + 1];
    for i in 1..=n {
        for j in 1..i {
            let g = gcd(a[i], a[j]);
            if g <= n {
                seen[g] = true;
            }
        }
    }
    for i in 1..=n {
        if !seen[i] {
            eprintln!("n={n} --- not seen {i}!!");
        }
    }
}

fn main() {
    tester::run_tests();

    for n in 3..=100 {
        let ops = doit(n);
        if ops.len() > n / 6 + 5 {
            eprintln!("{n} too many ops: {}> {}", ops.len(), n / 6 + 5);
        }
        check(n, ops);
    }
    // eprintln!("...");
    // for n in [6, 8, 14] {
    //     let ops = doit(n);
    //     eprintln!("n={} ops.len={}", n, ops.len());
    //     for (a, b, c) in ops {
    //         eprintln!(
    //             "{a} {b} {c}  =>  {} {} {}  =>  {} {} {}",
    //             b * c,
    //             a * c,
    //             a * b,
    //             gcd(b * c, a * c),
    //             gcd(b * c, a * b),
    //             gcd(a * b, a * c)
    //         );
    //     }
    // }
}
//END MAIN
