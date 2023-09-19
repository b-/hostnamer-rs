use random_word::Lang;
use std::net;

struct HostPair {
    name: String,
    ip: net::IpAddr,
}

impl Default for HostPair {
    fn default() -> Self {
        HostPair{name: generate_hostname(2), ip: [0,0,0,0].into()}
    }
}

fn main() {
    let host = HostPair{ip: [192,168,12,34].into(), ..Default::default()};
    println!("name: {}, ip: {}", host.name, host.ip);
}

fn generate_hostname(wordcount: usize) -> String {
    assert!(wordcount >= 1);
    let mut words: Vec<String> = Vec::with_capacity(wordcount);
    for _ in 0..wordcount{
        words.push(String::from(random_word::gen(Lang::En)));        
    };
    let mut hostname = words.pop().unwrap();
    while let Some(word) = words.pop() {
            hostname = hostname + "-" + &word;
    }
    hostname
}

