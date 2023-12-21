use std::{env, path::PathBuf};

use reqwest::blocking::ClientBuilder;
use typify::{TypeSpace, TypeSpaceSettings};

const WEBHOOK_SCHEMA: &str = "https://unpkg.com/@octokit/webhooks-schemas/schema.json";

/// Download .json and apply typify.
fn main() {
    let client = ClientBuilder::new()
        .build()
        .expect("Failed to create a client");

    let webhook_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("webhook.rs");
    let json = client
        .get(WEBHOOK_SCHEMA)
        .send()
        .expect("Failed")
        .json::<schemars::schema::RootSchema>()
        .expect("Failed");

    let mut type_space = TypeSpace::new(TypeSpaceSettings::default().with_struct_builder(true));
    type_space.add_root_schema(json).unwrap();

    let contents = format!(
        "{}\n{}",
        "use serde::{Deserialize, Serialize};",
        prettyplease::unparse(&syn::parse2::<syn::File>(type_space.to_stream()).unwrap())
    );

    std::fs::write(webhook_path, contents).unwrap();
}
