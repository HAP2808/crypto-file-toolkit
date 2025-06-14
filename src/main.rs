use clap::{Parser};

pub mod crypto;
#[derive(Parser)]
#[command(name = "crypto_file_toolkit", before_help = "Use this tool to encrypt or decrypt files using a specified key. \n\n\
EXAMPLES:\n\
  crypto_file_toolkit --encrypt --file input.txt --key mypassword --output encrypted.bin\n\
  crypto_file_toolkit --decrypt --file encrypted.bin --key mypassword --output decrypted.txt")]
struct Args {
    #[arg(short, long, help = "Encrypt the file")]
    encrypt: bool,

    #[arg(short, long, help = "Decrypt the file")]
    decrypt: bool,

    #[arg(long, help = "Path to the file to be processed")]
    file: String,

    #[arg(long, help = "Key used for encryption/decryption")]
    key: String,

    #[arg(short, long, help = "Output file path")]
    output: String,
}

fn main() {
    let args = Args::parse();
    
    if args.encrypt {
        let result = crypto::encrypt_file(&(args.file), &(args.output), args.key);
        if result.is_ok() {
            println!("File encrypted successfully.");
        } else {
            println!("Error encrypting file: {}", result.unwrap_err());
        }
    } else if args.decrypt {
        let result = crypto::decrypt_file(&(args.file), &(args.output), args.key);
        if result.is_ok() {
            println!("File decrypted successfully.");
        } else {
            println!("Error decrypting file: {}", result.unwrap_err());
        }
    } 
    else {
        println!("Specify --encrypt or --decrypt");
    }
}
