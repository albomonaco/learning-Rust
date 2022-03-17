fn main(){
    const PI:f32 = 3.14;

    println!("PI = {}", PI);

    let variavel:i32 = 300;
    println!("variavel = {}, tamanho = {} bytes", variavel, std::mem::size_of_val(&variavel));
    
    let decimal:f32 = 2.5;
    println!("decimal = {}", decimal);

    let booleana = true;
    println!("Booleana = {}, Tamanho booleana = {} bytes",booleana ,std::mem::size_of_val(&booleana));

    let letra:char= 'C';
    println!("letra = {}, tamanho da letra = {} bytes", letra, std::mem::size_of_val(&letra));
}