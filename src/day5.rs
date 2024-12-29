use axum::{
    http::{header::CONTENT_TYPE, HeaderMap, StatusCode},
    response::IntoResponse,
};
use cargo_manifest::Manifest;
use toml::Table;

fn format_orders(orders: &Vec<toml::Value>) -> String {
    orders
        .iter()
        .filter_map(|order| {
            let item = order.get("item");
            let quantity = order.get("quantity");

            if item.is_none() || quantity.is_none() {
                return None;
            }

            let item_str = item.unwrap().as_str();
            let quantity_int = quantity.unwrap().as_integer();

            if item_str.is_none() || quantity_int.is_none() {
                return None;
            }

            Some(format!("{}: {}", item_str.unwrap(), quantity_int.unwrap()))
        })
        .collect::<Vec<String>>()
        .join("\n")
}

pub async fn day5_manifest(headers: HeaderMap, manifest: String) -> impl IntoResponse {
    println!("manifest: {:?}", manifest);

    let content_type = headers.get(CONTENT_TYPE);

    let allowed_content_types = vec!["application/toml", "application/yaml", "application/json"];

    if content_type.is_none() {
        return (
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "Unsupported Media Type".to_string(),
        );
    }

    let content_type = content_type.unwrap().to_str().unwrap();
    if !allowed_content_types.contains(&content_type) {
        return (
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "Unsupported Media Type".to_string(),
        );
    }

    // validate
    let cargo_manifest = Manifest::from_slice(manifest.as_bytes());

    match cargo_manifest {
        Ok(cargo_manifest) => {
            if cargo_manifest.package.is_none() {
                return (StatusCode::BAD_REQUEST, "Invalid manifest".to_string());
            }

            if cargo_manifest.package.as_ref().unwrap().keywords.is_none() {
                return (
                    StatusCode::BAD_REQUEST,
                    "Magic keyword not provided".to_string(),
                );
            }

            let keywords = cargo_manifest
                .package
                .unwrap()
                .keywords
                .unwrap()
                .as_local()
                .unwrap();

            println!("keywords: {:?}", keywords);

            if !keywords.contains(&"Christmas 2024".to_string()) {
                println!("Magic keyword not provided");

                return (
                    StatusCode::BAD_REQUEST,
                    "Magic keyword not provided".to_string(),
                );
            }
        }
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid manifest".to_string()),
    }

    let table = manifest.parse::<Table>().unwrap();

    let response = table.get("package").and_then(|package| {
        package.get("metadata").and_then(|metadata| {
            metadata.get("orders").and_then(|orders| {
                let orders_arr = orders.as_array();

                match orders_arr {
                    Some(orders) => {
                        let response = format_orders(orders);

                        if response.is_empty() {
                            return None;
                        }

                        return Some((StatusCode::OK, response));
                    }
                    None => None,
                }
            })
        })
    });

    match response {
        Some((status, response)) => (status, response),
        None => (StatusCode::NO_CONTENT, "Invalid manifest".to_string()),
    }
}

#[cfg(test)]
mod day5_tests {
    use toml::Table;

    #[test]
    fn day5_test4() {
        let manifest_str = r#"
[package]
name = "not-a-gift-order"
authors = ["Not Santa"]
keywords = ["Christmas 2024"]

[[package.metadata.orders]]
item = "Toy car"
quantity = 2
[[package.metadata.orders]]
item = "Lego brick"
quantity = 1.5
[[package.metadata.orders]]
item = "Doll"
quantity = 2
[[package.metadata.orders]]
quantity = 5
item = "Cookie:::\n"
[[package.metadata.orders]]
item = "Thing"
count = 3
"#;

        let table = manifest_str.parse::<Table>().unwrap();

        println!("{:?}", table);
        println!("{:?}", table["package"]["metadata"]["orders"]);
    }
}
