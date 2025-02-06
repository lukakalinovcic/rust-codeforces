//{"name":"C. Mad MAD Sum","group":"Codeforces - Codeforces Round 960 (Div. 2)","url":"https://codeforces.com/contest/1990/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n1\n3\n2 2 3\n4\n2 1 1 2\n4\n4 4 4 4\n","output":"1\n13\n9\n40\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMadMADSum"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    out.print_line(doit(&a));
}

fn one_round(a: &[i32]) -> (i64, Vec<i32>) {
    let n = a.len();
    let mut cnt = vec![0; n + 1];
    let mut mx = 0;
    let mut b = vec![];
    let mut result = 0;
    for i in 0..n {
        result += a[i] as i64;
        cnt[a[i] as usize] += 1;
        if cnt[a[i] as usize] >= 2 {
            mx = mx.max(a[i]);
        }
        b.push(mx);
    }
    (result, b)
}

fn doit(a: &[i32]) -> i64 {
    let (mut result, a) = one_round(a);
    let n = a.len();
    let mut cnt = vec![0; n + 1];
    let mut mx = 0;
    let mut prev_mx;
    let mut run = 0;
    for i in 0..n {
        result += a[i] as i64;
        cnt[a[i] as usize] += 1;
        if cnt[a[i] as usize] >= 2 && a[i] > mx {
            prev_mx = mx;
            mx = a[i];
            result += prev_mx as i64 * (n - i + 1 + n - i + run) as i64 * run as i64 / 2;
            run = 1;
        } else {
            run += 1;
        }
    }
    result += mx as i64 * (run + 1) as i64 * run as i64 / 2;
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

fn brute(a: &[i32]) -> i64 {
    let mut a = a.to_vec();
    let mut result: i64 = 0;
    loop {
        let (add, b) = one_round(&a);
        if add == 0 {
            break;
        }
        result += add;
        a = b;
    }
    result
}

fn main() {
    tester::run_tests();

    use rand::Rng;
    let mut rng = rand::thread_rng();
    for test_num in 1.. {
        if test_num % 1000 == 0 {
            eprintln!("test num: {test_num}");
        }
        let n = rng.gen_range(2..=20);
        let a = (0..n).map(|_| rng.gen_range(1..=n)).collect::<Vec<_>>();
        let x = doit(&a);
        let y = brute(&a);
        if x != y {
            eprintln!("Mismatch at {a:?}   {x:?} != {y:?}");
            break;
        }
    }
}
//END MAIN
