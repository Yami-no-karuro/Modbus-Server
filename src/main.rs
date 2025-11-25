use std::thread;
use std::io;
use std::io::Read;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::Ipv4Addr;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

fn get_timestamp() -> String 
{
    let now = SystemTime::now();
    let datetime = now.duration_since(UNIX_EPOCH)
        .unwrap();

    let total_secs = datetime.as_secs();
    let hours = (total_secs / 3600) % 24;
    let minutes = (total_secs / 60) % 60;
    let seconds = total_secs % 60;

    return format!("[{:02}:{:02}:{:02}]", 
        hours, 
        minutes, 
        seconds
    );
}

fn handle_request(mut stream: TcpStream) -> io::Result<()> 
{
    let timestamp: String = get_timestamp();
    println!("- {}:", timestamp);
    
    // Parsing Modbus TCP Request
    // Modbus TCP ADU (Application Data Unit):
    // 
    // HEADER (7 bytes):
    // 0-1  Transaction ID       (2 bytes)  -> Identifies the transaction, used to correlate request/response
    // 2-3  Protocol ID          (2 bytes)  -> Always 0x0000 for Modbus
    // 4-5  Length               (2 bytes)  -> Number of bytes that follow (Unit ID + PDU)
    // 6    Unit ID              (1 byte)   -> Identifies the slave device
    //
    // PDU (Protocol Data Unit):
    // 7    Function Code        (1 byte)   -> Which Modbus function to execute (e.g., read registers)
    // 8-9  Starting Address     (2 bytes)  -> Starting address of coil/register
    // 10-11 Quantity of regs    (2 bytes)  -> Number of registers/coils to read or write
    //
    // Note: the minimum complete frame is 12 bytes (7 header + 5 PDU)
    
    let mut buffer = [0u8; 512];
    let size: usize = stream.read(&mut buffer)?;

    if size < 12 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Malformed! Modbus Frame too short.",
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
