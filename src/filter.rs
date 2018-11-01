//The module adjusts the sending object and controls the periodic sending function.
//
//
//Responsible for cluster maintenance
//
//
//

use::msg;
use::storage::Node;
use::storage;
use rand::Rng;
use std::time::Duration;
use std::thread;


//this function is used for generating random target nodes.
pub fn random_filter() -> Node{
    let list : Vec<Node> = storage::get_list();
    let mut len:u32 = 0;
    for item in list{
        len += 1;
    }
    let random_number = rand::thread_rng().gen_range(1, len+1);
    //if  { }   need to know the local node.
    let random_target = list.get(random_number)
}

//this function is used for converting nodes to IP.
pub fn filter(target:Node) -> iptype{
    let list : Vec<Node> = storage::get_list();
    let des_id = Node.Id;
    let mut flag:bool = false;
    for item in list{
        if item.id == des_id{
            flag = true;
            let ip = item.ip;
        }
    }
    if flag == false {
        println!("No Matched Node!");
    }
    return ip
}

//this function is used for random transmission.
pub fn Spread(){
	//Book update
	//Getting account book time-String
	let random_des = filter::random_filter();
	let version_msg = msg::Gossip_msg::new();
	version_msg.send(random_des，Data/*Data is book*/);
}

//this function is used for unicast.
pub fn Gossip_Send(des:Node){
	//Book update
	//Getting account book time-String
	let version_msg = msg::Short_msg::new();
	version_msg.send(des，Data/*Data is book*/);
}

//this function is used for cluster_maintenance
//by transmiting periodically.
pub fn Cluster_Maintenance(){
   for i in 0..3 {
	
	thread::spawn(move || {
        let alive_msg = msg::HeartBeat_msg::new();
        let op = random_filter();
        alive_msg.send(op,ture);

        storage::alive_check()；
    });
    thread::sleep(Duration::from_secs(1));
	}
}

//周期检测alive消息，若超时做dead处理
//
//修改整顿storage后完成