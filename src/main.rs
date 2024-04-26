fn dobro(numero: i32) -> i32
{
    return 2 * numero;
}

fn maior(a: i32, b: i32) -> i32
{
    if a >= b
    {
        return a;
    }
    else
    {
        return b;
    }
}

fn soma(valor_a: f32, valor_b: f32) -> f32
{
    // podemos usar o return e com o valor da soma.
    // return valor_a + valor_b;

    // retornando um numero
    // 10f32 converte o inteiro 10 para float.
    // 10 as f32 também converte o inteiro 10 para float.
   
    // o compilador também aceita sem o ";" sendo a ultima linda da função.
    valor_a + valor_b
}

fn main()
{
    let x: i32;
    x = 25;
    println!("O dobro de {} é {}", x, dobro(x));

    let a = 2;
    let b = 1;
    println!("O maior numero entre {} e {} é {}", a, b, maior(a, b));


    let v1: f32;
    let v2: f32;

    v1 = 5.0;
    v2 = 7.0;
    println!("A soma entre {} e {} é {}", v1, v2, soma(v1, v2));
}
