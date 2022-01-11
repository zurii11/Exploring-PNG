use std::io;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::env;
use std::process;
use std::fs::File;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str
{
    type_name::<T>()
}

fn read_bytes(mut file: &File, buffer: &mut [u8], st: &str) -> io::Result<()>
{
    if file.read(buffer)? == 0
    {
        eprintln!("ERROR: cannot read bytes from file {}", st);
        process::exit(1);
    }
    else
    {
    
        for byte in buffer
        {
            print!("{} ", byte);
        }
        println!("");
    }

    Ok(())
}

fn main() -> io::Result<()>
{
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 1, "ERROR: file not provided!");

    let filepath: &String = &args[1];
    let mut file = File::open(filepath)?;

    let mut sig: [u8; 8] = [0; 8];
    print!("Signature: ");
    read_bytes(&file, &mut sig, "sig");

    while true
    {
        let mut length: [u8; 4] = [0; 4];
        let mut ctype: [u8; 4] = [0; 4];
        let mut crc: [u8; 4] = [0; 4];

        print!("Length: ");
        read_bytes(&file, &mut length, "length");
        print!("Chunk Type: ");
        read_bytes(&file, &mut ctype, "ctype");
        file.seek(SeekFrom::Current(u32::from_be_bytes(length) as i64));
        print!("CRC: ");
        read_bytes(&file, &mut crc, "crc");
    }

    Ok(())
}
