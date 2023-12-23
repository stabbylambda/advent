# Daily Writeups

I'm gonna try something new this year. I've been writing these up in a Slack I'm in, but I thought it might be nice to keep this around for posterity. I started on Day 3, so nothing much to note on Days 1 and 2.

## Day 3

I made a misstep at the beginning when I made the decision to parse whole numbers instead of single digits. I wound up with a `Vec<Vec<>>` that wasn’t square which is A Problem ™️.

Once I backtracked and decided to just parse the square and then start scanning for numbers, things got better. At that point it’s some simple caching to see when you’ve already visited a square, iterating right/east to gather the rest of the digits, and checking the adjacent squares for each digit along the way.

I did move more of that processing logic into the parsing function once I hit part 2 because I didn’t want to call it twice once I realized I was just going to need to find the gears with two part numbers next to them. Once the data structure is set up correctly, both problem1 and problem2 turn into no big deal.

## Day 4

That was pretty straightforward. The parsing is a little wonky because of the columnar justification, but other than that, there wasn’t really anything tricky.

I think the theme this year is actually: even numbers are easy, odd numbers punch you in the dick.

## Day 5

I cheated a bit with the parsing by not giving a single shit about the headers. I noticed that in my test input and the real input, all of the almanac maps (e.g. seed-to-soil) are in order, so I just grabbed them all as a list. Then it’s just a matter of creating a `translate` function on the individual almanac range, the almanac map, and then the almanac itself turns into a `fold` over that list. Part 1 is straightforward.

I’m sure part 2 could have been done in a very fancy way with math and stuff. But I decided to just cheat my way to victory with rayon and parallelize the generation of all the seeds in the ranges and the translation of them to the result. My input contains 2.5 billion seed values…and completes in 20 seconds on my M1 Pro MBP when I compiled in `--release`  mode.

I will say that when I saw the wall of text in the problem I feared this was going to continue the trend of odd days being a real pain in the ass. But this one was actually pretty fun.

### Later...

The maps are essentially piecewise functions. I think there’s a way to do it where you basically perform function composition and get down to a linear time piecewise range function. And that's what I eventually did

## Day 6

Basic stuff. Parse the two lines, zip the vecs together to get a `Vec<Race>` and then “simulate” the races by doing some really really really basic math to see how long you have to hold the button to win.

Problem 2 was not that different from day 1. Just needed to mangle the string before going into the parser to remove all the spaces. The rest of the code functions as-is.

## Day 7

This shows my “I’ll make a struct! I’ll make an enum!” bias real hard. Parse everything into `Hands` which maintains a `Vec<(Hand, u32)>` for the bids. `Hand` is a `Vec<Card>` and a method to get the `HandType`. And then the matter of “winning” was just to define a total ordering on the `HandType` and the `Card` using the `Ord` trait. From there, it’s just standard comparison using iterator sorting.

For part 2, I took the route of adding a boolean to the parse function which changed how J was interpreted and added `Joker` to the top of the `Card` enum so it’s lowest. Then I changed `get_type` to take into account all the ways that jokers could change the hand to a better hand.

I’d like to thank the `itertools` package for providing `counts()` which made getting hand patterns much much easier.

## Day 8

### Part 1

First I created a `Direction` enum for `Left` and `Right`, As part of the parsing, I took the list of the rooms and split them from `AAA = (BBB, CCC)` into two items: `(AAA, Left) => BBB` and `(AAA, Right) => CCC` so that I could easily put the entire map in a hashmap and look up where to go based on both the current room and the next direction. Then it’s just a matter of incrementing a count and terminating once the current is ZZZ.
I think this another example of just setting up your data structures to make the solution of the problem easy.

### Part 2

As soon as I saw that I was going to need to spin up a bunch of ghosts and have them all find their way to the end, my first thought was “time to install rayon again” but I solved it naively at first just to see if it would complete in a normal amount of time. Basically for each direction in the input, have each ghost take one step and see if they’re all at the end. Spoiler alert, it didn’t complete. But I did notice from some debugging that the cycle times were stable. Once a ghost finds its way to the end, it’s going to keep finding its way to the end in that cycle.

So instead of “ah install rayon” it was “ah, cycle counts, so least common multiple”. So I inverted the loop. This time it was for each ghost, have them run until they hit an ending room, then return that as their cycle time. Then just `lcm` that list.

## Day 9 

### Part 1

I like the duality of this problem. In FP terms, it’s an `unfold` and then a `fold` again. Given the starting row, I generated a sequence of rows, eventually getting to the row with all zeros. Then once you have that list of rows, you just fold it back up by adding the last delta to the last item from the next row up. The accumulator keeps track of the delta, which means you don’t have to do a bunch of crappy row manipulation on your way back up the list.

### Part 2

I was really glad I kept all the numbers in all the rows around since the next part was to do the same thing to the first part of the row. The only difference in part 2 was the accumulator function needing to do a subtraction instead of addition.

## Day 10

### Part 1

Parsing was easy. Standard AoC grid type stuff. And then all the rules about how you can move got kinda gross. At first I thought I might be able to reuse some of my code from AoC 2017 Day 19 but that was not the case. I then overcomplicated things with a priority queue before scaling back and realizing that because S can only connect to two tiles, I just had to pick one and start traversing. Once you find the starting position, return the path length divided by 2. Pretty straightforward.

### Part 2

For part 2, I quickly realized I’d need not just the path length, but all the points, so quickly went back and reworked the code to track all the points I’d traversed. Then I started thinking about right-hand-ruling the pipe to find all the . characters on the inside until I thought WWCD (obviously What Would Chris Do) and summoned the most mathy part of my math brain and googled “area inside polygon integer coordinates” which led to [Pick’s theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem) and the [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula).

Some number of minutes later (it’s late now, so more than I’d like to admit) and one wild stab at placing an `abs()` where I needed it and I have a working solution.

[Update after thinking about it the next day] I think maybe I had the abs in there because the shoelace formula requires a counter-clockwise path and maybe my path was clockwise?

## Day 11

### Part 1

Parsing a grid is such a common task in aoc. After I had the `Vec<Vec<Tile>>` I wrote an `expand_universe` function that goes through each row and doubles it if it’s empty and does the same for columns. Then it’s just a matter of finding all the galaxy coordinates, getting the permutations of them and finding the [Manhattan Distance](https://en.wikipedia.org/wiki/Taxicab_geometry) (another common aoc thing) between them.

One tricky consideration here:
Permutation yields both `(a, b)` and `(b,a)`. Rather than sort/unique them, I just divided by 2.

### Part 2

Uh. Yeah. That’s just not gonna work. I’m not putting a million rows in for each of the empty rows. Back to the drawing board to the other thing I considered in part 1: using the base manhattan function and adding “extra costs” for traversal of empty rows and columns. I re-wrote this to leave the grid as-is for the manhattan function, instead keeping a vector of weights for the rows and column. Any row/col with a galaxy gets a weight of 0, any without a galaxy gets a weight of `expansion_amount - 1`. Then the modified manhattan function sums any extra row costs between `min_y` and `max_y` and any extra column costs between `min_x` and `max_x`.

This was pretty fun. I had a suspicion that my “oh just insert another row/col” code wasn’t going to fly in the second part. Just for fun I went back and did it the naive way…confirmed: did not work. Took forever just to manipulate the vectors. Not a good idea.

## Day 12

### Part 1

I hate dynamic programming problems. I spent a good 20 minutes going “how can I make this not a dynamic programming problem” before giving in. I really wanted to just generate possible solutions for each one and check validity as I went. But that was going to be very very very expensive and I knew DP was The Way. :man-shrugging:

I don’t even have much to say here. I knew about python’s `functools.lru_cache` so I found something similar for rust. I didn’t want to maintain a mutable cache through a bunch of recursion because I’ve done that before and it sucks. Macros to the rescue.

The actual DP problem is just…solve a small problem. Add new solutions to the front of it. Cache all steps. The logic is messy and I don’t like it. I’m sure there’s something better I could have done. Lots of print debugging along the way to validate that I was doing the right thing.

At least it’s fast?

### Part 2

If you didn’t need dynamic programming for part 1, you sure as hell need it for part 2. Multiply everything times 5. Yay. I had to bump from u32 to u64 for this.

My initial cut took 7s to run. I theorized this was because I was calling `remove(0)` on Vec to cut out the first item as I was recursing. Remove is, of course, O(n). So I swapped that out for a pair of `VecDeque` where `pop_front` is O(1). The total runtime is now 2s for both problems.

I know I can make that even faster if I stop cloning all over the place and make it use slices, but then I’m introducing a lifetime to the function and maybe that’s not even going to yield awesome results.

### Later

I managed to get this to a place I’m happy-ish with. The conditions are much cleaner now and I think it looks a lot nicer with the pattern matching cleaned up. Still not wild about the problem, but at least the solution looks nice.

## Day 13

### Part 1

Not that hard. I parse into row-major nested `Vec<Vec<char>>` . Then find all of the pairwise equal rows so you know where to start. Then move out comparing all of the equidistant rows until you run off the edge of the grid. That’s obviously all horizontally focused, so to do vertical reflections…transpose to the rescue! It’s just `find_reflection(tranpose(v))` basically.

### Part 2

Wrote some code to generate all smudges and then went back to swap out `char` for `bool` because that made swapping easier. Then wrote the code to find a different reflection in each grid, which was pretty straightforward. The real problem I ran into is that the very first bit of code I wrote for part 1 was a `find_map` instead of a `flat_map` so I wasn’t always searching the entire problem space. I also preferred horizontal over vertical because of the Option chaining. Lots of print debugging later: figured that I needed to grab all reflections and compare them against the original.

## Day 14

### Part 1

Parse a grid. Of course. Actually this one was fun. I did some stupid column-wise stuff in part 1 that I regretted in part 2. My favorite part of part 1 was realizing that I could take a single row/column, split it at each `#` and use `Ord`  to sort the slice in place, yielding something like `…OOO#` so that it simulated the rocks rolling as far as they could, then rejoin the array together.

### Part 2

I undid the stupid columnar stuff and grabbed some rotate90 code I had sitting around from 2020 Day 20 so that instead of “north” being column-wise and reversed (north-most was index 0 in each column), it was row-wise and normal (end of the row was north-most). That made the math for `load()`  way simpler and gathering the data for `tilt()` simpler as well. Then `spin_cycle()` is just 4 `tilt` and `rotate` calls.

Of course doing 1000000000 cycles of 4 tilts is ridiculous. Enter my cycle detection code from 2022 Day 17. Basically insert the board as the hashmap key with the current cycle index as the value. If there’s a collision, we’ve seen that particular board before, which means we hit a cycle. Now do some math to figure out how many cycles we should skip (and clear the hashmap) so that we can run the rest of the count to 1000000000.

This was fun!

## Day 15

### Part 1

That was kind of a welcome break. I resisted the urge to do something fancy for the first part and create a struct. The
instruction just needs to be hashed. Because add and mul both work in the group of integers mod _n_, it's easy to do the hash without overflowing u8.  

### Part 2

Struct time. I went back to the `nom` parser and actually parsed out the `-` and `=` which did require me to go back and rework the hash function. Next step was putting together the instructions, which was pretty straightforward. I used a `BTreeMap` so I could see the keys in order while debugging. Nothing really that fancy in this.

## Day 16

### Part 1

Another pretty easy one. As soon as I saw the splitting, I knew it was going to be a `VecDeque` solution where we push and pop. I initially let the autocomplete scaffold out all of the match conditions and then manually collapsed them when I figured out what the outcomes were. I initially hit an infinite loop because I assumed the beam would always end up off the edge and I'd get a None for the neighbor. But that's not true. So I updated the `visited` set to take into account the `Direction` because then we won't retrace steps.

### Part 2

Just extract part 1's function and then generate a new beam for each edge tile and then find the max. I don't know if there's much more to say about this one. 

## Day 17

### Part 1

Hello Dijkstra my old friend. So I have a pretty standard dijkstra available in my common implementation, but it really expects an adjacency list. And I didn’t really see an easy way to pre-compute the adjacency list, so I just copy/pasted the code and mangled the pieces necessary to use the whole state (including the direction and consecutive steps) as the lookup for previous costs. It’s slow though and I thought moving to `BTreeMap` was going to fix it, but that’s not looking like it’s true. There’s something wrong with my implementation and I’ll figure it out Later ™️.

### Part 2

Oh something’s super wrong with the implementation. This took like 10 minutes to compute on my macbook. I just had to add the min and max to the `get_eligible_directions` function and pass 1 and 3 for part 1 and 4 and 10 for part 2. I’ll take a look tomorrow at why this is so freaking slow. My guess is too much time spent popping and inserting into the map, but those should be fast.

### Later...

hahahahaha I was putting the entire `State` object as the cache key. That includes the `heat_loss` field which is also the value. So of course the `BTreeMap` was absolutely exploding, which made the entire process grind to a crawl. Defining the cache key as `((usize, usize), Direction, u32)` (i.e the entire state object minus the `heat_loss` field) makes it complete both problems in half a second. I also added a small optimization to only consider `DirectionType` instead of `Direction` because you really only care about horizontal and vertical movement when thinking about moving next, not individual directions.

## Day 18

### Part 1

Neat. Grab code from day 10. Change it a bit to just get the vertices and track the length. Use shoelace + pick's theorem. Done.

### Part 2

Okay, same thing, but instead, just decode some hex values first. And that's a big number.

## Day 19

### Part 1
These parsing problems are why I always use a parser combinator library. I would not want to do this with regex or a bunch of `split` calls.  Once you get the stuff into the right data structure, it’s kind of straightforward.  I modeled everything as `Rule` (an enum with `Branch` or `Fallthrough`) and then `Workflows` was a `BTreeMap<str, Vec<Rule>>`. Then `evaluate`  just turns into “keep going until you hit `A` or `R`”. Implementing that was easier than parsing the input.

### Part 2

I stumbled through this one for longer than I’d have liked. It wasn’t that dissimilar from the range splitting stuff in day 5, but I was having a harder time reconciling that with the map. The “aha” came when I rewrote my evaluate to `evaluate_part` and made it recursive.

I created a `RangedPart` which is basically `Part` but with ranges instead of scalars as values. Then I implemented an `evaluate_ranged_part` which does a DFS of the rules and, at each `Rule`, splits the ranges around the value in the rule, creating two `RangedParts`: one with the accepted values, one with the rejected values. Then we just keep whittling down the rejected part by splitting it into branches that are accepted. Finally at the end, what we’re left with is just the `Fallthrough` case, which will either be `A` or `R`. `A` gets the count of the `RangedPart` which is the product of the counts of all the ranges.

Fun!

## Day 20

### Part 1

I took a domain modeling approach to this one. Started off by creating all the different types of modules because I wanted to just write it, write a test for it, make sure it worked independently, and move on. This actually worked pretty well for me. The tricky bit is that I needed the list of incoming nodes for each of the conjunctions in order to know when to swap it. So I had to parse everything, then *invert* the map to find the incoming nodes for each conjunction, then create the actual map. I don't love the double parse, but I couldn't think of a better way to do it.

After that, it's just a matter of keeping pulses in a `VecDeque` and queueing and dequeueing them. Every time you see a high pulse, increment a counter. Same with low pulses. And multiply them together.

### Part 2

I wrote it naively and let it run, knowing it wasn't going to be the answer. Then I popped open the input and saw that `rx` only has one incoming node which is a `Conjunction` node. That node has four nodes coming into it. First thing I did was hardcode to my input. I wrote some code to print out which button press each of them swaps to a High pulse and noticed that the periods were stable. Shove them all in a cache, once you have all four, least common multiple them and there's the answer.

I went back to un-hardcode my solution and wound up doing a double invert of the map. Find the node coming into `rx`, get the count of nodes coming into that node. That wasn't that hard, but I definitely did it after getting the answer.

## Day 21

### Part 1

Super simple BFS. Literally nothing to add here.

### Part 2

This was hard. I went to inspect the input and noticed the diamond thanks to the VS Code preview pane. I figured it was going to be some sort of cycle detection code and I knew it was going to be a formula that grew non-linearly because you can see that in the text in part 2. But I had to check the subreddit to see what I was missing. The real trick is that that there are no walls in the row or column of the starting position (which is not easy to see in the preview pane). That means that eventually given enough steps, you'll sort yourself into a perfect quadratic equation, which you can eventually use to just find the right answer without simulation.

So:

 1. Simulate the steps, keeping track of where we have `            last % input.width == step_goal % input.width`.
 2. Once we have three of those, we can create the quadratic polynomial
 3. Have the computer do simple math on the input

I'm not sure I ever would have gotten this one without looking at a few hints. 

## Day 22

### Part 1

I waffled back and forth for a bit on best way to solve this. I was going to get fancy with a heightmap, but decided ultimately I would just model the entire 3d tower because the input wasn’t that large. First step was getting everything in the right order so I remapped all the triples from x-major to z-major so that they would sort correctly. Then, starting with the lowest block, drop it in the `settled` map, keeping track of each cube in the brick separately and assigning it the brick id. If it hits the ground, it just settles. If any cube in this brick hits any cube in another brick, we keep track of that in the `holding_up` and `sitting_on` maps (simulating a graph, I’m sure I “should” have pulled in petgraph for this). 

After that, it’s just a matter of iterating over the `holding_up` (bottom) bricks and seeing which ones above them have more than 1 brick in the `sitting_on` (top) map.

### Part 2

For this, I pulled out all the code that settles the tower and builds the graph into the `parse` function because it’s needed for part 2 as well as part 1. The other thing I changed was changing the `cubes` field in brick from a `Vec<Cube>` to a `BTreeSet<Cube>` because I was going to need set intersection.

For each brick in `holding_up` we disintegrate it by adding it to `unsupported` then we check to see if the bricks are only sitting on bricks that are in `unsupported` as well. If they are, then we add them to the queue and keep going until we have no more bricks to support. Then just count the `len` of `unsupported` minus the one that we pulled out and sum those all up.

## Day 23

### Part 1

This is a pretty straightforward DFS with some fun little conditions forcing you to go down the icy slopes. The logic got kinda messy here when checking for valid neighbors and I should go clean it up. It’s also not the fastest code, completing the entire search space on the input in around 3 seconds.

### Part 2

The version of the code where I just update the neighbors algorithm to ignore slopes? It’s still running right now as I type this. Has not yet spit out an answer and I’m sure it won’t by the time I kill it.

If you look at the input you’ll see that there are a ton of long, connected hallways. So we’re going to use edge compression between any points with > 2 neighbors (intersections). First a small BFS (`get_edges`) and then a DFS through that. It cuts the problem space way down. The code is still slow, taking 31 seconds to run both problems. So I clearly need to prune some of the problem space more. But that’s a problem for Future David :tm:.
