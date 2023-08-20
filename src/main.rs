use error_chain::error_chain;
use std::io::Read;

error_chain

fn main() -> Result<()> {
    let mut res = reqwestr::blocking::get("http://httpbin.org/get")?;
    
}