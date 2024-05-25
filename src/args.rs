pub enum Type {
    GenKeys,
    Decode,
    Encode,
    Server,
    Client
}

pub fn get_type(args: &mut impl Iterator<Item = String>) -> Result<Type, &'static str> {
    args.next();
    let type_str = match args.next() {
        Some(x) => x,
        None => return Err("Not operation specified. Usage: rsa_tool <gen|dec|enc|srv|cli> <other arguments>")
    };
    match type_str.as_str() {
        "gen" => Ok(Type::GenKeys),
        "enc" => Ok(Type::Encode),
        "dec" => Ok(Type::Decode),
        "srv" => Ok(Type::Server),
        "cli" => Ok(Type::Client),
        _ => Err("Invalid operation. Usage: rsa_tool <gen|dec|enc|srv|cli> <other arguments>")
    }
}

pub struct GenArgs {
    pub pub_filename: String,
    pub priv_filename: String
}

impl GenArgs {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<GenArgs, &'static str> {
        let pub_filename = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments. Usage: rsa_tool gen <pub filename> <priv filename>")
        };

        let priv_filename = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments. Usage: rsa_tool gen <pub filename> <priv filename>")
        };

        Ok(GenArgs { pub_filename, priv_filename })
    }
}

pub struct DecArgs {
    pub priv_filename: String,
    pub ciphertext_filename: String,
    pub plaintext_filename: String
}

impl DecArgs {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<DecArgs, &'static str> {
        let priv_filename = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments. Usage: rsa_tool dec <priv filename> <ciphertext/input filename> <plaintext/output filename>")
        };

        let ciphertext_filename = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments. Usage: rsa_tool dec <priv filename> <ciphertext/input filename> <plaintext/output filename>")
        };

        let plaintext_filename = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments. Usage: rsa_tool dec <priv filename> <ciphertext/input filename> <plaintext/output filename>")
        };

        Ok(DecArgs { priv_filename, ciphertext_filename, plaintext_filename })
    }
}

pub struct EncArgs {
    pub pub_filename: String,
    pub ciphertext_filename: String,
    pub plaintext_filename: String
}

impl EncArgs {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<EncArgs, &'static str> {
        let pub_filename = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments. Usage: rsa_tool enc <pub filename> <plaintext/input filename> <ciphertext/output filename>")
        };

        let plaintext_filename = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments. Usage: rsa_tool enc <pub filename> <plaintext/input filename> <ciphertext/output filename>")
        };

        let ciphertext_filename = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments. Usage: rsa_tool enc <pub filename> <plaintext/input filename> <ciphertext/output filename>")
        };

        Ok(EncArgs { pub_filename, ciphertext_filename, plaintext_filename })
    }
}

pub struct SrvArgs {
    pub port: u16
}

impl SrvArgs {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<SrvArgs, &'static str> {
        let port_str = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments. Usage: rsa_tool srv <tcp port>")
        };
        let port = match port_str.parse::<u16>() {
            Ok(x) => x,
            Err(_) => return Err("Invalid port specified.")
        };

        Ok(SrvArgs { port })
    }
}

pub struct ClientArgs {
    pub host: String,
    pub port: u16
}

impl ClientArgs {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<ClientArgs, &'static str> {
        let host = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments. Usage: rsa_tool cli <tcp server> <tcp port>")
        };

        let port_str = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments. Usage: rsa_tool cli <tcp server> <tcp port>")
        };
        let port = match port_str.parse::<u16>() {
            Ok(x) => x,
            Err(_) => return Err("Invalid port specified.")
        };

        Ok(ClientArgs { host, port })
    }
}