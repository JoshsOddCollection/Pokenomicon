use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Card {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    card_type: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    number: Option<String>,
    #[serde(default)]
    artist: Option<String>,
    #[serde(default)]
    rarity: Option<String>,
    #[serde(default)]
    variants: Vec<Variant>,
    #[serde(default)]
    error_variants: Vec<ErrorVariant>,
    #[serde(default)]
    cameos: Option<Vec<String>>,
    #[serde(default)]
    animal_cameos: Option<Vec<String>>,
    #[serde(default)]
    pokeball_cameos: Option<Vec<String>>,
    #[serde(default)]
    is_first_art_appearance_for_language: Option<bool>,
    #[serde(default)]
    id_of_first_art_appearance_for_language: Option<String>,

    // Catch extra fields
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Variant {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    variant: Option<String>,
    // This field should only exist if true
    #[serde(default)]
    uncorrected_error: Option<bool>,
    #[serde(default)]
    notes: Option<String>,

    // Catch extra fields
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ErrorVariant {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    error_name: Option<String>,
    #[serde(default)]
    error_type: Option<String>,
    #[serde(default)]
    error_side: Option<String>,
    #[serde(default)]
    notes: Option<ErrorNotes>,

    // Catch extra fields
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct ErrorNotes {
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    print_run: Option<String>,

    // Catch extra fields
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

fn parse_error_index(error_id: &str) -> Option<u32> {
    error_id.rsplit("_e_").next()?.parse().ok()
}

fn validate_cards(cards: &[Card]) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    let mut card_ids = HashSet::new();
    let mut variant_ids = HashSet::new();
    let mut error_ids = HashSet::new();
    let mut error_names: HashMap<String, String> = HashMap::new();

    let valid_card_types = ["Pokémon", "Trainer"];
    let valid_error_sides = ["Front", "Back", "Both"];
    let valid_error_types = [
        "Additional Ink",
        "Design",
        "Misaligned Print Layer",
        "Obstruction",
        "Printer Hickey",
    ];
    let card_id_regex = Regex::new(r"^[a-z0-9]+_\d+$").unwrap();
    let variant_id_regex = Regex::new(r"^[a-z0-9]+_\d+_\d+$").unwrap();
    let error_id_regex = Regex::new(r"^[a-z0-9]+_\d+_\d+_e_\d+$").unwrap();

    for card in cards {
        let card_id = match &card.id {
            Some(id) => id,
            None => {
                errors.push("Card missing id".to_string());
                continue;
            }
        };

        // Card ID format & uniqueness
        if !card_id_regex.is_match(card_id) {
            errors.push(format!("Invalid card id format: {}", card_id));
        }
        if !card_ids.insert(card_id) {
            errors.push(format!("Duplicate card id: {}", card_id));
        }

        // Card type
        match &card.card_type {
            Some(ct) => {
                if !valid_card_types.contains(&ct.as_str()) {
                    errors.push(format!("Invalid card_type for {}: {}", card_id, ct));
                }
            }
            None => errors.push(format!("Card {} missing card_type", card_id)),
        }

        // Extra fields in card
        for key in card.extra.keys() {
            errors.push(format!("Unexpected field in card {}: {}", card_id, key));
        }

        // Variants
        for variant in &card.variants {
            let variant_id = match &variant.id {
                Some(id) => id,
                None => {
                    errors.push(format!("Variant missing id for card {}", card_id));
                    continue;
                }
            };

            if !variant_id_regex.is_match(variant_id) {
                errors.push(format!("Invalid variant id format: {}", variant_id));
            }
            if !variant_ids.insert(variant_id) {
                errors.push(format!("Duplicate variant id: {}", variant_id));
            }
            if !variant_id.starts_with(card_id) {
                errors.push(format!(
                    "Variant id does not start with card id: {} (card {})",
                    variant_id, card_id
                ));
            }

            if variant.uncorrected_error == Some(false) {
                errors.push(format!(
                    "Variant {} has uncorrected_error set to false, which is not allowed",
                    variant.id.as_deref().unwrap_or("<unknown>")
                ));
            }

            for key in variant.extra.keys() {
                errors.push(format!(
                    "Unexpected field in variant {}: {}",
                    variant_id, key
                ));
            }
        }

        let mut error_indices_by_variant: HashMap<String, Vec<u32>> = HashMap::new();

        // Error variants
        for error in &card.error_variants {
            let error_id = match &error.id {
                Some(id) => id,
                None => {
                    errors.push(format!("Error variant missing id for card {}", card_id));
                    continue;
                }
            };

            // Track error indices to make sure the error numbers are sequential
            if let Some(index) = parse_error_index(error_id) {
                if let Some(variant) = card.variants.iter().find(|v| {
                    v.id.as_deref()
                        .map(|vid| error_id.starts_with(vid))
                        .unwrap_or(false)
                }) {
                    if let Some(variant_id) = &variant.id {
                        error_indices_by_variant
                            .entry(variant_id.clone())
                            .or_default()
                            .push(index);
                    }
                }
            }

            match &error.error_name {
                Some(name) if !name.trim().is_empty() => {
                    let normalized_error_name = name.trim().to_lowercase();

                    // error_name needs to be unique
                    if let Some(first_id) =
                        error_names.insert(normalized_error_name.clone(), error_id.to_string())
                    {
                        errors.push(format!(
                            "Duplicate error_name '{}' used by {} and {}",
                            name, first_id, error_id
                        ));
                    }

                    // error_name must contain the card name
                    if let Some(card_name) = &card.name {
                        if !name.contains(card_name) {
                            errors.push(format!(
                                "error_name '{}' for {} must contain the card name '{}'",
                                name, error_id, card_name
                            ));
                        }
                    }
                }
                Some(_) => {
                    errors.push(format!(
                        "Error variant {} has an empty error_name",
                        error_id
                    ));
                }
                None => {
                    errors.push(format!("Error variant {} is missing error_name", error_id));
                }
            }

            if !error_id_regex.is_match(error_id) {
                errors.push(format!("Invalid error variant id format: {}", error_id));
            }
            if !error_ids.insert(error_id) {
                errors.push(format!("Duplicate error variant id: {}", error_id));
            }

            if !card.variants.iter().any(|v| {
                if let Some(vid) = &v.id {
                    error_id.starts_with(vid)
                } else {
                    false
                }
            }) {
                errors.push(format!(
                    "Error variant id does not start with a variant id: {}",
                    error_id
                ));
            }

            match &error.error_side {
                Some(side) => {
                    if !valid_error_sides.contains(&side.as_str()) {
                        errors.push(format!("Invalid error_side for {}: {}", error_id, side));
                    }
                }
                None => errors.push(format!("Error variant missing error_side: {}", error_id)),
            }

            match &error.error_type {
                Some(error_type) => {
                    if !valid_error_types.contains(&error_type.as_str()) {
                        errors.push(format!(
                            "Invalid error_type for {}: {}",
                            error_id, error_type
                        ));
                    }
                }
                None => {
                    errors.push(format!("Error variant {} is missing error_type", error_id));
                }
            }

            match &error.notes {
                Some(notes) => {
                    if notes.description.as_deref().unwrap_or("").is_empty() {
                        errors.push(format!("Error description missing for {}", error_id));
                    }

                    for key in notes.extra.keys() {
                        errors.push(format!(
                            "Unexpected field in error notes {}: {}",
                            error_id, key
                        ));
                    }
                }
                None => errors.push(format!("Error notes missing for {}", error_id)),
            }

            for key in error.extra.keys() {
                errors.push(format!(
                    "Unexpected field in error variant {}: {}",
                    error_id, key
                ));
            }
        }

        // Validate that the error indices are sequential
        for (variant_id, mut indices) in error_indices_by_variant {
            indices.sort_unstable();

            for (expected, actual) in (1u32..).zip(indices.iter()) {
                if *actual != expected {
                    errors.push(format!(
                        "Error variant ids for variant {} are not sequential: expected _e_{}, found _e_{}",
                        variant_id, expected, actual
                    ));
                    break;
                }
            }
        }

        // Cameos
        for (name, field) in [
            ("cameos", &card.cameos),
            ("animal_cameos", &card.animal_cameos),
            ("pokeball_cameos", &card.pokeball_cameos),
        ] {
            if let Some(list) = field {
                if list.iter().any(|s| s.is_empty()) {
                    errors.push(format!(
                        "{} contains empty string for card {}",
                        name, card_id
                    ));
                }
            }
        }

        // First art appearance
        if matches!(card.is_first_art_appearance_for_language, Some(false)) {
            if card.id_of_first_art_appearance_for_language.is_none() {
                errors.push(format!(
                    "id_of_first_art_appearance_for_language must be set if is_first_art_appearance_for_language is false for card {}",
                    card_id
                ));
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_file_location = "./data/english/cards/fossil.json";
    let json_data = std::fs::read_to_string(json_file_location)?;
    // Every field is optional to avoid crashing serde if fields don't exist
    let cards: Vec<Card> = serde_json::from_str(&json_data)?;

    match validate_cards(&cards) {
        Ok(_) => println!("All cards in `{}` are valid!", json_file_location),
        Err(errs) => {
            println!("Validation errors:");
            for e in errs {
                println!("- {}", e);
            }
        }
    }

    Ok(())
}
