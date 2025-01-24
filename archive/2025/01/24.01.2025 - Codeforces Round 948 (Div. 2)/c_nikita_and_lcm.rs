//{"name":"C. Nikita and LCM","group":"Codeforces - Codeforces Round 948 (Div. 2)","url":"https://codeforces.com/contest/1977/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n5\n1 2 4 8 16\n6\n3 2 10 20 60 1\n7\n2 3 4 6 12 100003 1200036\n9\n2 42 7 3 6 7 7 1 6\n8\n4 99 57 179 10203 2 11 40812\n1\n1\n","output":"0\n4\n4\n5\n8\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CNikitaAndLCM"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i32 = 1000000000;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i32, b: i32) -> i32 {
    let g = gcd(a, b);
    let result = a as i64 * (b / g) as i64;
    if result > INF as i64 {
        0
    } else {
        result as i32
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut a = input.read_int_vec(n);
    a.sort();
    let an = a[n - 1];

    let mut full_lcm = 1;
    for i in 0..n {
        if full_lcm != 0 {
            full_lcm = lcm(full_lcm, a[i]);
        }
    }
    if full_lcm == 0 || full_lcm > an {
        out.print_line(n);
        return;
    }

    let mut divs = vec![];
    for d in 1.. {
        if an % d == 0 {
            divs.push(d);
            if an / d != d {
                divs.push(an / d);
            }
        }
        if d * d > an {
            break;
        }
    }
    divs.sort();

    let m = divs.len();
    let mut pos = HashMap::new();
    for i in 0..m {
        pos.insert(divs[i], i);
    }

    let mut curr = vec![-INF; m];
    curr[0] = 0;
    for i in 0..n {
        let mut next = curr.clone();
        for j in 0..m {
            let k = *pos.get(&lcm(divs[j], a[i])).unwrap();
            next[k] = next[k].max(curr[j] + 1);
        }
        curr = next;
    }
    for i in 0..n {
        curr[*pos.get(&a[i]).unwrap()] = 0;
    }
    out.print_line(curr.into_iter().max().unwrap_or_default());
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
}
//END MAIN
