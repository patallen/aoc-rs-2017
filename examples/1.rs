extern crate support;

fn reverse_captcha(string: String, skip: usize) -> u32 {
    let captcha = string.chars().collect::<Vec<_>>();
    captcha
        .iter()
        .skip(skip)
        .chain(captcha.iter())
        .zip(captcha.iter())
        .fold(0, |s, (b1, b2)| match b1 == b2 {
            true => s + b2.to_string().parse::<u32>().unwrap(),
            false => s,
        })
}

fn main() {
    let captcha = support::read_input("1-1").unwrap().trim().to_owned();
    println!("{}", reverse_captcha(captcha.clone(), 1));
    println!("{}", reverse_captcha(captcha.clone(), captcha.len() / 2));
}
