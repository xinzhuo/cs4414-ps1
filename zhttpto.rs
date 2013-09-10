//
// zhttpto.rs
//
// University of Virginia - cs4414 Fall 2013
// Weilin Xu and David Evans
// Version 0.1

extern mod extra;

use extra::uv;
use extra::{net_ip, net_tcp};
use std::str;
use std::io;
use std::result;

static BACKLOG: uint = 5;
static PORT:    uint = 4414;
static IPV4_LOOPBACK: &'static str = "127.0.0.1";
static mut visitor_count:   int=0;

fn new_connection_callback(new_conn :net_tcp::TcpNewConnection, _killch: std::comm::SharedChan<Option<extra::net_tcp::TcpErrData>>)
{
    do spawn {
        let accept_result = extra::net_tcp::accept(new_conn);
        match accept_result {
            Err(err) => {
               println(fmt!("Connection error: %?", err));
            },  
            Ok(sock) => {
                let peer_addr: ~str = net_ip::format_addr(&sock.get_peer_addr());
                println(fmt!("Received connection from: %s", peer_addr));
                
                let read_result = net_tcp::read(&sock, 0u);
                match read_result {
                    Err(err) => {
                        println(fmt!("Receive error: %?", err));
                    },
                    Ok(bytes) => {
                        unsafe{
                            visitor_count += 1;
                        }

                        let request_str = str::from_bytes(bytes.slice(0, bytes.len() - 1));

                        let mut counter = 4;
                        let mut position = 0;
                        while position == 0{
                            if request_str.slice(counter, counter+1) == " " {
                                position = counter;
                            }
                            else{
                                counter += 1;
                            }
                        }

                        unsafe{
                            println(fmt!("Number of visitors: %i", visitor_count));
                        }
                        println(fmt!("Request received:\n%s", request_str));

                        let addr = request_str.slice(5, position);
                        println(fmt!("%s", addr));
                        if addr.len() != 0{
                            let maybe_test_reader: Result<@Reader, ~str> = io::file_reader(~PosixPath(addr));

                            let test_reader: @Reader = result::unwrap(maybe_test_reader);
                            let mut bytes: ~[u8] = ~[];
                            loop {
                                let byte: int = test_reader.read_byte();
                                debug!("%d", byte);
                                if test_reader.eof() { break }
                                (&mut bytes).push(byte as u8);
                            }
                            let maybe_success: ~str = str::from_bytes(bytes);
                            //println(fmt!("%s", maybe_success));

                            let response: ~str = maybe_success.to_str();

                            net_tcp::write(&sock, response.as_bytes_with_null_consume());
                        } 
                        else{
                            let response: ~str = ~
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                             <doctype !html><html><head><title>Hello, Rust!</title>
                             <style>body { background-color: #111; color: #FFEEAA }
                                    h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                             </style></head>
                             <body>
                             <h1>Greetings, Rusty!</h1>
                             </body></html>\r\n";

                            net_tcp::write(&sock, response.as_bytes_with_null_consume());
                        }
                    },
                };
            }
        }
    };
}

fn main() {
    net_tcp::listen(net_ip::v4::parse_addr(IPV4_LOOPBACK), PORT, BACKLOG,
                    &uv::global_loop::get(),
                    |_chan| { println(fmt!("Listening on tcp port %u ...", PORT)); },
                    new_connection_callback);
}
