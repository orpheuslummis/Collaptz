// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use ecdsa_methods::{ECDSA_VERIFY_ELF, ECDSA_VERIFY_ID};
use k256::{
    ecdsa::{signature::Signer, Signature, SigningKey, VerifyingKey},
    EncodedPoint,
};
use rand_core::OsRng;
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Executor, ExecutorEnv, SessionReceipt,
};

/// Given an secp256k1 verifier key (i.e. public key), message and signature,
/// runs the ECDSA verifier inside the zkVM and returns a receipt, including a
/// journal and seal attesting to the fact that the prover knows a valid
/// signature from the committed public key over the committed message.
fn prove_ecdsa_verification(
    verifying_key: &VerifyingKey,
    message: &[u8],
    signature: &Signature,
) -> Box<dyn SessionReceipt> {
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&(verifying_key.to_encoded_point(true), message, signature)).unwrap())
        .build()
        .unwrap();

    let mut exec = Executor::from_elf(env, ECDSA_VERIFY_ELF).unwrap();
    let session = exec.run().unwrap();
    session.prove().unwrap()
}

fn main() {
    // Generate a random secp256k1 keypair and sign the message.
    let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
    let message = b"This is a message that will be signed, and verified within the zkVM";
    let signature: Signature = signing_key.sign(message);

    // Run signature verified in the zkVM guest and get the resulting receipt.
    let receipt = prove_ecdsa_verification(signing_key.verifying_key(), message, &signature);

    // Verify the receipt and then access the journal.
    receipt.verify(ECDSA_VERIFY_ID.into()).unwrap();
    let (receipt_verifying_key, receipt_message) =
        from_slice::<(EncodedPoint, Vec<u8>), _>(&receipt.get_journal())
            .unwrap()
            .try_into()
            .unwrap();

    println!(
        "Verified the signature over message {:?} with key {}",
        std::str::from_utf8(&receipt_message[..]).unwrap(),
        receipt_verifying_key,
    );
}
