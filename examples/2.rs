extern crate support;

fn reverse_captcha(string: String, skip: usize) -> u32 {}

fn main() {
    let captcha = support::read_input("2-1").unwrap().trim().to_owned();
    println!("{}", reverse_captcha(captcha.clone(), 1));
    println!("{}", reverse_captcha(captcha.clone(), captcha.len() / 2));
}
