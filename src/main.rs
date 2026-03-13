fn main() {
    let digest = aws_lc_rs::digest::digest(&aws_lc_rs::digest::SHA256, b"hello world");
    println!("{:?}", digest);
}
