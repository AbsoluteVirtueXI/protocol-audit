use keystore_update::Keystore;
use std::env;

fn main() {
    fastrand::seed(0);
    let password = env::var("HOME").unwrap();
    let keystore = Keystore::new(password.as_str());
    println!("{:?}", keystore);
}
