fn main() {
    let target_txt = r"a;jhgoqoghqoj0329 u0tyu10hg0h9Y0Y9827342482y(Y0y(G)_)lajf;lqjfgqhgpqjopjqa=)*(^!@#$%^&*())9999999";
    let pattern = r"(?<=\D{4})\w{3,11}\w+(?=\w+)";
    println!("target: {}, pattern: {}", target_txt, pattern);
}
