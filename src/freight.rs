use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

fn generate_templates(
    cities: &[&str],
    equipment_commodities: &HashMap<&str, Vec<&str>>,
) -> Vec<String> {
    let mut templates = Vec::new();

    for city in cities {
        for (equipment, commodities) in equipment_commodities {
            for commodity in commodities {
                let template = format!(
                    "📍 Location: {}\n🚚 Equipment: {}\n📦 Commodity: {}",
                    city, equipment, commodity
                );
                templates.push(template);
            }
        }
    }

    templates
}

fn random_template(
    cities: &[&str],
    equipment_commodities: &HashMap<&str, Vec<&str>>,
) -> String {
    let mut rng = rand::thread_rng();
    let random_city = cities.choose(&mut rng).unwrap();
    let random_equipment = equipment_commodities.keys().collect::<Vec<_>>().choose(&mut rng).unwrap();
    let random_commodity = equipment_commodities
        .get(*random_equipment)
        .unwrap()
        .choose(&mut rng)
        .unwrap();

    format!(
        "📍 Location: {}\n🚚 Equipment: {}\n📦 Commodity: {}",
        random_city, random_equipment, random_commodity
    )
}

fn main() {
    let cities = vec!["New York, NY", "Los Angeles, CA", "Chicago, IL"];
    let equipment_commodities: HashMap<&str, Vec<&str>> = [
        (
            "Reefer",
            vec![
                "Fresh produce",
                "Frozen foods",
                "Dairy products",
                "Pharmaceuticals",
            ],
        ),
        (
            "Dry Van",
            vec!["Electronics", "Clothing", "Non-perishable food", "Paper products"],
        ),
        (
            "Flatbed",
            vec![
                "Building materials",
                "Lumber",
                "Steel",
                "Heavy machinery",
                "Construction equipment",
            ],
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let templates = generate_templates(&cities, &equipment_commodities);
    println!("Generated Templates:");
    for template in &templates {
        println!("{}", template);
        println!("---");
    }

    println!("Randomly Generated Template:");
    let random_template = random_template(&cities, &equipment_commodities);
    println!("{}", random_template);
}
