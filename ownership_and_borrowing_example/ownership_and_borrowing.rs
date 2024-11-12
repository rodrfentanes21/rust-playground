fn main() {
    // *  Ownership
    let s1 = String::from("Olá, mundo!"); // s1 é o dono da String
    let s2 = s1; // Transfere a posse de s1 para s2

    //! ERRO: s1 não é mais válido após a transferência
    // println!("{}", s1);
    //? Funciona, pois s2 é o novo dono da String
    println!("{}", s2);

    // * Borrowing (Empréstimo Imutável)
    let s3 = String::from("Rust");

    let len = calculate_length(&s3); // Passa uma referência de s3, sem transferir a posse
    println!("O comprimento de '{}' é {}.", s3, len); // s3 ainda é válido aqui
}

//* Função que recebe uma referência imutável
fn calculate_length(s: &String) -> usize {
    s.len()
}