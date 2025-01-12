struct Laptop {
    brand: String,
    cost_per_unit: u64,
    quantity_available: u64,
}

impl Laptop {
    //open chat|find proplem|find solution 
    fn total_cost(&self, quantity_to_purchase: u64) -> u64 {
        self.cost_per_unit * quantity_to_purchase
    }
 fn main() {
    // Define the laptops for each company
    let hp_laptop = Laptop {
        brand: String::from("HP"),
        cost_per_unit: 650_000,
        quantity_available: 10,
    };

    let ibm_laptop = Laptop {
        brand: String::from("IBM"),
        cost_per_unit: 755_000,
        quantity_available: 6,
    };

    let toshiba_laptop = Laptop {
        brand: String::from("Toshiba"),
        cost_per_unit: 550_000,
        quantity_available: 10,
    };

    let dell_laptop = Laptop {
        brand: String::from("Dell"),
        cost_per_unit: 850_000,
        quantity_available: 4,
    };
    // Number of laptops the customer purchases from each brand
    let quantity_to_purchase = 3;

    // Calculate total cost for each brand
    let total_cost_hp = hp_laptop.calculate_total_cost(quantity_to_purchase);
    let total_cost_ibm = ibm_laptop.calculate_total_cost(quantity_to_purchase);
    let total_cost_toshiba = toshiba_laptop.calculate_total_cost(quantity_to_purchase);
    let total_cost_dell = dell_laptop.calculate_total_cost(quantity_to_purchase);

    // Calculate the total cost for all laptops purchased
    let total_cost = total_cost_hp + total_cost_ibm + total_cost_toshiba + total_cost_dell;

    // Output the total cost
    println!("Overall cost for 3 laptops from each brand: {}", total_cost);
}
