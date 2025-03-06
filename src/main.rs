use std::collections::HashMap;
use serde_json;
use std::io;
use std::io::Write;
use std::fs;
use rand::Rng;

//Base62 encoding
fn to_base62(num: i32) -> String {
    let chars = String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    let mut result:String = String::from("");
    if num == 0 {
        return chars.chars().nth(0).unwrap().to_string();
    }
    let mut num = num;
    while num > 0 {
        let remainder = num % 62;
        result += chars.chars().nth(remainder as usize).unwrap().to_string().as_str();
        num /= 62;
    }

    result
}

fn main() {
    //Read file and initialize new URL mappings
    let mut file = fs::OpenOptions::new().write(true).open("url_mappings.json").expect("Failed to open/create file");
    let mut url_mappings:HashMap<String, String> = HashMap::new();
    
    loop {
        println!("Enter Long URL (Type in exit to quit): ");
        let mut long_url = String::new();
        io::stdin().read_line(&mut long_url).expect("Unable to retrieve long URL");
        long_url = long_url.trim().to_string();

        if long_url.to_lowercase() == "exit"{
            break;
        }

        //Encode shortened URL
        let url_encoding = rand::thread_rng().gen_range(1..100000); //Random number generator
        let url_str = to_base62(url_encoding);
        let short_url = format!("short.ly/{}", url_str);
        println!("Short URL: {}", short_url);

        url_mappings.insert(long_url, short_url);
    }

    let file_contents = fs::read_to_string("url_mappings.json").unwrap(); //Get file contents as a string
    let mut filemap = serde_json::from_str::<HashMap<String, String>>(&file_contents).unwrap(); //Convert the file contents to a hashmap
    filemap.extend(url_mappings); //Add the current url_mappings to the URLs already stored in the file; with extend it'll automatically update to the new short URLs if there were any duplicates

    let serialized_urls = serde_json::to_string_pretty(&filemap).expect("Failed to serialize file");
    file.write_all(serialized_urls.as_bytes()).expect("Failed to write to file");
    println!("Thank you for using this program!");
}
