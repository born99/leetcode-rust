mod reverse_string;
use reverse_string::reverse_string;

fn main() {

    // Reverse String
    let mut s = vec!['h','e','l','l','o'];
    reverse_string(&mut s);
    dbg!(&s);
    eprintln!("{:#?}", &s);

    // println!(
    //    "cargo:rustc-env=EXP_RUST_BUILD_TIME={}",
    //    time::now_utc().strftime("%Y-%m-%d %H:%M:%S").unwrap()
    // );
    
}
