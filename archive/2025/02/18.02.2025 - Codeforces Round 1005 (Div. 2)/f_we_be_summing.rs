//{"name":"F. We Be Summing","group":"Codeforces - Codeforces Round 1005 (Div. 2)","url":"https://codeforces.com/contest/2064/problem/F","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n5 7\n1 2 3 4 5\n7 13\n6 6 6 6 7 7 7\n6 9\n4 5 6 6 5 1\n5 9\n5 5 4 5 5\n5 6\n3 3 3 3 3\n6 8\n4 5 4 5 4 5\n","output":"2\n12\n3\n8\n10\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FWeBeSumming"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let k = input.read_int();
    let a = input.read_int_vec(n);
    out.print_line(doit(&a, k));
}

fn doit(a: &[i32], k: i32) -> i64 {
    let n = a.len();
    let mut pos = vec![vec![]; n + 1];
    for i in 0..n {
        pos[a[i] as usize].push(i as i32);
    }
    let (prev_lt, prev_gte) = process_prev(a);
    let (next_gt, next_lte) = process_next(a);

    let mut result: i64 = 0;
    for i in 0..n {
        let b = k - a[i];
        if b < 1 || b > n as i32 {
            continue;
        }
        let next_eq_i = {
            let (Ok(t) | Err(t)) = pos[a[i] as usize].binary_search(&(i as i32));
            if t + 1 == pos[a[i] as usize].len() {
                n as i32
            } else {
                pos[a[i] as usize][t + 1]
            }
        };
        let i_end = next_lte[i];
        let (Ok(t) | Err(t)) = pos[b as usize].binary_search(&(i_end));
        let mut next_eq_j = {
            if t + 1 >= pos[b as usize].len() {
                n as i32
            } else {
                pos[b as usize][t + 1]
            }
        };
        let mut t = t;
        let left = (i as i32 - prev_lt[i]) as i64;
        if t < pos[b as usize].len() {
            let j = pos[b as usize][t];
            if next_eq_i == next_lte[i] {
                if j <= next_eq_i {
                    if b == a[i] {
                        let right = (next_gt[j as usize].min(next_eq_j) - j) as i64;
                        result += right * left;
                    } else {
                        let right = (next_gt[j as usize] - j) as i64;
                        result += right * left;
                    }
                }
            } else {
                let j_begin = prev_gte[j as usize];
                if j_begin < i_end {
                    let right = (next_gt[j as usize] - j) as i64;
                    result += right * left;
                }
            }
            next_eq_j = j;
        }
        while t > 0 && pos[b as usize][t - 1] > i as i32 {
            t -= 1;
            let j = pos[b as usize][t];
            let right = (next_gt[j as usize].min(next_eq_j) - j) as i64;
            result += right * left;
            next_eq_j = j;
        }
    }
    result
}

const INF: i32 = 1000000000;

fn process_prev(a: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut prev_lt = vec![];
    let mut prev_gte = vec![];
    let mut s_lt = vec![(-INF, -1)];
    let mut s_gte = vec![(INF, -1)];
    for i in 0..a.len() {
        while s_lt.last().unwrap().0 >= a[i] {
            s_lt.pop();
        }
        prev_lt.push(s_lt.last().unwrap().1);
        s_lt.push((a[i], i as i32));

        while s_gte.last().unwrap().0 < a[i] {
            s_gte.pop();
        }
        prev_gte.push(s_gte.last().unwrap().1);
        s_gte.push((a[i], i as i32));
    }
    (prev_lt, prev_gte)
}

fn process_next(a: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let a = a.iter().cloned().rev().map(|x| -x).collect::<Vec<_>>();
    let (prev_gt, prev_lte) = process_prev(&a);
    let n = a.len() as i32;
    let next_gt = prev_gt.into_iter().rev().map(|i| n - i - 1).collect();
    let next_lte = prev_lte.into_iter().rev().map(|i| n - i - 1).collect();
    (next_gt, next_lte)
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

fn brute(a: &[i32], k: i32) -> i64 {
    let n = a.len();
    let mut result = 0;
    for i in 0..n {
        for j in i + 1..n {
            let mut ok = false;
            for mid in i..j {
                let mut mini = INF;
                let mut maxi = -INF;
                for t in i..=mid {
                    mini = mini.min(a[t]);
                }
                for t in mid + 1..=j {
                    maxi = maxi.max(a[t]);
                }
                if mini + maxi == k {
                    ok = true;
                }
            }
            if ok {
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
        if test_num % 20000 == 0 {
            eprintln!("test num: {test_num}");
        }
        let n = rng.gen_range(2..=20);
        let k = rng.gen_range(n + 1..2 * n);
        let a = (0..n).map(|_| rng.gen_range(1..=n)).collect::<Vec<_>>();
        let x = doit(&a, k);
        let y = brute(&a, k);
        if x != y {
            eprintln!("Mismatch at {a:?} {k}  {x:?} != {y:?}");
            break;
        }
    }
}
//END MAIN
