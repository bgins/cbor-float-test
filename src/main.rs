use serde_cbor;
use serde_cbor::value::Value;
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let test_file = File::open("output/test.cbor")?;
    let val: Value = serde_cbor::from_reader(test_file)?;

    println!("{val:?}");

    Ok(())
}
