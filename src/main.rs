use clap::Parser;

#[derive(Clone, Copy, Debug)]
struct CC2Randomness {
    num1: u8,
    num2: u8,
}

impl CC2Randomness {
    fn random(self: &mut Self) -> u8 {
        let mut n: u8 = (self.num1 >> 2).wrapping_sub(self.num1);
        if (self.num1 & 0x02) == 0 {
            n = n.wrapping_sub(1);
        }
        self.num1 = (self.num1 >> 1) | (self.num2 & 0x80);
        self.num2 = (self.num2 << 1) | (n & 0x01);
        return self.num1 ^ self.num2;
    }
    fn check_pattern(self: &Self, pattern: &Vec<u8>, pattern_modulo: u8) -> bool {
        let mut test_rand = self.clone();
        for exp_res in pattern {
            let our_res = test_rand.random() % pattern_modulo;
            if exp_res != &our_res {
                return false;
            }
        }
        return true;
    }
}

fn nat_search(pattern: &Vec<u8>) {
    let mut rand = CC2Randomness { num1: 0, num2: 0 };
    let mut iter_n = 0;
    while iter_n <= 256 * 256 {
        if rand.check_pattern(pattern, 4) {
            println!("Found pattern at iter {iter_n}.")
        }
        rand.random();
        iter_n += 1;
    }
}

fn all_search(pattern: &Vec<u8>) {
    for num1 in 0..=255 {
        for num2 in 0..=255 {
            let rand = CC2Randomness { num1, num2 };
            if rand.check_pattern(pattern, 4) {
                println!("Found pattern with {num1} {num2}.");
            }
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    all_search: bool,

    #[arg(short, long)]
    nat_saerch: bool,

    #[arg(short, long, required=true, num_args=1.., value_delimiter=',')]
    pattern: Vec<u8>,
}

fn main() {
    let args = Args::parse();
    if args.all_search {
        all_search(&args.pattern);
    }
    if args.nat_saerch {
        nat_search(&args.pattern);
    }
}
