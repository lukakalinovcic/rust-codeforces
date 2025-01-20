//{"name":"E1. Billetes MX (Easy Version)","group":"Codeforces - Codeforces Round 978 (Div. 2)","url":"https://codeforces.com/contest/2022/problem/E1","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n3 3 8 0\n2 1 6\n3 2 12\n1 2 6\n2 2 0\n1 3 10\n1 1 0\n2 3 12\n3 1 10\n2 5 2 0\n1 1 10\n1 2 30\n","output":"1\n489373567\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1BilletesMXEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;
use algo_lib::modulo::utils::power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let m: usize = input.read();
    let k: usize = input.read();
    let q: usize = input.read();
    let mut component = vec![vec![]; n + m];
    let mut head = vec![0; n + m];
    let mut xor = vec![0; n + m];
    for i in 0..n + m {
        component[i].push(i);
        head[i] = i;
    }

    let mut still_valid = true;
    let mut num_components = n + m;
    for qi in 0..=k + q {
        if qi >= k {
            if still_valid {
                out.print_line(power::<1000000007>(2, 30 * (num_components - 1) as u32));
            } else {
                out.print_line(0);
            }
            if qi == k + q {
                break;
            }
        }
        let r: usize = input.read();
        let c: usize = input.read();
        let v = input.read_int();
        let i = r - 1;
        let j = n + c - 1;
        if head[i] == head[j] {
            if v != xor[i] ^ xor[j] {
                still_valid = false;
            }
        } else {
            let (ci, cj) = (head[i], head[j]);
            let (ci, cj) = if component[ci].len() >= component[cj].len() {
                (ci, cj)
            } else {
                (cj, ci)
            };
            let xor_diff = xor[i] ^ xor[j] ^ v;
            let mut tmp = component[cj].drain(..).collect::<Vec<_>>();
            for k in tmp.drain(..) {
                component[ci].push(k);
                xor[k] ^= xor_diff;
                head[k] = ci;
            }
            num_components -= 1;
        }
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
