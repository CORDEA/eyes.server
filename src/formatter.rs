use crate::client::response::AddressComponent;

pub fn format(addresses: &Vec<AddressComponent>) -> String {
    let mut result: String = String::new();

    if let Some(adrs) = find_by_type(addresses, "sublocality_level_2") {
        result += &adrs.long_name;
    }
    if let Some(adrs) = find_by_type(addresses, "locality") {
        if !result.is_empty() {
            result += ", "
        }
        result += &adrs.long_name;
    }
    if let Some(adrs) = find_by_type(addresses, "administrative_area_level_1") {
        if !result.is_empty() {
            result += ", "
        }
        result += &adrs.long_name;
    }
    result
}

fn find_by_type<'a>(addresses: &'a Vec<AddressComponent>, find_type: &'static str) -> Option<&'a AddressComponent> {
    addresses
        .iter()
        .find(|&address|
            address.types
                .iter()
                .find(|&typ| typ == find_type)
                .is_some()
        )
}
