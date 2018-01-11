extern crate ofborg;
extern crate amqp;
extern crate env_logger;

#[macro_use]
extern crate log;

use std::env;

use amqp::Basic;
use amqp::Session;
use amqp::Table;

use ofborg::config;
use ofborg::worker;
use ofborg::tasks;


fn main() {
    let cfg = config::load_security_advisories(env::args().nth(1).unwrap().as_ref());

    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info");
        env_logger::init().unwrap();
        info!("Defaulting RUST_LOG environment variable to info");
    } else {
        env_logger::init().unwrap();
    }

    println!("Hello, world!");


    let mut session = Session::open_url(&cfg.rabbitmq.as_uri()).unwrap();
    println!("Connected to rabbitmq");
    {
        println!("About to open channel #1");
        let hbchan = session.open_channel(1).unwrap();

        println!("Opened channel #1");

        tasks::heartbeat::start_on_channel(hbchan, "security-advisories-service".to_string());
    }

    let mut channel = session.open_channel(2).unwrap();
    channel.exchange_declare("github-events", "topic", true, true, false, false, false, Table::new()).unwrap();
    println!("Declared exchange");
    let read_queue = channel.queue_declare("security-events", false, true, false, false, false, Table::new()).unwrap();
    println!("Declared queue");
    channel.queue_bind(read_queue.queue.as_ref(), "github-events", "*", false, Table::new()).unwrap();
    println!("Bound to queue");

    channel.basic_prefetch(1).unwrap();
    channel.basic_consume(
        worker::new(tasks::security_advisories::SecurityAdvisoryWorker::new()),
        read_queue.queue.as_ref(),
        "security-advisories-service",
        false,
        false,
        false,
        false,
        Table::new()
    ).unwrap();

    channel.start_consuming();

    println!("Finished consuming?");

    channel.close(200, "Bye").unwrap();
    println!("Closed the channel");
    session.close(200, "Good Bye");
    println!("Closed the session... EOF");
}
