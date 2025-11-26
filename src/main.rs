use std::thread;
use std::io;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::Ipv4Addr;

mod time;
mod functions;

fn handle_request(mut stream: TcpStream) -> Result<(), io::Error>
{
    let current_time: String = time::get_time();
    println!("- {}:", current_time);
    
    let mut buffer = [0u8; 512];
    let size: usize = stream.read(&mut buffer)?;

    if size < 12 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unable to parse request. Modbus Frame too short.",
        ));
    }

    let transaction_id: u16 = u16::from_be_bytes([buffer[0], buffer[1]]);
    let protocol_id: u16 = u16::from_be_bytes([buffer[2], buffer[3]]);
    let length: u16 = u16::from_be_bytes([buffer[4], buffer[5]]);
    let unit_id: u8 = buffer[6];

    println!("Transaction ID:   {}", transaction_id);
    println!("Protocol ID:      {}", protocol_id);
    println!("Length:           {}", length);
    println!("Unit ID:          {}", unit_id);

    let function: u8 = buffer[7];
    let starting_address: u16 = u16::from_be_bytes([buffer[8], buffer[9]]);
    let quantity: u16 = u16::from_be_bytes([buffer[10], buffer[11]]);
    
    let function_literal: &str = match function {
        0x01 => "Read Coils",
        0x02 => "Read Discrete Inputs",
        0x03 => "Read Holding Registers",
        0x04 => "Read Input Registers",
        0x05 => "Write Single Coil",
        0x06 => "Write Single Register",
        0x07 => "Read Exception Status",
        0x08 => "Diagnostics",
        0x0B => "Get Comm Event Counter",
        0x0C => "Get Comm Event Log",
        0x0F => "Write Multiple Coils",
        0x10 => "Write Multiple Registers",
        0x11 => "Report Server ID",
        0x14 => "Read File Record",
        0x15 => "Write File Record",
        0x16 => "Mask Write Register",
        0x17 => "Read/Write Multiple Registers",
        0x18 => "Read FIFO Queue",
        _ => "Unknown",
    };

    println!("Function:         {}", function_literal);
    println!("Starting addr:    {}", starting_address);
    println!("Quantity:         {}", quantity);

    // Example...
    // Handling for example the "Read Holding Register" function.
    if function == 0x03 {
        let _ = functions::read_holding_registers(
            stream, 
            transaction_id, 
            protocol_id, 
            unit_id, 
            quantity
        );
    } else {
        let _ = functions::handle_illegal_function(
            stream,
            transaction_id,
            protocol_id,
            unit_id,
            function,
            0x01
        );
    }

    return Ok(());
}

fn main() -> Result<(), io::Error> 
{
    let port: u16 = 5002;
    let ip_address = Ipv4Addr::new(127, 0, 0, 1);
    let listener: TcpListener = TcpListener::bind((ip_address, port))?;

    println!("=======================");
    println!("==== Modbus Server ====");
    println!("=======================");
    println!("");

    println!("Status: Ready");
    println!("Address: 127.0.0.1:5002");

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        thread::spawn(move || {
            if let Err(e) = handle_request(stream) {
                eprintln!("Error: \"{}\".", e);
            }
        });
    }

    return Ok(());
}
