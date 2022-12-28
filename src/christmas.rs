pub fn twelve_days_of_christmas() {
    let lines = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five gold rings (five golden rings)",
        "Four calling birds",
        "Three French hens",
        "Two turtledoves",
        "And a partridge in a pear tree",
    ];

    let mut counter = 1;

    while counter <= 12 {
        println!("On the {} day of Christmas, my true love sent to me", counter);
        let lines = &lines[12-counter..12];
        for val in lines.iter() {
            println!("{}", val);
        }
        println!();
        counter += 1;
    }
}
