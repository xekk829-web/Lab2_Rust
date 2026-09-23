use std::println;

fn main() {
    let product_name = "Earphones";
    let price = 9000.0;
    let in_stock = true;
    let category = "Electronics";

    println!("{} | {} rub. | {} | in stock: {}", product_name, price, category, in_stock);

    let stock_before = 10;

    let stock_after = {
        let mut stock = stock_before;
        stock -= 3;
        stock -= 1;
        stock
    };

    println!("Before: {} | After: {}", stock_before, stock_after);

    let init_price = 1500;

    let final_price = {
        let mut price2 = init_price;
        price2 -= 500;
        price2 -= 300;
        price2
    };

    println!("Initial price: {} | Final price: {}", init_price, final_price);

    let price1 = 1000;

    {
        let price1 = price1 + 500;
        println!("Inside block: {}", price1);

    }

    println!("Outside block: {}", price1);

    let price3 = "1590.0";

    let final_price2 = {
        let mut price3: f32 = price3.parse().unwrap();
        price3 *= 1.15;
        price3
    };

    println!("FINAL PRICE: {}", final_price2);


}
