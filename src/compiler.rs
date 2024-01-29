use crate::scanner::{Scanner, ScanResult};

pub(crate) fn compile(source: &str) {
    let mut line = -1;
    let mut scanner = Scanner::new(source);

    loop {
	let result = scanner.scan_token();

	match result {
	    ScanResult::Normal(mut token) => {
		if token.line != line {
		    print!("{:04} ", token.line);
		    line = token.line;
		} else {
		    print!("    | ");
		}

		println!("{:02}", token.get_lexeme());
	    },
	    ScanResult::EOF(_) => break,
	    ScanResult::Error(e) => panic!("{}", e.message),
	}

    }

}
