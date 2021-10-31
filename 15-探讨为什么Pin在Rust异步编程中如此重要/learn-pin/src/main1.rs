fn main() {
    let mut s1 = String::from("Hello");
    let s2 = s1; // s1的所有权转移给了s2，这里发生了move
    // let s3 = s1; // s1的所有权以及转移走了，不能再move，否则会报错：error[E0382]: use of moved value: `s1`
}