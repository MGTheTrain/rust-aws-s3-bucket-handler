// The MIT License
//
// Copyright (c) 2024 MGTheTrain
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use colored::Colorize;
use log::info;
use std::env;

pub mod connectors {
    pub mod blob_connector;
    pub mod aws_s3_bucket_handler;
}

pub fn are_env_vars_set(env_var_names: &[&str]) -> bool {
    let mut all_set = true;
    for &env_var_name in env_var_names {
        match env::var(env_var_name) {
            Ok(value) => {
                // info!("{} is set to: {}", env_var_name, value);
            }
            Err(_) => {
                let mut colored_string: colored::ColoredString;
                colored_string = format!("{} is not set.", env_var_name).red();
                info!("{}", colored_string);

                all_set = false;
            }
        }
    }
    all_set
}
