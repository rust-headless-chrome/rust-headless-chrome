use auto_generate_cdp::init;

fn main() {
    println!("cargo::rerun-if-changed=json");
    init();
}
