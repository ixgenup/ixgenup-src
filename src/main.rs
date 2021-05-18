use std::io::{stdin,stdout,Write};
fn main() {
    println!("Ixgenup 1.0.0 for {} {}", std::env::consts::OS, std::env::consts::ARCH);
    println!("Type \"new\" to post random data to a new URL or type \"get\" to get data from a site.");
    lp();
}

fn lp(){
    let mut new=String::new();
    print!("ixgenup$ ");
    let _=stdout().flush();
    stdin().read_line(&mut new).expect("Did not enter a correct string");
    if let Some('\n')=new.chars().next_back() {
        new.pop();
    }
    if let Some('\r')=new.chars().next_back() {
        new.pop();
    }
    if new == "new" {
        let mut ur=String::new();
        print!("Enter a URL: ");
        let _=stdout().flush();
    stdin().read_line(&mut ur).expect("Did not enter a correct string");
        if let Some('\n')=ur.chars().next_back() {
            ur.pop();
        }
        if let Some('\r')=ur.chars().next_back() {
            ur.pop();
        }
        let mut bod=String::new();
        print!("Enter a body: ");
        let _=stdout().flush();
    stdin().read_line(&mut bod).expect("Did not enter a correct string");
        if let Some('\n')=bod.chars().next_back() {
            bod.pop();
        }
        if let Some('\r')=bod.chars().next_back() {
            bod.pop();
        }
        post(&ur, &bod);
    } else if new == "get" {
        let mut ur=String::new();
        print!("Enter a URL: ");
        let _=stdout().flush();
    stdin().read_line(&mut ur).expect("Did not enter a correct string");
        if let Some('\n')=ur.chars().next_back() {
            ur.pop();
        }
        if let Some('\r')=ur.chars().next_back() {
            ur.pop();
        }
        get(&ur);
    } else {
        println!("Command not recognized");
        lp();
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn post(url: &str, body: &str) {
    let response = reqwest::Client::new()
        .post(url)
        .body(format!("{}", body))
        .send()
        .await
        .expect("send");
    println!("Response status {}", response.status());
    lp();
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn get(urlforget: &str){
    let body = reqwest::get(urlforget).await.unwrap().text().await.unwrap();

    println!("body = {}", body);

    lp();
}
