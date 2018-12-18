/*
 * Copyright 2018 Intel Corporation
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ------------------------------------------------------------------------------
 */

use std::env;
use std::path::Path;
use std::process::Command;
use std::io::{self, Write};

fn main() {
    // Tell cargo to run bin/build_sgx_binary file
    let bin_path = Path::new("../../bin");
    assert!(env::set_current_dir(&bin_path).is_ok());

    println!("Successfully changed working directory to {}!", bin_path.display());

    // Spawn a process, wait for it to finish, and collect it's output
    let the_output = Command::new("/bin/bash").arg("-c").arg("source build_sgx_binary").output()
        .expect("Failed to execute."); 

    io::stdout().write_all(&the_output.stdout).unwrap();
    io::stderr().write_all(&the_output.stderr).unwrap();
}
