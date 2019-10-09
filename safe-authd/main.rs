// Copyright 2019 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under The General Public License (GPL), version 3.
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied. Please review the Licences for the specific language governing
// permissions and limitations relating to use of the SAFE Network Software.

mod authd;
mod quic_client;
mod update;

use env_logger;
use log::error;
use std::process;

#[macro_use]
extern crate human_panic;

#[macro_use]
extern crate failure;
#[macro_use]
extern crate slog;

#[cfg(not(feature = "mock-network"))]
#[macro_use]
extern crate self_update;

use authd::run;

fn main() {
    setup_panic!();
    env_logger::init();

    if let Err(e) = run() {
        error!("safe-authd error: {}", e);
        process::exit(1);
    }
}
