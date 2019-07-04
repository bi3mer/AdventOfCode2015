extern crate md5;

use indicatif::ProgressBar;

// fn find_value(secret_key: &str) -> String {
    
// }

fn main() {
    let input = "ckczppom";
    let bar = ProgressBar::new_spinner();

    let mut i = 0;
    loop {
        let input_str = format!("{}{}", input, i.to_string());

        let digest = md5::compute(&input_str);
        let stringified_md5 = format!("{:x}", digest);

        if stringified_md5.starts_with("000000") {
            bar.finish_with_message(&input_str);
            break
        }

        i += 1;
        bar.tick();
    }
}