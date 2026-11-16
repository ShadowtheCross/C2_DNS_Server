mod dns;

use dns::dns_communication;

use std::io;



fn main() -> io::Result<()> {
    dns_communication("127.0.0.1", 5353)
}