fn main() {
    //println!("Hello, world!");
    const word1: &str = "Wort1";
    const word2: &str = "Wort2";
    println!("{:?}", word1.chars());
    println!("{:?}", word2.chars());
    const len_word1: usize = word1.len();
    const len_word2: usize = word2.len();
    println!("{}", len_word1);
    println!("{}", len_word2);
    let mut table = [[0u8;len_word1 + 1]; len_word2 + 1];
    //table[0][1] = 42;
    println!("{:#?}", &table);


}
