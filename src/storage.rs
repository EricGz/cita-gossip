#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate chrono;

use chrono::prelude::*;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
#[derive(Serialize)]
#[derive(Debug)]
pub struct Node {
    id: String,
    ip: String,
    hostname: String,
    timestamp: String,
}

#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Serialize)]
pub struct NodeConfig
{
    changetime: String,
    node: Vec<Node>
}
impl Node{
    fn new(id: String, ip: String, hostname: String, timestamp: String) -> Node {
        Node {id,ip,hostname,timestamp,}
    }
    fn default() -> Node{
        Node{
            id: String::from("default id"),
            ip: String::from("default ip"),
            hostname: String::from("default hostname"),
            timestamp: String::from("default timestamp"),
        }
    }
}

// this function get a filename, return a vector of Nodes from the file.
pub fn get_list() -> (String , Vec<Node>) {

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
    let schangetime :String = changetime.timestamp_millis().to_string();
    let config = NodeConfig{
        changetime: schangetime,
        node: nodelist,
    };
    let toml = toml::to_string(&config).unwrap();
    file.write(toml.as_bytes()).expect("file not write");

    //
    println!("Send List_msg to tell others the change.");
}

// this function get a Node and a filename, change the Node.
pub fn alive_deal(alive_node: Node) {

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

// this function get a Node and a filename, change the Node's timestamp in the file into "***".
pub fn dead_deal(dead_node: Node) {

    let (_changetime,nodelist) = get_list();
    let mut newlist: Vec<Node> = Vec::new();
    
    {
    let mut flag :bool = false;
    for item in nodelist {
        if item.id == dead_node.id {
            flag = true;
            continue;
        }
        else {
            newlist.push(item);
        }
    }
    if flag == false {
            println!("Dead Node not matched!");;
        }
    }

    /* write the new Vec<Node> into file */
    write_list(newlist);
}

// this function check out which node is dead because of overtime.
pub fn alive_check(){

    let (changetime,nodelist) = get_list();
    let mut newlist: Vec<Node> = Vec::new();
    
    let changetime :i64 = changetime.trim().parse().unwrap();
    for item in nodelist {
        let nodetime :i64 = item.timestamp.trim().parse().unwrap();
        let time_diff = changetime - nodetime;
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