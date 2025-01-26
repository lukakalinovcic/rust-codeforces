//{"name":"D. Fixing a Binary String","group":"Codeforces - Codeforces Round 951 (Div. 2)","url":"https://codeforces.com/contest/1979/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n8 4\n11100001\n4 2\n1110\n12 3\n111000100011\n5 5\n00000\n6 1\n101001\n8 4\n01110001\n12 2\n110001100110\n","output":"3\n-1\n7\n5\n4\n-1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DFixingABinaryString"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let k = input.read();
    let s = input.read_line().into_bytes();
    out.print_line(doit(n, k, s));
}

fn doit(n: usize, k: usize, s: Vec<u8>) -> i32 {
    let mut prefix_k_alternating = vec![true; n];
    let mut prefix_running = vec![(0, 0); n];
    let mut running = (b'x', 0);
    let mut k_alternating = true;
    for i in 0..n {
        if s[i] != running.0 {
            running = (s[i], 1);
        } else {
            running.1 += 1;
        }
        if i > 0 && i < k {
            k_alternating &= s[i] == s[i - 1];
        }
        if i >= k {
            k_alternating &= s[i] != s[i - k];
        }
        prefix_running[i] = running;
        prefix_k_alternating[i] = k_alternating;
    }
    if running.1 == k && k_alternating {
        return n as i32;
    }

    let mut k_alternating = true;
    let mut running = (b'x', 0);
    for i in (0..n).rev() {
        if i + k < n {
            let m = n - i - 1;
            if running.1 == k && k_alternating {
                let want_running = (running.0 ^ ((m / k) % 2) as u8, k - m % k);
                if prefix_running[i] == want_running && prefix_k_alternating[i] {
                    return (i + 1) as i32;
                }
            }
        }

        if s[i] != running.0 {
            running = (s[i], 1);
        } else {
            running.1 += 1;
        }
        if i + k < n {
            k_alternating &= s[i] != s[i + k];
        }
    }
    -1
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

fn brute(n: usize, k: usize, s: Vec<u8>) -> Vec<i32> {
    let mut result = vec![];
    for p in 1..=n {
        let mut a = s[..p].to_vec();
        let b = s[p..].to_vec();
        a.reverse();
        let c = vec![b, a].concat();
        let mut ok = true;
        for i in 1..n {
            if i < k {
                if c[i] != c[i - 1] {
                    ok = false;
                    break;
                }
            } else {
                if c[i] == c[i - k] {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            result.push(p as i32);
        }
    }
    result
}

fn main() {
    tester::run_tests();

    use rand::Rng;
    let mut rng = rand::thread_rng();
    for test_num in 1.. {
        if test_num % 100 == 0 {
            eprintln!("test num: {test_num}");
        }
        let n = rng.gen_range(1..=20);
        let k = loop {
            let k = rng.gen_range(1..=n);
            if n % k == 0 {
                break k;
            }
        };
        let s = (0..n)
            .map(|_| rng.gen_range(b'0'..=b'1'))
            .collect::<Vec<_>>();

        let a = doit(n, k, s.clone());
        let b = brute(n, k, s.clone());
        if b.is_empty() && a != -1 || !b.is_empty() && !b.contains(&a) {
            eprintln!(
                "Mismatch at {n} {k} {}   {a} not in {b:?}",
                String::from_utf8(s).unwrap()
            );
            break;
        }
    }
}
//END MAIN
