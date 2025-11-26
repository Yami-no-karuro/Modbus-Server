use std::io;
use std::io::Write;
use std::net::TcpStream;

pub fn read_holding_registers(
    mut stream: TcpStream,
    transaction_id: u16,
    protocol_id: u16,
    unit_id: u8,
    quantity: u16
) -> Result<(), io::Error>
{
    let byte_count: u8 = (quantity * 2) as u8;
    let mut data: Vec<u8> = Vec::new();
    
    // Example...
    for _ in 0..quantity {
        data.push(0x12);
        data.push(0x34);
    }

    let response_length: usize = 3 + data.len();
    let mut response: Vec<u8> = Vec::new();
    
    response.extend(&transaction_id.to_be_bytes());
    response.extend(&protocol_id.to_be_bytes());
    response.extend(&(response_length as u16).to_be_bytes());
    
    response.push(unit_id);
    response.push(0x03);
    response.push(byte_count);
    
    response.extend(&data);

    stream.write_all(&response)?;
    return Ok(());
}

pub fn handle_illegal_function(
    mut stream: TcpStream,
    transaction_id: u16,
    protocol_id: u16,
    unit_id: u8,
    function_code: u8,
    exception_code: u8
) -> Result<(), io::Error> 
{
    let pdu_len: u16 = 3;
    let exception_fn: u8 = function_code | 0x80;
    let mut response: Vec<u8> = Vec::new();

    response.extend(&transaction_id.to_be_bytes());
    response.extend(&protocol_id.to_be_bytes());
    response.extend(&pdu_len.to_be_bytes());
    
    response.push(unit_id);
    response.push(exception_fn);
    response.push(exception_code);

    stream.write_all(&response)?;
    return Ok(());
}
