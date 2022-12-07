mod pcre2;
use pcre2::*;

fn print_matched(target: &str, begin: usize, end: usize) {
    println!("{begin}: {end}");
    println!("  {}", &target[begin..end]);
}

fn main() {
    let target = r"a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999";
    let pattern = r"(?<=\d{4})[^\d\s]{3,11}(?=.)";

    for mat in Regex::new(pattern).unwrap().find_iter(target) {
        print_matched(target, mat.0, mat.1);
    }
}
