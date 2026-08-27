fn main() {
    let mut a_list = vec![23, 3, 4, 10, 45, 67, 34, 90];
    a_list.sort();

    let median = {
        if a_list.len() % 2 == 0 {
            let item_a = a_list[(a_list.len() / 2) -1];
            let item_b = a_list[((a_list.len() / 2) + 1) - 1];

            let even = (item_a + item_b) / 2;
            even
        } else {
            let odd = (a_list.len() + 1) / 2 - 1;
            a_list[odd]
        }
    };
    println!("{:#?}", median);
}
