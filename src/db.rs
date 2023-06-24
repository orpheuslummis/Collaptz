

fn verify_receipt(receipt: Receipt) {
    receipt.verify(...)
}

fn append_to_local_log(receipt) {
    // we extract sequence and input and proof
    // store them in their respective types / columns
    // write local to log file CSV
}