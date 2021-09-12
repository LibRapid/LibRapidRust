/**
Implements the Huffman-compression algorithm in Rust.

# Efficiency
Although the efficiency varies for each text, you could say that it is one of the most efficient ways of compressing text.

## Example
Task: *Compress a "Lorem Ipsum" .txt file with 10,239,270 Bytes (9.77 MB)*
\
\
Average Time (over 100 iterations): **1.17 Seconds**[^1]

Old Size: **10,239,270 Bytes** or **9.77 MB**

New Size: **5,399,428 Bytes** or **5.15 MB**

Space Savings: **47.27 %** (**4,839,842 Bytes** or **4.62 MB**)
\
\
\
[^1] Note that this also includes reading and writing from/to files which is performance-wise expensive.

# Attention
This alogrithm roughly gets more efficient the bigger texts it has to store.

# Trivia
Huffman invented this algorithm for text compression, but this is now the base of many more compression methods.
He proved that this was the mathematically most efficient way of assigning bits to chars (can also be used for strings).
It works by calculating the overall probablity for each character to appear. The one with the highest probability gets the lowest value etc.

*/
pub mod huffman;