use reqwest;
use std::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel();
    for _ in 0..500 {
        let tx_clone = mpsc::Sender::clone(&tx);
        tokio::spawn(async move {
            let resp = reqwest::get("http://upcdatabase.org/random").await.unwrap();
            let url = resp.url();
            let upc = url.path().replace("/code/", "");
            tx_clone.send(upc).unwrap()
        });
    }
    drop(tx);

    for received in rx {
        println!("{}", received);
    }

    println!("done!");

    Ok(())
}
