use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_query(&args);

    let result = get_address_class(&parse_address_splices(&config.ipaddr))
        .expect("Your entered value is incorrect");
    println!("{}", result);
}

struct Config {
    ipaddr: String,
}

fn parse_query(args: &[String]) -> Config {
    let ipaddr = args[1].clone();

    Config { ipaddr  }
}

fn parse_address_splices(ipaddr: &String) -> Vec<i32> {
    let addr_splices: Vec<&str> = ipaddr.split(".").collect();

    let mut addr_i32_splices: Vec<i32> = Vec::new();
    for octet in addr_splices {
        addr_i32_splices.push(octet.parse().unwrap());
    }
    
    addr_i32_splices
}

fn get_address_class(addr_splices: &Vec<i32>) -> Result<&'static str, &'static str> {
    let mut senior_octet: Vec<char> = format!("{:b}", addr_splices[0]).to_string().chars().collect();

    while senior_octet.len() < 8 {
        let mut zero_vec = vec!['0'];
        zero_vec.append(&mut senior_octet);
        senior_octet = zero_vec;
    }

    if senior_octet[0] == '0' {
        Ok("A")
    } else if senior_octet[1] == '0' {
        Ok("B")
    } else if senior_octet[2] == '0' {
        Ok("C")
    } else if senior_octet[3] == '0' {
        Ok("D")
    } else if senior_octet[4] == '0' {
        Ok("E")
    } else {
        Err("invalid address")
    }
}
