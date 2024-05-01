pub enum Type {
    GenKeys,
    Decode,
    Encode
}

pub fn get_type(args: &mut impl Iterator<Item = String>) -> Result<Type, &'static str> {
    args.next();
    let type_str = match args.next() {
        Some(x) => x,
        None => return Err("Not operation specified. Usage: rsa <gen|dec|enc> <other arguments>")
    };
    match type_str.as_str() {
        "gen" => Ok(Type::GenKeys),
        "enc" => Ok(Type::Encode),
        "dec" => Ok(Type::Decode),
        _ => Err("Invalid operation. Usage: rsa <gen|dec|enc> <other arguments>")
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
            None => return Err("Not enough arguments. Usage: rsa gen <pub filename> <priv filename>")
        };

        let priv_filename = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments. Usage: rsa gen <pub filename> <priv filename>")
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
            None => return Err("Not enough arguments. Usage: rsa dec <priv filename> <ciphertext/input filename> <plaintext/output filename>")
        };

        let ciphertext_filename = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments. Usage: rsa dec <priv filename> <ciphertext/input filename> <plaintext/output filename>")
        };

        let plaintext_filename = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments. Usage: rsa dec <priv filename> <ciphertext/input filename> <plaintext/output filename>")
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
            None => return Err("Not enough arguments. Usage: rsa enc <pub filename> <plaintext/input filename> <ciphertext/output filename>")
        };

        let plaintext_filename = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments. Usage: rsa enc <pub filename> <plaintext/input filename> <ciphertext/output filename>")
        };

        let ciphertext_filename = match args.next() {
            Some(x) => x,
            None => return Err("Not enough arguments. Usage: rsa enc <pub filename> <plaintext/input filename> <ciphertext/output filename>")
        };

        Ok(EncArgs { pub_filename, ciphertext_filename, plaintext_filename })
    }
}
