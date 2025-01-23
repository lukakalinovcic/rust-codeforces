//{"name":"F. Missing Subarray Sum","group":"Codeforces - Codeforces Round 941 (Div. 2)","url":"https://codeforces.com/contest/1966/problem/F","interactive":false,"timeLimit":5000,"tests":[{"input":"7\n3\n1 2 3 4 1\n4\n18 2 11 9 7 11 7 2 9\n4\n5 10 5 16 3 3 13 8 8\n4\n8 10 4 6 4 20 14 14 6\n5\n1 2 3 4 5 4 3 2 1 1 2 3 2 1\n5\n1 1 2 2 2 3 3 3 3 4 5 5 6 8\n3\n500000000 1000000000 500000000 500000000 1000000000\n","output":"1 2 1\n7 2 2 7\n3 5 5 3\n6 4 4 6\n1 1 1 1 1\n2 1 2 1 2\n500000000 500000000 500000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FMissingSubarraySum"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let a = input.read_int_vec(n * (n + 1) / 2 - 1);
    let result = doit(n, a);
    out.print_iter(result.into_iter());
    out.print_line("");
}

fn doit(n: usize, mut a: Vec<i32>) -> Vec<i32> {
    a.sort();
    a.reverse();
    let sum_all = if a[0] != a[1] {
        a[0]
    } else {
        let mut i = 2;
        while i < a.len() {
            let mut j = i;
            while j < a.len() && a[i] == a[j] {
                j += 1;
            }
            if (j - i) % 2 == 1 {
                break;
            }
            i = j;
        }
        a[0] + a[1] - a[i]
    };
    let mut prefix = vec![];
    let mut to_take: HashMap<i32, i32> = HashMap::new();
    let mut i = 0;
    *to_take.entry(sum_all).or_default() += 1;
    prefix.push(0);
    while prefix.len() <= n / 2 {
        while i < a.len() {
            let e = to_take.entry(a[i]).or_default();
            if *e > 0 {
                *e -= 1;
            } else {
                break;
            }
            i += 1;
        }
        let psum = prefix.last().cloned().unwrap();
        let el = sum_all - psum - a[i];

        for p in &prefix {
            *to_take.entry(psum + el - *p).or_default() += 2;
        }
        let psum = psum + el;
        for p in &prefix {
            *to_take.entry(sum_all - psum - *p).or_default() += 2;
        }
        *to_take.entry(sum_all - 2 * psum).or_default() += 1;
        prefix.push(psum);
    }
    let mut result = vec![];
    for i in 1..prefix.len() {
        result.push(prefix[i] - prefix[i - 1]);
    }
    if n % 2 == 1 {
        let psum = prefix.last().cloned().unwrap();
        result.push(sum_all - 2 * psum);
    }
    for i in (1..prefix.len()).rev() {
        result.push(prefix[i] - prefix[i - 1]);
    }
    result
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

    use rand::Rng;
    let mut rng = rand::thread_rng();
    for test_num in 1.. {
        if test_num % 1000 == 0 {
            eprintln!("test num: {test_num}");
        }
        let n = rng.gen_range(3..=10);
        let mut a = vec![0; n];
        for i in 0..=n / 2 {
            let x = rng.gen_range(1..=30);
            a[i] = x;
            a[n - i - 1] = x;
        }
        let mut b = vec![];
        for i in 0..n {
            let mut sum = 0;
            for j in i..n {
                sum += a[j];
                b.push(sum);
            }
        }
        let mut c = b.clone();
        let k = rng.gen_range(0..c.len());
        c.remove(k);
        let aa = doit(n, c.clone());
        if aa != a {
            eprintln!("Mismatch!\n a: {a:?}\n b: {b:?}\n c: {c:?}\n got {aa:?}");
            break;
        }
    }
}
//END MAIN
