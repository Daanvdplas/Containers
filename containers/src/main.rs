use containers::MyVec;

fn main() {
    let mut myvec = MyVec::new();
    println!("{myvec:?}");
    myvec.push("hallo");
    myvec.push("wereld");
    println!("{myvec:?}");

    let mut realvec = Vec::new();
    println!("{realvec:?}");
    realvec.push("hallo");
    realvec.push("wereld");
    println!("{realvec:?}");
}
