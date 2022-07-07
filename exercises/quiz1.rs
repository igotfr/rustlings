// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the quantity bought. No hints this time!

// ✓ I AM NOT DONE

// Put your function here!
// fn calculate_apple_price {
fn calculate_apple_price(apples_qty: u32) -> u32 {
    /*let mut total_price: u32;

    if apples_qty <= 40 {
        total_price = 2 * apples_qty;
    } else {
        total_price = apples_qty;
    }

    total_price*/

    /*let total_price: u32 = if apples_qty <= 40 {
        2 * apples_qty
    } else {
        apples_qty
    };

    total_price*/

    /*if apples_qty <= 40 {
        2 * apples_qty
    } else {
        apples_qty
    }*/

    (apples_qty <= 40) as u32 * (2 * apples_qty) + (apples_qty > 40) as u32 * apples_qty
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(40);
    let price3 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(65, price3);
}

// $ rustlings hint quiz1
// No hints this time ;)