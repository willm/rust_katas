use std::collections::HashMap;

fn main() {
    checkout("AA");
}

struct Offer {
    amount: i32,
    price: i32,
}

struct Item {
    price: i32,
    offers: Vec<Offer>
}

fn parse_basket(items: &str) -> HashMap<char, i32> {
    let mut basket = HashMap::new();
    for i in items.chars() {
        match basket.get(&i).cloned() {
            Some(amount) => {basket.insert(i, amount + 1)},
            None => {basket.insert(i, 1)}
        };
    }
    basket
}

fn checkout(items: &str) -> i32 {
    let mut catalogue = HashMap::new();
    catalogue.insert('A',Item {price: 50, offers: vec![Offer {amount: 3, price: 130}]});
    catalogue.insert('B',Item {price: 30, offers: vec![Offer {amount: 2, price: 45}]});
    catalogue.insert('C',Item {price: 20, offers: vec![]});
    catalogue.insert('D',Item {price: 15, offers: vec![]});

    let mut total = 0;
    let basket =  parse_basket(items);

    for code in basket.keys() {
        let number_of_times_in_basket = basket.get(code).unwrap();
        let item = catalogue.get(code).unwrap();

        if item.offers.len() == 0 {
            total = total + (number_of_times_in_basket * item.price)
        }
        for offer in item.offers.iter() {
            let offer_applications = number_of_times_in_basket / offer.amount as i32;
            total = total +
                (offer_applications * offer.price) +
                (number_of_times_in_basket % offer.amount * item.price);
        }
    }
    total
}

#[test]
fn buying_nothing_returns_zero() {
    let total = checkout("");
    assert_eq!(total, 0);
}

#[test]
fn buying_a_single_a() {
    let total = checkout("A");
    assert_eq!(total, 50);
}

#[test]
fn buying_multiple_items() {
    let total = checkout("AA");
    assert_eq!(total, 100);
}

#[test]
fn buying_special_offers() {
    let total = checkout("AAA");
    assert_eq!(total, 130);
}

#[test]
fn buying_special_offers_b() {
    let total = checkout("BB");
    assert_eq!(total, 45);
}

#[test]
fn buying_different_items() {
    let total = checkout("ABCD");
    assert_eq!(total, 115);
}
