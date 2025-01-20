//{"name":"E. Best Price","group":"Codeforces - Codeforces Round 995 (Div. 3)","url":"https://codeforces.com/contest/2051/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2 0\n2 1\n3 4\n1 1\n2\n5\n3 3\n1 5 2\n3 6 4\n4 3\n2 3 2 8\n3 7 3 9\n3 1\n2 9 5\n12 14 9\n","output":"2\n5\n9\n14\n15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EBestPrice"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn read_and_sort(input: &mut Input, n: usize) -> Vec<i32> {
    let mut a: Vec<_> = (0..n).map(|_| input.read_int()).collect();
    a.sort();
    a
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let (n, k) = (input.read_int() as usize, input.read_int() as usize);
    let a = read_and_sort(input, n);
    let b = read_and_sort(input, n);
    let (mut i, mut j) = (0, 0);
    let mut result = 0;
    let mut buying = n;
    let mut bad = 0;
    while i < n || j < n {
        let a_i = a.get(i).cloned().unwrap_or(2100000000);
        let b_j = b.get(j).cloned().unwrap_or(2100000000);
        let x = a_i.min(b_j);
        if bad <= k {
            result = result.max(buying as u64 * x as u64);
        }
        while i < n && a[i] == x {
            bad += 1;
            i += 1;
        }
        while j < n && b[j] == x {
            bad -= 1;
            buying -= 1;
            j += 1;
        }
    }
    out.print_line(result);
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
