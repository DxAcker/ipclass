use structopt::StructOpt;
use regex::Regex;

fn main() -> Result<(), &'static str> {
    let args = Config::from_args();

    if validate_address(&args.ipaddr, args.is_binary) {
        let result = get_address_class(&args.ipaddr, args.is_binary)
            .expect("Class is not exists for this ip (senior binary octet)");

        println!("{}", result);
        Ok(())
    } else {
        Err("Invalid input")
    }
}

#[derive(StructOpt)]
struct Config {
    #[structopt(short = "b", long = "binary")]
    is_binary: bool,
    ipaddr: String,
}

fn parse_address_splices(ipaddr: &String) -> (Vec<&str>, Vec<i32>) {
    let addr_splices: Vec<&str> = ipaddr.split(".").collect();

    let mut addr_i32_splices: Vec<i32> = Vec::new();
    for octet in addr_splices.iter() {
        addr_i32_splices.push(octet.parse().unwrap());
    }

    (addr_splices, addr_i32_splices)
}

fn get_address_class(ipaddr: &String, is_binary: bool) -> Result<&'static str, String> {
    let (_, addr_splices) = parse_address_splices(&ipaddr);

    let mut senior_octet: Vec<char>;

    if !is_binary {
        senior_octet = format!("{:b}", addr_splices[0])
            .to_string()
            .chars()
            .collect();
    } else {
        senior_octet = addr_splices[0].to_string().chars().collect();
    }

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

fn validate_address(ipaddr: &String, is_binary: bool) -> bool {
    if !is_binary {
        let (_, ipaddr_splices) = parse_address_splices(&ipaddr);

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
    } else {
        let binary_re = Regex::new(r"^([0-1]{8}\.){3}[0-1]{8}$").unwrap();

        binary_re.is_match(&ipaddr[..])
    }
}
