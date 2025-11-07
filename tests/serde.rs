//! Tests for Serde-compatible API

#[cfg(feature = "serde")]
mod serde_tests {
    use serde::{Deserialize, Serialize};
    use toon_rust::{from_str, to_string};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Product {
        sku: String,
        qty: u32,
        price: f64,
    }

    #[test]
    fn test_serde_encode_decode() {
        let products = vec![
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
        ];

        let toon = to_string(&products).unwrap();
        let decoded: Vec<Product> = from_str(&toon).unwrap();
        assert_eq!(products, decoded);
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct User {
        id: u32,
        name: String,
        active: bool,
    }

    #[test]
    fn test_serde_simple_struct() {
        let user = User {
            id: 1,
            name: "Alice".to_string(),
            active: true,
        };

        let toon = to_string(&user).unwrap();
        let decoded: User = from_str(&toon).unwrap();
        assert_eq!(user, decoded);
    }
}

