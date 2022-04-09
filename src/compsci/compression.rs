//! Algorithms for compressing data. We're working hard to bring other algorithms into LibRapid!
/// Implements the Huffman-compression algorithm in Rust.
/// # Efficiency
/// Although the efficiency varies for each text, you could say that it is one of the most efficient ways of compressing text.
/// ## Example
/// Task: *Compress a "Lorem Ipsum" .txt file with 10,000,000 Bytes.*
/// \
/// \
/// Average Time (over 100 iterations in release mode): **710 Milliseconds**
/// Old Size: **10,0 Megabytes.**
/// New Size: **5.3048 Megabytes.**
/// Space Savings: **~46.85 %.**
/// # Attention
/// This algorithm roughly gets more efficient the bigger texts it has to store.
/// # Trivia
/// Huffman invented this algorithm for text compression, but this is now the base of many more compression methods.
/// He proved that this was the mathematically most efficient way of assigning bits to chars (can also be used for strings).
/// It works by calculating the overall probablity for each character to appear. The one with the highest probability gets the lowest value etc.
pub mod huffman;