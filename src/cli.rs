use std::env;

const FLAG_DYNAMIC_CONTRACTS: u8 = 1 << 0;

#[derive(Default, Debug)]
pub struct ParsedArgs {
    inpfiles: Vec<String>,
    outfile: String,
    flags: u8
}

impl ParsedArgs {
    pub fn check_dynamic_contracts(&self) -> bool {
        self.flags & FLAG_DYNAMIC_CONTRACTS != 0
    }

    fn set_dynamic_contracts(&mut self) {
        self.flags = self.flags | FLAG_DYNAMIC_CONTRACTS;
    }
}

// Note: According to the docs, on Unix systems globs will be expanded,
//       but not on Windows.
pub fn parse_args(args: &mut env::Args) -> ParsedArgs {
    let mut parsed = ParsedArgs::default();

    args.next(); // skip the filename
    let mut outflag: bool = false;
    for arg in args {
        if arg == "-d" && !outflag {
            parsed.set_dynamic_contracts();
        } else if arg == "-o" && !outflag {
            outflag = true;
        } else if outflag {
            parsed.outfile = arg;
            outflag = false;
        } else {
            parsed.inpfiles.push(arg);
        }
    }

    if outflag {
        panic!("Was expecting an output file name!");
    }

    parsed
}

