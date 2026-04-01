use std::io::{Read, Write};
use rand::Rng;
use reqwest::blocking::Client;
use std::thread;
use std::time::Duration;
use std::fs;
use std::fs::OpenOptions;



fn gen_code(len: usize) -> String {
    let charsiki = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM1234567890";
    let mut rng = rand::thread_rng();
    
    (0..len).map(|_| {
        let i = rng.gen_range(0..charsiki.len());
        charsiki.chars().nth(i).unwrap()
    }).collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    loop {
        let client = Client::new();

        let code = gen_code(5);
        println!("тест {code}");

        let response = client.get(&format!("https://dropmefiles.com/{}", code))
            .send()?;


        println!("Статус: {}", response.status());
        
        let resp = response.text()?;

        if resp.contains("var UPLOADID =") {
            println!("+");
            println!("https://dropmefiles.com/{}", code);
            let url = format!("https://dropmefiles.com/{}\n", code);
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("good.txt")
                .unwrap();
            
            file.write_all(url.as_bytes()).unwrap();
            
        } else {
            println!("-");
        }

        thread::sleep(Duration::from_millis(2000));

    }
    Ok(())
}
