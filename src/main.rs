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

use factors::multiply_factors;
use factors_methods::MULTIPLY_ID;

// produce an execution receipt from the f(data) and then submit receipt to database system
fn main() {
    n = receive_data_from_user(); // because we want it to be an open computational task
    let (receipt, _) = perform_zkcollatz(n);

    receipt.verify(MULTIPLY_ID.into()).expect(
        "Code you have proven should successfully verify; did you specify the correct image ID?",
    );

    send_receipt_to_database(receipt); // let's assume this is remote
}



