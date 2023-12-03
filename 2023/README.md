# Advent of code 2023

As a frontend developer, I have opted for [Rust](https://www.rust-lang.org/) as the programming language for the exercises.

My aim is not to become an expert in the language but rather to explore its capabilities. Additionally, I will strive to adhere to functional programming paradigms rather than opting for imperative development.

## Learned lessons

### Day 1

In general I found the challenge hard for the first day, I struggled for quiet some time to make the regex work properly in the part 2.

- [How to open](https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html) a `.txt` file and read its content.
    - I was a bit confused with the returned type `io::Result<io::Lines<io::BufReader<File>>>`.
- Great to see a [`.filter_map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter_map), as it does both as the same time, different than JavaScript.
- Interest to see how regex work in Rust. I liked the `.find_iter`.
    - I was confused with the matching for the cases like `oneight`.
- The `.reduce` method is called [`.fold`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold) in Rust.

### Day 2

It was easier than day 1, mainly because I could reuse some of the logic from the previous day.

- The [`.enumerate`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.enumerate) is used to create an iterator with the tuple `(index, value)`.
- I liked the [`.split_whitespace`](https://doc.rust-lang.org/std/primitive.str.html#method.split_whitespace) to split a string in the whitespace char.
- The `match` made the code readable.
- I liked the [`.sum`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum).

### Day 3

It looked as a medium problem to solve, but I spent too much time struggling with indexes out of bound.

- I need to learn the differences between [`.iter`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter) and [`.into_iter`](https://doc.rust-lang.org/std/primitive.slice.html#method.into_iter).
- Loved the [`.map_or`](https://doc.rust-lang.org/std/option/enum.Option.html#method.map_or).
- No ternary operator? Crazy! Although, the usage of `if/else` in the same line makes sense.
- I have a good knowledge of [`.flat_map`](https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.flat_map) but I need to study more to know exactly when to use it.
- As the `.sum`, it's handy to have a [`.product`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.product).
- As `line: Option<&String>`, I got the lenght of the line as `line.as_ref().unwrap().len()`, but I wonder if there are other ways to do it.