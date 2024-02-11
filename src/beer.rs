const NO_MORE: &str = "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n";

const ONE_MORE: &str = "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n";

const TWO_MORE: &str = "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n";

pub fn verse(n: u32) -> String {
    match n {
        0 => String::from(NO_MORE),
        1 => String::from(ONE_MORE),
        2 => String::from(TWO_MORE),
        num => format!(
            "{num} bottles of beer on the wall, {num} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n",
            num - 1
        )
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = String::new();
    let mut beers = start;

    while beers >= end {
        let v = verse(beers);
        result.push_str(v.as_str());

        if beers == 0 {
            break;
        } else {
            beers -= 1;
        }

        if beers >= end {
            result.push_str("\n");
        }
    }

    result
}
