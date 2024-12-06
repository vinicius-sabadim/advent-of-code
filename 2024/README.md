# Advent of code 2024

Last year I started to do the challenge in Rust, which was not a good decision. As I didn't know anything about Rust, it was hard to embrace the challenge of a new language and the increasing complexity of the advent of code. This year I chose to go with the language I have more experience, so Javascript it is.

## Learned lessons

### Day 1

It wasn't hard. For the part 1, my first solution didn't consider that values could be negative when doing list2 - list1, so then I used the Math.abs(), which solved the problem. For the part 2, my solution worked in the first time, which makes me happy.

### Day 2

In the part 1 the only problem that I had with my first attempt was to not converting the input to number, as they were read was string, which lead to wrong math. The part 2 was interesting, because I could refactor a lot my solution in the part 1, making the reusable function `isReportSafe`.

### Day 3

I don't remember the last time I wrote a regex in Javascript, that felt nice.
The part 1 was easy to do, the part 2 took me time to reach the proper solution, I was making mistakes about the begin and end to be used in the `.substring()`. I was explaining the problem to my wife when I realised the mistake I've made with the boundaries :)
I can definitely improve the code to make usage of functional concepts, but my goal is not to learn Javascript, as I'm using it since 2012.
