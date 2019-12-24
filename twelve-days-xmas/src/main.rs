fn main() {
    let days_presents: [(&str, &str); 12] = [
        ("first", "a partridge in a pear tree"),
        ("second", "two turtle doves"),
        ("third", "three french hens"),
        ("fourth", "four calling birds"),
        ("fiveth", "five gold rings"),
        ("sixth", "six geese a-laying"),
        ("seventh", "seven swans a-swimming"),
        ("eighth", "eight maids a-milking"),
        ("ninth", "nine ladies dancing"),
        ("tenth", "ten lords a-leaping"),
        ("eleventh", "eleven pipers piping"),
        ("twelfth", "twelve drummers drumming"),
    ];

    for day in 0..12 {
        println!(
            "On the {} day of Christmas my true love sent to me:",
            days_presents[day].0
        );

        for present in (0..day + 1).rev() {
            if present == 0 && day + 1 != 1 {
                println!("And {}", days_presents[present].1);
            } else {
                println!("{}", days_presents[present].1);
            }
        }

        println!("--");
    }
}
