pub fn verse(n: u32) -> String {
    let v1;
    let v2;
    let populate = |n: u32| -> String {
        let p1;
        let p2;
        match n - 1 {
            n if n == 0 => {
                p1 = "Take it down and pass it around, ".to_string();
                p2 = "no more bottles of beer on the wall.".to_string();
            }
            n if n == 1 => {
                p1 = "Take one down and pass it around, ".to_string();
                p2 = "1 bottle of beer on the wall.".to_string();
            }
            n if n > 1 => {
                p1 = "Take one down and pass it around, ".to_string();
                p2 = format!("{n} bottles of beer on the wall.");
            }
            _ => panic!("")
        };
        format!("{p1}{p2}")
    };
    match n {
        n if n == 0 => { 
            v1 = "No more bottles of beer on the wall, no more bottles of beer.".to_string();
            v2 = "Go to the store and buy some more, 99 bottles of beer on the wall.".to_string();
        },
        n if n == 1 => {
            v1 = "1 bottle of beer on the wall, 1 bottle of beer.".to_string();
            v2 = populate(n);
        }
        n => {
            v1 = format!("{n} bottles of beer on the wall, {n} bottles of beer.");
            v2 = populate(n);
        }
    }
    return format!("{v1}\n{v2}\n")
}

// First sentence is about the number of beers.

pub fn sing(start: u32, end: u32) -> String {
    let mut res = String::from("");
    let mut current = start;
    while current >= end {
        let s = verse(current);
        res += s.as_str();
        if current == end {
            break
        }
        current -= 1;
        res += "\n";
    }
    res
}
