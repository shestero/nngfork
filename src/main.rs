use nng::{Message, Protocol, Socket};

const SERVICE_URL: &str = "tcp://127.0.0.1:10230";

const SERVICE_URL1: &str = "tcp://127.0.0.1:10234";
const SERVICE_URL2: &str = "tcp://127.0.0.1:10235";


fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    println!("Starting server at {SERVICE_URL}");

    let s = Socket::new(Protocol::Rep0)?;
    s.listen(SERVICE_URL)?;

    println!("Child URL #1: {SERVICE_URL1}");
    let c1 = Socket::new(Protocol::Req0)?;
    c1.dial(SERVICE_URL1)?;

    println!("Child URL #2: {SERVICE_URL2}");
    let c2 = Socket::new(Protocol::Req0)?;
    c2.dial(SERVICE_URL2)?;

    loop {
        let req: Message = s.recv()?;
        c1.send(req.clone()).map_err(|(_, err)| err)?;
        c2.send(req.clone()).map_err(|(_, err)| err)?;
        let resp1: Message = c1.recv()?;
        let resp2: Message = c2.recv()?;
        if resp1.to_vec()!=resp2.to_vec() {
            eprintln!(
                "Responses not confirmed! req={:#?}, response1={:#?}, response2={:#?}.",
                req, resp1, resp2
            );
        }
        let _ = s.send(resp1.as_slice()).map_err(|(_, e)| e)?;
    }
    // Ok(())
}
