use std::io;
use std::io::Read;
use std::io::Write;
use std::io::Seek;
use std::io::SeekFrom;
use std::env;
use std::process;
use std::fs::File;
use std::fs::OpenOptions;
use std::any::type_name;

const PNG_SIG_CAPACITY: usize = 8;
const BUFFER_CAPACITY: usize = 32*1024;
const IHDR_CHUNK: [u8; 4] = [73, 72, 68, 82];

fn type_of<T>(_: T) -> &'static str
{
    type_name::<T>()
}

fn read_bytes(mut file: &File, buffer: &mut [u8], st: &str) -> io::Result<()>
{
    if file.read(buffer)? == 0
    {
        eprint!("ERROR: cannot read bytes from file");
        process::exit(1);
    }
    else
    {
        print!("{}: ", st);
        for byte in buffer
        {
            match st
            {
                "Chunk Type" => print!("{}", byte),
                _ => print!("")
            }
        }
        print!("\n");
    }

    Ok(())
}

fn write_bytes(mut file: &File, buffer: &[u8], st: &str) -> io::Result<()>
{
    file.write_all(buffer).expect("Some error");

    //print!("Writing {}:", st);
    //for byte in buffer
    //{
    //    print!("{}", byte);
    //}
    //print!("\n");

    
    Ok(())
}

fn read_file(mut in_file: &File, mut out_file: &File) -> io::Result<()>
{
    let mut sig: [u8; PNG_SIG_CAPACITY] = [0; PNG_SIG_CAPACITY];
    let mut chunk_buffer: [u8; BUFFER_CAPACITY] = [0; BUFFER_CAPACITY];

    read_bytes(in_file, &mut sig, "Signature")?;
    write_bytes(out_file, &sig, "Signature")?;
    print!("------------------------------\n");

    loop
    {
        let mut length: [u8; 4] = [0; 4];
        let mut ctype: [u8; 4] = [0; 4];
        let mut crc: [u8; 4] = [0; 4];

        read_bytes(in_file, &mut length, "Length")?;
        write_bytes(out_file, &length, "Length")?;

        read_bytes(in_file, &mut ctype, "Chunk Type")?;
        write_bytes(out_file, &ctype, "Chunk Type")?;
        // Check for IHDR chunk
        if ctype == IHDR_CHUNK 
        {
            let chunk_length: u32 = 3;
            write_bytes(out_file, &chunk_length.to_be_bytes(), "chunk_length");

            let chunk_type: &str = "zuRA";
            write_bytes(out_file, &chunk_type.as_bytes(), "chunk_type");

            let chunk_data: &str = "yep";
            write_bytes(out_file, &chunk_data.as_bytes(), "chunk_data");

            let chunk_crc: u32 = 0;
            write_bytes(out_file, &chunk_crc.to_be_bytes(), "chunk_crc");
        }
        print!("{}", u32::from_be_bytes(ctype));
        

        let mut n: usize = u32::from_be_bytes(length) as usize;
        while n > 0
        {
            let mut m: usize = n;
            if m > BUFFER_CAPACITY
            {
                m = BUFFER_CAPACITY;
            }

            read_bytes(in_file, &mut chunk_buffer, "Chunk Data")?;
            write_bytes(out_file, &chunk_buffer, "Chunk Data")?;
            n -= m;
        }
        let yl: &str = "yleoshen";
        write_bytes(out_file, &yl.as_bytes(), "chunk_crc");

        //file.seek(SeekFrom::Current(u32::from_be_bytes(length) as i64));
        read_bytes(in_file, &mut crc, "CRC")?;
        write_bytes(out_file, &crc, "CRC")?;

        

        print!("------------------------------\n");
    }

    Ok(())
}

fn write_file(mut file: &File) -> io::Result<()>
{
    let length: [u8; 4] = [1, 2, 3, 4];
    let ctype: [u8; 4] = [1, 2, 3, 4];
    let crc: [u8; 4] = [1, 2, 3, 4];

    write_bytes(file, &length, "Length")?;
    write_bytes(file, &ctype, "Chunk Type")?;
    write_bytes(file, &crc, "CRC")?;

    Ok(())
}

fn usage()
{
    print!("Usage: ./main input_file.png output_file.png\n");
}

fn main() -> io::Result<()>
{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2
    {
        print!("ERROR: No input file provided\n");
        usage();
        process::exit(1);
    }
    else if args.len() < 3
    {
        print!("ERROR: No output file specified\n");
        usage();
        process::exit(1);
    }

    let input_file: &String = &args[1];
    let output_file: &String = &args[2];
    let mut in_file = File::open(input_file)?;
    let mut out_file = File::create(output_file)?;

    read_file(&in_file, &out_file);
    //write_file(&file);

    Ok(())
}
