/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

use aws_sdk_dynamodb::types::{AttributeValue, ReturnValue};
use aws_sdk_dynamodb::{Client, Error};

pub struct Item {
    pub key_id: String,
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct ItemOut {
    pub key_id: Option<AttributeValue>,
    pub name: Option<AttributeValue>,
}

// Add an item to a table.
// snippet-start:[dynamodb.rust.add-item]
pub async fn add_item(client: &Client, item: Item, table: &String) -> Result<(), Error> {
    let key_id_av = AttributeValue::S(item.key_id);
    let name_av = AttributeValue::S(item.name);

    let request = client
        .put_item()
        .table_name(table)
        .item("keyId", key_id_av)
        .item("name", name_av)
        .return_values(ReturnValue::None);

    // .return_values(ReturnValue::AllOld);
    // println!("Executing request [{request:?}] to add item...");

    let _resp = request.send().await?;

    // let attributes = resp.attributes().unwrap();
    // let key_id = attributes.get("KeyId").cloned();
    // let name = attributes.get("name").cloned();
    // println!("Added item {:?} {:?}", key_id, name);
    // Ok(ItemOut { key_id, name })

    Ok(())
}
