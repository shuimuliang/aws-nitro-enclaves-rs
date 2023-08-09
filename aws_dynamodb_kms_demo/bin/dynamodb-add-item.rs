/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

#![allow(clippy::result_large_err)]

use aws_dynamodb_kms_demo::{
    make_config,
    scenario::add::{add_item, Item},
    scenario::error::Error,
    BaseOpt,
};
use aws_sdk_dynamodb::{error::DisplayErrorContext, Client};
use clap::Parser;
use std::process;

#[derive(Debug, Parser)]
struct Opt {
    /// The key_id of the user
    #[structopt(short, long)]
    key_id: String,

    /// The name of the user
    #[structopt(short, long)]
    name: String,

    /// The table name.
    #[structopt(short, long)]
    table: String,

    #[structopt(flatten)]
    base: BaseOpt,
}

/// Adds an item to an Amazon DynamoDB table.
/// The table schema must use one of username, p_type, age, first, or last as the primary key.
/// # Arguments
///
/// * `-t TABLE` - The name of the table.
/// * `-k key_id` - The key_id of the new table item.
/// * `-n NAME` - The name of the new table item.
/// * `[-r REGION]` - The region in which the table is created.
///   If not supplied, uses the value of the **AWS_REGION** environment variable.
///   If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display additional information.
#[tokio::main]
async fn main() {
    if let Err(err) = run_example(Opt::parse()).await {
        eprintln!("Error: {}", DisplayErrorContext(err));
        process::exit(1);
    }
}

async fn run_example(
    Opt {
        key_id,
        name,
        table,
        base,
    }: Opt,
) -> Result<(), Error> {
    let shared_config = make_config(base).await?;
    let client = Client::new(&shared_config);

    add_item(&client, Item { key_id, name }, &table).await?;

    Ok(())
}
