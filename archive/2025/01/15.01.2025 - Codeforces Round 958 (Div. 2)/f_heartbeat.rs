//{"name":"F. Heartbeat","group":"Codeforces - Codeforces Round 958 (Div. 2)","url":"https://codeforces.com/contest/1988/problem/F","interactive":false,"timeLimit":5000,"tests":[{"input":"3\n1 1 1\n1 1 1\n1 1 1\n","output":"1 2 6\n"},{"input":"3\n1 2 3\n2 3 1\n3 1 2\n","output":"6 13 34\n"},{"input":"5\n1 4 2 5 3\n2 5 1 3 4\n300000000 100000000 500000000 400000000 200000000\n","output":"600000000 303511294 612289529 324650937 947905622\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FHeartbeat"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const MOD: i64 = 998244353;

const MAX: usize = 701;

fn mult(a: i32, b: i32) -> i32 {
    ((a as i64 * b as i64) % MOD) as i32
}

fn add(a: i32, b: i32) -> i32 {
    (a + b) % MOD as i32
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mx: usize = input.read();
    let a = vec![vec![0], input.read_long_vec(mx)].concat();
    let b = vec![vec![0], input.read_long_vec(mx)].concat();
    let c = vec![input.read_long_vec(mx), vec![0]].concat();

    let mut choose = [[0; MAX]; MAX];
    for n in 0..=mx {
        choose[n][0] = 1;
        choose[n][n] = 1;
        for k in 1..n {
            choose[n][k] = add(choose[n - 1][k], choose[n - 1][k - 1]);
        }
    }

    // f[n][pre][inc] => perm of size n, with p prefix maxes and i increases.
    let mut f = [[[0; MAX]; MAX]; 2];
    // af[l][inc] = sum over pre of a[pre + 1] * f[l][pre][inc]
    // bf[r][dec] = sum over suff of b[suff + 1] * f[r][suff][inc]
    let mut af = [[0; MAX]; MAX];
    let mut bf = [[0; MAX]; MAX];

    f[0][0][0] = 1;
    f[1][1][0] = 1;
    for n in 0..=mx {
        let curr = n % 2;
        if n > 1 {
            f[curr][0][0] = 0;
            let prev = (n - 1) % 2;
            for pre in 1..=n {
                for inc in 0..n {
                    f[curr][pre][inc] = f[prev][pre][inc];
                    f[curr][pre][inc] = add(f[curr][pre][inc], mult(inc as i32, f[prev][pre][inc]));
                    if inc >= 1 {
                        f[curr][pre][inc] = add(f[curr][pre][inc], f[prev][pre - 1][inc - 1]);
                        f[curr][pre][inc] = add(
                            f[curr][pre][inc],
                            mult((n - 1 - inc) as i32, f[prev][pre][inc - 1]),
                        );
                    }
                }
            }
        }

        if n < mx {
            let l = n;
            let max_inc = if l <= 1 { 0 } else { l - 1 };
            for inc in 0..=max_inc {
                for pre in 0..=l {
                    af[l][inc] = add(af[l][inc], mult(a[pre + 1], f[curr][pre][inc]));
                    let r = l;
                    let suff = pre;
                    let dec = (r as i32 - 1 - inc as i32).max(0) as usize;
                    bf[r][dec] = add(bf[r][dec], mult(b[suff + 1], f[curr][suff][inc]));
                }
            }
        }
    }

    // t[l][dec] = sum over inc of c[incdec] * af[l][inc];
    let mut caf = [[0; MAX]; MAX];
    for l in 0..mx {
        for dec in 0..=mx {
            for inc in 0..=l {
                let incdec = inc + dec + if l != 0 { 1 } else { 0 };
                if incdec > mx {
                    continue;
                }
                caf[l][dec] = add(caf[l][dec], mult(c[incdec], af[l][inc]));
            }
        }
    }

    let mut result = vec![0; mx + 1];
    for l in 0..mx {
        for r in 0..mx {
            let n = l + r + 1;
            if n > mx {
                continue;
            }
            for dec in 0..=r {
                result[n] = add(
                    result[n],
                    mult(mult(choose[n - 1][l], caf[l][dec]), bf[r][dec]),
                );
            }
        }
    }
    out.print_iter(result[1..].iter());
    out.print_line("");
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
}
//END MAIN
