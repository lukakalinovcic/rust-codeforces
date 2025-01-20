//{"name":"D. Genokraken","group":"Codeforces - Codeforces Round 983 (Div. 2)","url":"https://codeforces.com/contest/2032/problem/D","interactive":true,"timeLimit":2000,"tests":[{"input":"3\n4\n\n1\n\n5\n\n1\n\n0\n\n9\n","output":"\n? 2 3\n\n! 0 0 1\n\n? 2 3\n\n? 2 4\n\n! 0 0 1 2\n\n! 0 0 0 1 3 5 6 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DGenokraken"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn query(input: &mut Input, out: &mut Output, a: usize, b: usize) -> usize {
    out.print_line(format!("? {a} {b}"));
    out.flush();
    input.read()
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read();
    let mut parent = vec![0; n];
    parent[1] = 0;
    let mut curr_layer = vec![1];
    let mut i = 2;
    while query(input, out, 1, i) == 1 {
        parent[i] = 0;
        curr_layer.push(i);
        i += 1;
    }
    while i < n {
        let mut next_layer = vec![];
        for u in curr_layer.iter() {
            let u = *u;
            if u == 1 || query(input, out, u, i) == 0 {
                parent[i] = u;
                next_layer.push(i);
                i += 1;
            }
            if i == n {
                break;
            }
        }
        curr_layer = next_layer;
    }
    out.print("! ");
    out.print_iter(parent[1..].iter());
    out.print("\n");
    out.flush();
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Interactive;

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
    let mut sin = std::io::stdin();
    let input = algo_lib::io::input::Input::new(&mut sin);
    let mut stdout = std::io::stdout();
    let output = algo_lib::io::output::Output::new(&mut stdout);
    run(input, output);
}

//END MAIN
