extern crate amqp;
extern crate env_logger;

use ofborg::message::security_advisories;
use ofborg::worker;
use amqp::protocol::basic::{Deliver,BasicProperties};


pub struct SecurityAdvisoryWorker {
}

impl SecurityAdvisoryWorker {
    pub fn new() -> Self {
        Self { }
    }
}

impl worker::SimpleWorker for SecurityAdvisoryWorker {
    type J = security_advisories::SecurityAdvisoryJob;

    fn msg_to_job(&self, _: &Deliver, _: &BasicProperties,
                  body: &Vec<u8>) -> Result<Self::J, String> {
        println!("lmao I got a job?");
        return match security_advisories::from(body) {
            Ok(e) => { Ok(e) }
            Err(e) => {
                println!("{:?}", String::from_utf8(body.clone()));
                panic!("{:?}", e);
            }
        }
    }

    fn consumer(&self, job: &Self::J) -> worker::Actions {
        vec![worker::Action::NackRequeue]
    }
}
