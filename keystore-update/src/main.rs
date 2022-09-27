use keystore_update::Keystore;

fn main() {
    let (key, keystore) = Keystore::create();
    println!(
        "Save your encryption key in a secret place: {}",
        hex::encode(&key)
    );
    println!("{:?}", keystore);
}
