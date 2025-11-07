//! Serde-compatible API usage example

use serde::{Deserialize, Serialize};
use toon_rust::{from_str, to_string};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Product {
    sku: String,
    qty: u32,
    price: f64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct User {
    id: u32,
    name: String,
    products: Vec<Product>,
}

fn main() {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        products: vec![
            Product {
                sku: "A1".to_string(),
                qty: 2,
                price: 9.99,
            },
            Product {
                sku: "B2".to_string(),
                qty: 1,
                price: 14.5,
            },
        ],
    };

    // Serialize to TOON
    println!("Serializing to TOON:\n");
    let toon = to_string(&user).unwrap();
    println!("{}", toon);

    // Deserialize from TOON
    println!("\nDeserializing from TOON:\n");
    let decoded: User = from_str(&toon).unwrap();
    println!("Decoded: {:?}", decoded);
    assert_eq!(user, decoded);
}

