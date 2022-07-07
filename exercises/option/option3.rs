// option3.rs
// Make me compile! Execute `rustlings hint option3` for hints

// ✓ I AM NOT DONE

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        //Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}

// $ rustlings hint option3
/*
The compiler says a partial move happened in the `match`
statement. How can this be avoided? The compiler shows the correction
needed. After making the correction as suggested by the compiler, do
read: https://doc.rust-lang.org/std/keyword.ref.html
*/