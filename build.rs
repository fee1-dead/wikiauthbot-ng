fn main() {
    println!("cargo:rerun-if-changed=wikiauthbot-db/src/migrations");
}
