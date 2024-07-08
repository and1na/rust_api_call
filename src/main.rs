use serde::Deserialize;
use reqwest::Error;
use reqwest::header::USER_AGENT;


//We will use this Struct as method to deserialize the JSON response from the API
#[derive(Deserialize, Debug)]
struct User{
    login: String ,
    id: u32
}

/*
Se utiliza para configurar una función como el punto de entrada principal
de una aplicación asincrónica que utiliza Tokio como el runtime asincrónico.
 */
#[tokio::main]
async fn main() -> Result<(), Error> {
    //URL de la API
    let url: String = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook");

    println!("URL: {}", url);


    let client = reqwest::Client::new();

    //&url es una referencia a la variable url, no es como pasarle directamente la variable
    //esto es por el moving de Rust (cambia la propiedad de la variable) y luego no puedes
    //cambiarla de nuevo 
    let response = client
        .get(&url)
        .header(USER_AGENT, "rust web-api-client demo")
        .send()
        //el ? es para manejar el error de manera más limpia, si la expresión devuelve un error
        //se devuelve el error, si no, se sigue con la ejecución (asigna el valor a la variable)
        .await?;

    let users : Vec<User> = response.json().await?;

    println!("{:?}", users);


    //Aqui se podria escribir o "Ok(())" o "return Ok(())"
    //porque el return implicito de Rust se escribe sin el punto y coma
    //en el caso de que pusiera el ; solo se evaluaria la funcion pero no
    //se devolveria el valor
    return Ok(());
}

