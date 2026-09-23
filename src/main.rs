use std::{format, println};

fn main() {
    // Step 1. Creating variables and getting your first look at scope
    // let product_name = "Earphones";
    // let price = 9000.0;
    // let in_stock = true;
    // let category = "Electronics";

    // println!("{} | {} rub. | {} | in stock: {}", product_name, price, category, in_stock);

    // let stock_before = 10;

    // Step 2. Mutability and immutability within the block
    // let stock_after = {
    //     let mut stock = stock_before;
    //     stock -= 3;
    //     stock -= 1;
    //     stock
    // };

    // println!("Before: {} | After: {}", stock_before, stock_after);

    // let init_price = 1500;

    // let final_price = {
    //     let mut price2 = init_price;
    //     price2 -= 500;
    //     price2 -= 300;
    //     price2
    // };

    // println!("Initial price: {} | Final price: {}", init_price, final_price);

    // Step 3. Shadowing and its connection to the field of view
    // let price1 = 1000;

    // {
    //     let price1 = price1 + 500;
    //     println!("Inside block: {}", price1);

    // }

    // println!("Outside block: {}", price1);

    // let price3 = "1590.0";

    // let final_price2 = {
    //     let mut price3: f32 = price3.parse().unwrap();
    //     price3 *= 1.15;
    //     price3
    // };

    // println!("FINAL PRICE: {}", final_price2);

    // Step 4. Scope: statement and expression
    // let base_price = 4990.0;

    // let order_total ={
    //     let item_1 = base_price;

    //     let item_2: f64 = {
    //         let discount = 15.0;
    //         let multiplier = 1.0 - (discount/100.0);
    //         base_price * multiplier
    //     };

    //     item_1 + item_2
    // };

    // println!("The final order amount: {}", order_total);

    // Step 5. if/else as an expression within a scope
    let order_total = 2500;
    let status = if order_total >= 3000 {
        let isFree = true;
        format!("Доставка бесплатная (статус: {})", isFree)
    } else {
        let isFree = false;
        format!("Доставка платная (статус: {})", isFree)
    };

    println!("{}", status);
}
