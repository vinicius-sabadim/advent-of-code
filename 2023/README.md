# Advent of code 2023

As a frontend developer, I'm not used to languages that are not JavaScript/TypeScript, so I have opted for [Rust](https://www.rust-lang.org/) as the programming language for the exercises.

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

### Day 4

I tried to solve the part 2 without using a vector, but going with a HashMap, made thinks confusing to me.

- Study more about [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) and learn about its methods.
- How the get the parts in a [`.splitn`](https://doc.rust-lang.org/std/primitive.slice.html#method.splitn) without calling `.next`?
- Learn better ways to use the `.map` without the need of `else { 0 }`.

### Day 5

In the first part, I started with a top-down approach, going from the seed to the location.

In the part 2, if I followed the same approach, I'd potentially go through billions of entries in the seeds array.
Then, I created new functions to use the destination to get the source. Doing that, I was able to start from the location
and go upwards until I reach the necessary seed. After that, it was a matter of checking if the seed was provided or not.

- I used some brute-force for the part 2 adapting the part 1, but then I realised how much time my code was taking to run.
- I didn't know about the [`loop`](https://doc.rust-lang.org/std/keyword.loop.html).
- I finally learned how to do some tests in Rust :)
- Is there a better way to structure the tests?
- I didn't spend time to improve the functions in the utils module. I believe I can improve them.