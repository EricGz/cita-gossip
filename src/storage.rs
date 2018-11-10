use toml;
//use serde_derive;
use chrono::prelude::*;

use std::fs::File;
use std::io::prelude::*;

//use std::collections::hash_map::DefaultHasher;
//use std::hash::{Hash, Hasher};
use std::collections::HashMap;

#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
pub struct Node {
    id: String,
    ip: String,
    hostname: String,
    timestamp: i64,
}

#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Serialize)]
pub struct NodeConfig
{
    changetime: i64,
    node: Vec<Node>,
}
impl Node{
    pub fn new(_id: &str, _ip: &str, _hostname: &str, _timestamp: i64) -> Node {
        Node {
            id:_id.to_string(),
            ip:_ip.to_string(),
            hostname:_hostname.to_string(),
            timestamp:_timestamp,
        }
    }
    pub fn default() -> Node{
        Node{
            id: String::from("default id"),
            ip: String::from("default ip"),
            hostname: String::from("default hostname"),
            timestamp: 0,
        }
    }
}

// this function get a filename, return a vector of Nodes from the file.
pub fn get_list() -> (i64 , Vec<Node>) {

    let file_path = "data/AliveList.toml";
    let mut file = File::open(file_path).expect("file not opened");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let config: NodeConfig = toml::from_str(&contents).unwrap();
    /*println!("{}", config.changetime);
    let nodelist : Vec<Node> = config.node;
    for item in nodelist {
         println!("{:?}", item);
    };*/
    (config.changetime , config.node)

}


// this function get a vector of Nodes and a filename, write this vector into the file.
pub fn write_list(nodelist:Vec<Node>) {

    let file_path = "data/AliveList.toml";
    let mut file = File::create(file_path).expect("file not created");

    let changetime = Utc::now();
    let ichangetime = changetime.timestamp_millis();
    let config = NodeConfig{
        changetime: ichangetime,
        node: nodelist,
    };
    let toml = toml::to_string(&config).unwrap();
    file.write(toml.as_bytes()).expect("file not write");

    //
    println!("Send List_msg to tell others the change.");
}
// this function get a filename, return a vector of Nodes from the file.
pub fn get_neighbor() -> (i64 , Vec<Node>) {

    let file_path = "data/Neighbor.toml";
    let mut file = File::open(file_path).expect("file not opened");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let config: NodeConfig = toml::from_str(&contents).unwrap();
    /*println!("{}", config.changetime);
    let nodelist : Vec<Node> = config.node;
    for item in nodelist {
         println!("{:?}", item);
    };*/
    (config.changetime , config.node)

}


// this function get a vector of Nodes and a filename, write this vector into the file.
pub fn write_neighbor(nodelist:Vec<Node>) {

    let file_path = "data/Neighbor.toml";
    let mut file = File::create(file_path).expect("file not created");

    let changetime = Utc::now();
    let ichangetime = changetime.timestamp_millis();
    let config = NodeConfig{
        changetime: ichangetime,
        node: nodelist,
    };
    let toml = toml::to_string(&config).unwrap();
    file.write(toml.as_bytes()).expect("file not write");

    //
    println!("Send List_msg to tell others the change.");
}

// this function get a Node , change the neighbor.
pub fn alive_neighbor(alive_node: Node) {

    let (_changetime,nodelist) = get_list();
    let mut newlist: Vec<Node> = Vec::new();
    
    {
    let new_node = Node{
                id : alive_node.id.clone(),
                ip : alive_node.ip.clone(),
                hostname : alive_node.hostname.clone(),
                timestamp : alive_node.timestamp.clone(),
            };
    for item in nodelist {
        
        if item.id == alive_node.id {
            continue;
        }
        else {
            newlist.push(item);
        }
    }
    newlist.push(new_node);
    }

    /* write the new Vec<Node> into file */
    write_list(newlist);
}

// this function check out which node is dead because of overtime.
pub fn alive_check(){

    let (_changetime,nodelist) = get_list();
    let mut newlist: Vec<Node> = Vec::new();
    
    for item in nodelist {
        let nodetime = item.timestamp;
        let time_diff = _changetime - nodetime;
        if time_diff > 200 {      //the number 200 should have a right num.
            continue;
        }
        else {
            newlist.push(item);
        }
    }

    /* write the new Vec<Node> into file */
    write_list(newlist);
 } 
 // this function check out which node is dead because of overtime.
pub fn neighbor_check(){

    let (_changetime,nodelist) = get_neighbor();
    let mut newlist: Vec<Node> = Vec::new();
    
    for item in nodelist {
        let nodetime = item.timestamp;
        let time_diff = _changetime - nodetime;
        if time_diff > 200 {      //the number 200 should have a right num.
            continue;
        }
        else {
            newlist.push(item);
        }
    }

    /* write the new Vec<Node> into file */
    write_neighbor(newlist);
 } 

 pub fn get_lru() -> HashMap<String,i64> {

    let file_path = "data/DataHash.toml";
    let mut file = File::open(file_path).expect("file not opened");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let map: HashMap<String,i64> = toml::from_str(&contents).unwrap();
    map

}

 // this function gets a HashMap, and writes that into the file.
 pub fn write_lru(hashmap:HashMap<String,i64>) {

    let file_path = "data/DataHash.toml";
    let mut file = File::create(file_path).expect("file not created");


    let toml = toml::to_string(&hashmap).unwrap();
    file.write(toml.as_bytes()).expect("file not write");

}

 // this function check out which msg is 'timeout' because of overtime.
 pub fn check_lru() {

    let mut lru : HashMap<String,i64> = get_lru();

    let time = Utc::now();
    let utime = time.timestamp_millis();

    lru.retain(| _, &mut v|  v - utime < 3000);

    write_lru(lru);

}
