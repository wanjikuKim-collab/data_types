fn main() {
    // Compound data types
        // tuples
    let tuple = ("Cheers to learning rust", 3);
    let months = tuple.1;// dot notation
    let (celebration, months_learning)= tuple;
    println!("I've learnt rust for {} months", months);
    println!("{} for {} months", celebration, months_learning);
}
