//{"name":"F2. Dyn-scripted Robot (Hard Version)","group":"Codeforces - Codeforces Round 963 (Div. 2)","url":"https://codeforces.com/contest/1993/problem/F2","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n2 4 2 2\nUR\n4 2 1 1\nLLDD\n6 3 3 1\nRLRRRL\n5 6 3 3\nRUURD\n7 5 3 4\nRRDLUUU\n7 123456789999 3 2\nULULURD\n","output":"1\n4\n3\n1\n1\n41152263332\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F2DynScriptedRobotHardVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let _n = input.read_size();
    let k = input.read_long();
    let w = input.read_long();
    let h = input.read_long();
    let s = input.read_line().into_bytes();
    let mut x = 0;
    let mut y = 0;
    let mut p = vec![];
    for ch in s {
        match ch {
            b'L' => x -= 1,
            b'R' => x += 1,
            b'U' => y += 1,
            b'D' => y -= 1,
            _ => panic!("unexpected char {ch}"),
        }
        p.push((x, y));
    }
    let mut result = 0;
    let (dx, dy) = (x, y);
    for (x0, y0) in p {
        let (x_k0, x_dk) = match doit_1d(x0, dx, w) {
            None => continue,
            Some((x_k0, x_dk)) => (x_k0, x_dk),
        };
        let (y_k0, y_dk) = match doit_1d(y0, dy, h) {
            None => continue,
            Some((y_k0, y_dk)) => (y_k0, y_dk),
        };
        let (k0, dk) = match crt(x_k0, x_dk, y_k0, y_dk) {
            None => continue,
            Some((k0, dk)) => (k0, dk),
        };
        result += ((k - k0 + dk - 1) / dk).max(0);
    }
    out.print_line(result);
}

fn crt(a: i64, m: i64, b: i64, n: i64) -> Option<(i64, i64)> {
    let (a, m, b, n) = if n > m { (b, n, a, m) } else { (a, m, b, n) };
    let (g, (x, _y)) = euclid(m, n);
    if (a - b) % g != 0 {
        None
    } else {
        let x = (b - a) % n * x % n / g * m + a;
        let period = m * n / g;
        if x < 0 {
            Some((x + m * n / g, period))
        } else {
            Some((x, period))
        }
    }
}

fn doit_1d(x0: i64, dx: i64, w: i64) -> Option<(i64, i64)> {
    // a * 2*w + b * dx == x0
    if dx == 0 {
        return if x0 % (2 * w) == 0 {
            Some((0, 1))
        } else {
            None
        };
    }
    let (dx, dx_flip) = if dx < 0 { (-dx, true) } else { (dx, false) };
    let (g, (_a, b)) = euclid(2 * w, dx);
    if x0 % g != 0 {
        None
    } else {
        let period = 2 * w / g;
        let mut first = {
            let k = x0 / g;
            if !dx_flip {
                -b * k
            } else {
                b * k
            }
        };
        if first < 0 {
            first += {
                let k = (-first + period - 1) / period;
                k * period
            };
        } else if first >= period {
            first -= {
                let k = first / period;
                k * period
            };
        }
        Some((first, period))
    }
}

fn euclid(a: i64, b: i64) -> (i64, (i64, i64)) {
    if b == 0 {
        (a, (1, 0))
    } else {
        let (g, (y, x)) = euclid(b, a % b);
        (g, (x, y - (a / b * x)))
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

fn main() {
    tester::run_tests();
}
//END MAIN
