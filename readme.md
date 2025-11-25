# Modbus Server

### Intro

Modbus (or MODBUS) is a client/server data communications protocol in the application layer.  
It was originally designed for use with programmable logic controllers (PLCs), but has become a de facto standard communication protocol for communication between industrial electronic devices in a wide range of buses and networks.

### TCP ADU (Application Data Unit)

Note: the minimum complete frame is 12 bytes (7 header + 5 PDU)

| Field                 | Offset | Size     | Description                                          |
|-----------------------|--------|----------|------------------------------------------------------|
| Transaction ID        | 0–1    | 2 bytes  | Identifies the transaction for request/response      |
| Protocol ID           | 2–3    | 2 bytes  | Always 0x0000 for Modbus                             |
| Length                | 4–5    | 2 bytes  | Number of following bytes (Unit ID + PDU)            |
| Unit ID               | 6      | 1 byte   | Identifies the slave device                          |
| Function Code         | 7      | 1 byte   | Modbus function to execute                           |
| Starting Address      | 8–9    | 2 bytes  | Starting coil/register address                       |
| Quantity of Registers | 10–11  | 2 bytes  | Number of registers/coils to read or write           |


### How to get things running 

Launch the server via `cargo run`.  
Once the server is up and running we can emulate actual requests via [netcat](en.wikipedia.org/wiki/Netcat):

```bash
# [
#     0x00, 0x01,
#     0x00, 0x00,
#     0x00, 0x06,
#     0x01,
#     0x03,
#     0x00, 0x00,
#     0x00, 0x0A
# ]
printf "\x00\x01\x00\x00\x00\x06\x01\x03\x00\x00\x00\x0A" | nc 127.0.0.1 5002
```
