extern crate crypto;
use crypto::md5::Md5;

fn main() {
    let input = "ckczppom";

    let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
    assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
}