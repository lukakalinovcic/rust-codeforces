//{"name":"G. Yasya and the Mysterious Tree","group":"Codeforces - Codeforces Round 950 (Div. 3)","url":"https://codeforces.com/contest/1980/problem/G","interactive":false,"timeLimit":2500,"tests":[{"input":"2\n3 7\n1 2 1\n3 1 8\n^ 5\n? 2 9\n^ 1\n? 1 10\n^ 6\n? 3 1\n? 2 9\n5 6\n1 2 777\n3 2 2812\n4 1 16\n5 3 1000000000\n^ 4\n? 3 123\n? 5 1000000000\n^ 1000000000\n? 1 908070\n? 2 1\n","output":"13 15 11 10\n1000000127 2812 999756331 999999756\n"},{"input":"3\n8 4\n8 6 3\n6 3 4\n2 5 4\n7 6 2\n7 1 10\n4 1 4\n5 1 2\n^ 4\n^ 7\n? 7 8\n? 4 10\n5 6\n3 1 4\n2 3 9\n4 3 6\n5 2 10\n? 5 7\n^ 1\n^ 8\n? 4 10\n? 1 9\n? 3 6\n4 2\n2 1 4\n4 3 5\n2 3 4\n^ 13\n? 1 10\n","output":"14 13\n13 8 11 11\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GYasyaAndTheMysteriousTree"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

struct Node {
    l: i32,
    r: i32,
    cnt: i32,
}

struct Trie {
    a: Vec<Node>,
}

impl Trie {
    fn new() -> Self {
        Self {
            a: vec![Node {
                l: -1,
                r: -1,
                cnt: 0,
            }],
        }
    }

    fn update(&mut self, x: i32, delta: i32) {
        let mut i = 0;
        for bit in (0..30).rev() {
            self.a[i as usize].cnt += delta;
            i = if (x & (1 << bit)) == 0 {
                if self.a[i as usize].l == -1 {
                    self.a[i as usize].l = self.a.len() as i32;
                    self.a.push(Node {
                        l: -1,
                        r: -1,
                        cnt: 0,
                    });
                }
                self.a[i as usize].l
            } else {
                if self.a[i as usize].r == -1 {
                    self.a[i as usize].r = self.a.len() as i32;
                    self.a.push(Node {
                        l: -1,
                        r: -1,
                        cnt: 0,
                    });
                }
                self.a[i as usize].r
            };
        }
        self.a[i as usize].cnt += delta;
    }

    fn find(&self, search_pattern: i32) -> Option<i32> {
        if self.a[0].cnt == 0 {
            return None;
        }
        let mut i = 0;
        let mut x = 0;
        for bit in (0..30).rev() {
            let (l, r) = (self.a[i as usize].l, self.a[i as usize].r);
            if (search_pattern & (1 << bit)) == 0 {
                if r != -1 && self.a[r as usize].cnt > 0 {
                    i = r;
                    x |= 1 << bit;
                } else {
                    i = l;
                }
            } else {
                if l != -1 && self.a[l as usize].cnt > 0 {
                    i = l;
                } else {
                    i = r;
                    x |= 1 << bit;
                }
            }
        }
        Some(x)
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: usize = input.read();
    let m: usize = input.read();
    let mut adj = vec![vec![]; n];
    for _ in 1..n {
        let u = input.read::<usize>() - 1;
        let v = input.read::<usize>() - 1;
        let w = input.read_int();
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    let mut seen = vec![false; n];
    let mut root_path_xor = vec![0; n];
    let mut root_path_parity = vec![0; n];
    let mut q = vec![0];
    seen[0] = true;
    for i in 0..n {
        let u = q[i];
        for (v, w) in &adj[u] {
            let (v, w) = (*v, *w);
            if seen[v] {
                continue;
            }
            seen[v] = true;
            root_path_xor[v] = root_path_xor[u] ^ w;
            root_path_parity[v] = root_path_parity[u] ^ 1;
            q.push(v);
        }
    }

    let mut odd_trie = Trie::new();
    let mut even_trie = Trie::new();
    for i in 0..n {
        if root_path_parity[i] == 0 {
            even_trie.update(root_path_xor[i], 1);
        } else {
            odd_trie.update(root_path_xor[i], 1);
        }
    }

    let mut results = vec![];
    let mut global_xor = 0;
    for _ in 0..m {
        let op = input.read_char();
        match op {
            '^' => global_xor ^= input.read_int(),
            '?' => {
                let u = input.read::<usize>() - 1;
                let query_xor = input.read_int();

                if root_path_parity[u] == 0 {
                    even_trie.update(root_path_xor[u], -1);
                } else {
                    odd_trie.update(root_path_xor[u], -1);
                }

                let even_search_pattern = if root_path_parity[u] == 1 {
                    query_xor ^ global_xor ^ root_path_xor[u]
                } else {
                    query_xor ^ root_path_xor[u]
                };
                let odd_search_pattern = if root_path_parity[u] == 0 {
                    query_xor ^ global_xor ^ root_path_xor[u]
                } else {
                    query_xor ^ root_path_xor[u]
                };
                let result_odd = odd_trie
                    .find(odd_search_pattern)
                    .map(|x| x ^ odd_search_pattern)
                    .unwrap_or_default();
                let result_even = even_trie
                    .find(even_search_pattern)
                    .map(|x| x ^ even_search_pattern)
                    .unwrap_or_default();
                results.push(result_odd.max(result_even));

                if root_path_parity[u] == 0 {
                    even_trie.update(root_path_xor[u], 1);
                } else {
                    odd_trie.update(root_path_xor[u], 1);
                }
            }
            _ => panic!("unexpected op"),
        }
    }
    out.print_iter(results.into_iter());
    out.print_line("");
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
