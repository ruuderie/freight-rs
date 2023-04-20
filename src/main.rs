use std::io;

fn main() {
    let mut origin_city_state = String::new();
    let mut pickup_date = String::new();
    let mut pickup_time = String::new();
    let mut destination_city_state = String::new();
    let mut delivery_date = String::new();
    let mut delivery_time = String::new();
    let mut commodity_description = String::new();
    let mut weight = String::new();
    let mut equipment_type = String::new();
    let mut total_rate = String::new();

    println!("Please enter the origin city and state:");
    io::stdin()
        .read_line(&mut origin_city_state)
        .expect("Failed to read input");

    println!("Please enter the pick-up date:");
    io::stdin().read_line(&mut pickup_date).expect("Failed to read input");

    println!("Please enter the pick-up time window:");
    io::stdin().read_line(&mut pickup_time).expect("Failed to read input");

    println!("Please enter the destination city and state:");
    io::stdin()
        .read_line(&mut destination_city_state)
        .expect("Failed to read input");

    println!("Please enter the delivery date:");
    io::stdin().read_line(&mut delivery_date).expect("Failed to read input");

    println!("Please enter the delivery time window:");
    io::stdin().read_line(&mut delivery_time).expect("Failed to read input");

    println!("Please enter the commodity description:");
    io::stdin()
        .read_line(&mut commodity_description)
        .expect("Failed to read input");

    println!("Please enter the weight (in lbs or kgs):");
    io::stdin().read_line(&mut weight).expect("Failed to read input");

    println!("Please enter the equipment type:");
    io::stdin()
        .read_line(&mut equipment_type)
        .expect("Failed to read input");

    println!("Please enter the total rate:");
    io::stdin().read_line(&mut total_rate).expect("Failed to read input");

    println!("\nGenerated Template:");
    println!("📍 Location: {}", origin_city_state.trim());
    println!("📅 Date: {}", pickup_date.trim());
    println!("⏰ Time: {}", pickup_time.trim());
    println!("\nDelivery:");
    println!("📍 Location: {}", destination_city_state.trim());
    println!("📅 Date: {}", delivery_date.trim());
    println!("⏰ Time: {}", delivery_time.trim());
    println!("\nFreight Details:");
    println!("📦 Commodity: {}", commodity_description.trim());
    println!("🔢 Weight: {}", weight.trim());
    println!("🚚 Equipment: {}", equipment_type.trim());
    println!("💵 Rate: {}", total_rate.trim());
}
