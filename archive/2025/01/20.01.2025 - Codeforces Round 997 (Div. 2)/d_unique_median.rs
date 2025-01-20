//{"name":"D. Unique Median","group":"Codeforces - Codeforces Round 997 (Div. 2)","url":"https://codeforces.com/contest/2056/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4\n1 1 1 1\n5\n1 10 2 3 3\n10\n6 3 2 3 5 3 4 2 3 5\n","output":"10\n11\n42\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DUniqueMedian"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn do_simple(a: &[i32]) -> i64 {
    let mut cnt_prefix_sum: HashMap<i32, i32> = HashMap::new();
    let mut prefix_sum: i32 = 0;
    let mut result = 0;
    for i in 0..a.len() {
        result += cnt_prefix_sum.get(&prefix_sum).cloned().unwrap_or_default() as i64;

        *cnt_prefix_sum.entry(prefix_sum).or_insert(0) += 1;
        prefix_sum += a[i];
        if a[i] == 0 {
            cnt_prefix_sum.clear();
            prefix_sum = 0;
        }
    }
    result += cnt_prefix_sum.get(&prefix_sum).cloned().unwrap_or_default() as i64;

    result
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let a = input.read_int_vec(n);
    out.print_line(doit(&a));
}

fn doit(a: &[i32]) -> i64 {
    let n = a.len();
    let (cnt_odd, cnt_even) = {
        let wh = (n + 1) / 2;
        let bl = n / 2;
        let cnt_odd = bl * (bl + 1) / 2 + wh * (wh + 1) / 2;
        let cnt_even = if n % 2 == 0 {
            wh * (wh + 1) / 2 + bl * (bl - 1) / 2
        } else {
            bl * (bl + 1) / 2 + bl * (bl + 1) / 2
        };
        (cnt_odd as i64, cnt_even as i64)
    };

    let mut cnt_bad = 0;
    for i in 1..=9 {
        cnt_bad += do_simple(
            &a.iter()
                .map(|a| if *a <= i { -1 } else { 1 })
                .collect::<Vec<_>>(),
        );
    }
    for i in 2..=9 {
        cnt_bad -= do_simple(
            &a.iter()
                .map(|a| {
                    if *a < i {
                        -1
                    } else if *a == i {
                        0
                    } else {
                        1
                    }
                })
                .collect::<Vec<_>>(),
        );
    }
    cnt_odd + cnt_even - cnt_bad
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
    let n = a.len();
    let mut result = 0;
    for i in 0..n {
        let mut b = vec![];
        for j in i..n {
            b.push(a[j]);
            b.sort();
            let m = b.len();
            if b[(m - 1) / 2] == b[m / 2] {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    tester::run_tests();

    use rand::Rng;
    let mut rng = rand::thread_rng();
    for test_num in 1.. {
        if test_num % 10000 == 0 {
            eprintln!("test num: {test_num}");
        }
        let n = rng.gen_range(1..=20);
        let a: Vec<_> = (0..n).map(|_| rng.gen_range(1..=10)).collect();
        let x = doit(&a);
        let y = brute(&a);
        if x != y {
            eprintln!("Mismatch at {a:?}   {x} vs. {y:?}");
            break;
        }
    }
}
//END MAIN
