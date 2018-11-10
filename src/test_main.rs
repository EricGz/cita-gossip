pub mod storage;
pub mod msg;
//pub mod filter;

#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate chrono;

use chrono::prelude::*;
use storage::Node;
use std::collections::HashMap;

//testing codes
fn main() {


 //test function get_list()
    let (changetime, nodelist) = storage::get_list();
    println!("{:?}",changetime );
    for item in nodelist {
         println!("{:?}", item);
    }

    storage::check_lru();

	let my_a_hash : HashMap<String,i64> = storage::get_lru();
	println!("{:?}",my_a_hash);

/*
    let times = Utc::now();
    let stimes :String = times.timestamp_millis().to_string();
    let send_n = Node::new("001","127.0.0.1","001",stimes);
    let mut lmsg = msg::List_msg::new();
    lmsg.send(send_n,true);
    lmsg.receive();
*/
}
/*
	let mut my_hash = HashMap::new();
    let time = Utc::now();
    let utime = time.timestamp_millis();
    my_hash.insert(String::from("message"),utime);

    storage::write_LRU(my_hash);
 */

/*
//test function alive_deal
//test function dead_deal
//test function write_list() 
    let time1 = Local::now();
    let stime1 :String = time1.timestamp_millis().to_string();
    let dead_n = Node::new("001","127.0.0.1","001",stime1);
    dead_deal(dead_n);

    let (changetime, nodelist) = storage::get_list();
    println!("{:?}",changetime );
    for item in nodelist {
         println!("{:?}", item);
    }

    let time2 = Local::now();
    let stime2 :String = time2.timestamp_millis().to_string();
    let alive_n = Node::new("005","127.0.0.5","005",stime2);
    alive_deal(alive_n);

    let (changetime, nodelist) = storage::get_list();
    println!("{:?}",changetime );
    for item in nodelist {
         println!("{:?}", item);
    }
*/