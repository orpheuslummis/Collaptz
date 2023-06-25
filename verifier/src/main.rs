use serde_json;
use clap::Parser;
use risc0_zkvm::SessionReceipt;
use std::path::Path;

#[derive(Parser)]
#[clap(about, version, author)]
struct Args {
    image_id: String,  // should be a bytearray (Vec[u8]) instead?
    receipt_file: String,  // receipt.dat
}

fn load_receipt(p: &Path) -> SessionReceipt {
    let data = std::fs::read(p).unwrap();
    risc0_zkvm::serde::from_slice(&data).unwrap()
}

fn extract_result_from_receipt() {
    //
}

/// When called from Command Line returns error code 0 if verified, otherwise panics.
fn main() {

    let args = Args::parse();

    // Deserialize image_id
    let image_id: [u32; 8] = serde_json::from_str(&args.image_id).expect("Failed to decode");
    println!("Image ID (program hash): {:?}", image_id);

    // Convert image_id to a Digest accepted by receipt.verify()
    let digest = risc0_zkvm::sha::Digest::new(image_id);

    // Load the receipt from file
    let receipt = load_receipt(Path::new(&args.receipt_file));

    // Retrieve the sequence that was computed
    // let sequence: Vec<u64> = from_slice(receipt.get_journal()).expect(
    //     "Journal output should deserialize into the same types (& order) that it was written",
    // );

    // Verify receipt
    receipt.verify(image_id).unwrap();
}
