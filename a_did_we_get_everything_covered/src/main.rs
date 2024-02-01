//{"name":"A. Did We Get Everything Covered?","group":"Codeforces - Codeforces Round 921 (Div. 1)","url":"https://codeforces.com/problemset/problem/1924/A","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 2 4\nabba\n2 2 3\nabb\n3 3 10\naabbccabab\n","output":"YES\nNO\naa\nNO\nccc\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADidWeGetEverythingCovered"}}}

use std::env;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let m = input.read_size();

    let mut s = vec![0usize; m];
    for i in 0..m {
        s[i] = input.read_char() as usize - 'a' as usize;
    }

    let mut ans = vec![0u8; n];
    let mut consumed = 0usize;
    let mut constructed = false;
    for i in 0..n {
        let mut char_pos = vec![1000000000; k];
        for j in (consumed..m).rev() {
            char_pos[s[j]] = j;
        }
        if let Some(max_char_ref) = char_pos.iter().max() {
            if let Some(max_char_pos) = char_pos.iter().position(|x| x == max_char_ref) {
                if char_pos[max_char_pos] < m {
                    ans[i] = max_char_pos as u8;
                    consumed = char_pos[max_char_pos] + 1;
                } else {
                    ans[i] = max_char_pos as u8;
                    constructed = true;
                }
            }
        }
    }
    if constructed {
        out.print_line("NO");
        for ch in ans {
            out.print((ch + 'a' as u8) as char)
        }
        out.print_line("");
    } else {
        out.print_line("YES");
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}


//START MAIN
mod tester;

fn main() {
    if let Ok(pwd) = env::current_dir() {
        let pwd_str = pwd.to_str().unwrap();
        println!("PWD: {pwd_str}");
    }
    tester::run_tests();
}
//END MAIN
