use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match parse_query(&args) {
        Ok(conf) => conf,
        Err(_) => {
            panic!(
                "\n
Please, try using `ipclass [-b/--binary] 127.0.0.1`\n 
Where you can change 127.0.0.1 to any ip\n\n
Using flag -b or --binary enter binary ip to get its class
                "
            );
        }
    };
    let addr_splices =
        parse_address_splices(&config.ipaddr).expect("Your entered value is incorrect");
    let result = get_address_class(&addr_splices)
        .expect("Class is not exists for this ip (senior binary octet)");

    println!("{}", result);
}

struct Config {
    ipaddr: String,
}

fn parse_query(args: &[String]) -> Result<Config, &'static str> {
    match args.len() {
        2 => Ok(Config {
            ipaddr: args[1].clone(),
        }),
        3 => Ok(Config {
            ipaddr: args[2].clone(),
        }),
        _ => Err(""),
    }
}

fn parse_address_splices(ipaddr: &String) -> Result<Vec<i32>, &'static str> {
    let addr_splices: Vec<&str> = ipaddr.split(".").collect();

    let mut addr_i32_splices: Vec<i32> = Vec::new();
    for octet in addr_splices {
        addr_i32_splices.push(octet.parse().unwrap());
    }

    if validate_address(&addr_i32_splices) {
        Ok(addr_i32_splices)
    } else {
        Err("Error while trying parse address splices")
    }
}

fn get_address_class(addr_splices: &Vec<i32>) -> Result<&'static str, String> {
    let mut senior_octet: Vec<char> = format!("{:b}", addr_splices[0])
        .to_string()
        .chars()
        .collect();

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
    } else if addr_splices[0] != 255 {
        Ok("E")
    } else {
        let senior_octet: String = senior_octet.into_iter().collect();
        Err(senior_octet)
    }
}

fn validate_address(ipaddr_splices: &Vec<i32>) -> bool {
    if ipaddr_splices.len() != 4 {
        false
    } else if ipaddr_splices
        .iter()
        .any(|&splice| splice < 0 || splice > 255)
    {
        false
    } else {
        true
    }
}
