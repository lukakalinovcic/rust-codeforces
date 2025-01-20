//{"name":"B. Kar Salesman","group":"Codeforces - Codeforces Round 978 (Div. 2)","url":"https://codeforces.com/contest/2022/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3 2\n3 1 2\n3 3\n2 1 3\n5 3\n2 2 1 9 2\n7 4\n2 5 3 3 5 2 5\n","output":"3\n3\n9\n7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BKarSalesman"}}}

use std::collections::BinaryHeap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let x: usize = input.read();
    let h = input.read_int_vec(n);
    out.print_line(doit(x, h));
}

fn doit(x: usize, h: Vec<i32>) -> i64 {
    let mut h = h.into_iter().map(|h| h as i64).collect::<Vec<_>>();
    h.sort();
    h.reverse();
    while h.len() < x {
        h.push(0);
    }
    h.push(0);

    let mut result = 0;
    let mut dh = (&h[0..x - 1])
        .iter()
        .map(|hi| *hi - h[x - 1])
        .collect::<Vec<_>>();
    for i in x - 1..h.len() - 1 {
        let mut reservoir_width = (i + 2 - x) as i64;
        let reservoir_height = (h[i] - h[i + 1]) as i64;
        let mut reservoir: i64 = reservoir_width * reservoir_height;
        for t in 0..x - 1 {
            dh[t] += reservoir_height;
        }

        for j in (0..x - 1).rev() {
            let k_from_reservoir = (x - j - 1) as i64;
            if dh[j] as i64 > reservoir / k_from_reservoir {
                let k = reservoir / k_from_reservoir;
                result += k;
                for t in 0..=j {
                    dh[t] -= k;
                }
                let remaining = reservoir % k_from_reservoir;
                for t in 0..remaining {
                    dh[j + t as usize + 1] = 1;
                }
                reservoir = 0;
                break;
            }
            let dd = k_from_reservoir - reservoir_width;

            let k = if dd <= 0 {
                0
            } else {
                (reservoir - dh[j] * reservoir_width + dd - 1) / dd
            };
            result += k;
            for t in 0..=j {
                dh[t] -= k;
            }
            reservoir -= k * k_from_reservoir;
            reservoir += dh[j];
            reservoir_width += 1;
            dh[j] = 0;
        }
        if reservoir > 0 {
            result += reservoir / x as i64;
            let remaining = reservoir % x as i64;
            for t in 0..remaining {
                dh[t as usize] = 1;
            }
        }
    }
    result + dh.first().cloned().unwrap_or_default()
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

fn brute(x: usize, a: Vec<i32>) -> i64 {
    let mut pq = BinaryHeap::from_iter(a.into_iter());
    let mut result: i64 = 0;
    while pq.len() > 0 {
        let mut ops = vec![];
        for _ in 0..x.min(pq.len()) {
            ops.push(pq.pop().unwrap());
        }
        result += 1;
        for op in ops {
            if op > 1 {
                pq.push(op - 1);
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
        let n = rng.gen_range(1..=30);
        let x = rng.gen_range(1..=10);
        let a: Vec<_> = (0..n).map(|_| rng.gen_range(1..10)).collect::<Vec<_>>();
        let s1 = doit(x, a.clone());
        let s2 = brute(x, a.clone());
        if s1 != s2 {
            eprintln!("Mismatch at {x} {a:?}   {s1} != {s2}");
            break;
        }
    }
}
//END MAIN
