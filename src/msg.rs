//The module defines data structures, 
//which declares message types, 
//and encapsulates logical operations.
//
//
//
//
use::storage;
//use::filter;
use::storage::Node;
use chrono::prelude::*;
use std::collections::HashMap;

//The four structure are used for message transmission.
#[derive(Debug)]
pub struct ListMsg{
    src:Node,
    des:Node,
    is_request:bool,
    list:Vec<Node>,
}
#[derive(Debug)]
pub struct GossipPushMsg{
    src:Node,
    des:Node,
    timestamp:i64,
    data:String,
}
#[derive(Debug)]
pub struct GossipPullrqMsg{
    src:Node,
    des:Node,
    versioninfo: Versioninfo,
}
#[derive(Debug)]
pub struct GossipPullrpMsg{
    src:Node,
    des:Node,
    data:String,
}
#[derive(Debug)]
pub struct HeartBeatMsg{
    src:Node,
    des:Node,
    timestamp:i64,
}
#[derive(Debug)]
pub struct Versioninfo{
	versioncode : String,
	height : u32,
}
impl Versioninfo {
	pub fn new(_versioncode: &str,_height:u32) -> Self {
		Versioninfo{
			versioncode : _versioncode.to_string(),
			height : _height, 
		}
	}
}


//Impl for ListMsg:new()-initialization,send() & receive()
impl ListMsg{
    pub fn send_new() -> Self{ 
        ListMsg{
            src : Node::default(),
            des : Node::default(),
            is_request: true,
            list: Vec::new(),
        }
    }
    pub fn receive_new(src:Node,des:Node,is_request:bool,list:Vec<Node>) -> Self{ 
        ListMsg{
            src,
            des,
            is_request,
            list,
        }
    }
    pub fn send(&mut self,_des:Node,_is_request:bool){//the destination is provided by Mod-filter
        self.des = _des;
        self.is_request = _is_request;
        if _is_request == false {
            let (_time,list) = storage::get_list();
        	self.list = list;
        }
        println!("#ListMsg#:{:?} is sent.",self);
//        des_ip = filter::filter(_des);
//        super::comm::ListSend();    //use comm mod to transmit.
    }
    pub fn receive(self){
        //change local file through is_request
        if self.is_request == true {
        	let mut re_msg = ListMsg::send_new();
        	re_msg.send(self.src,false);
        }
/*        else {
        	//compare
        	storage::write_list(rec_msg.List);
        	let random_node = filter::random_filter();
        	let random_msg = ListMsg::new();
        	random_msg.send(random_node,false);
        }
*/
    }
}
//Impl for GossipPushMsg:new()-initialization,send() & receive()
impl GossipPushMsg{
    pub fn send_new() -> Self{
        GossipPushMsg{
            src : Node::default(),
            des : Node::default(),
            timestamp : 0,
            data : String::from("Default Data"),
        }
    }
     pub fn receive_new(src:Node,des:Node,timestamp:i64,data:String) -> Self{ 
        GossipPushMsg{
            src,
            des,
            timestamp,
            data,
        }
    }
    pub fn send(&mut self,_des:Node,_data:String){//the destination is provided by Mod-filter
        self.des = _des;
        self.data = _data;
        let utctime = Utc::now();
        let iutctime = utctime.timestamp_millis();
        self.timestamp = iutctime;
//      des_ip = filter::filter(_des);
//      super::comm::GossipSend(); 
        println!("#GossipPushMsg#:{:?} is sent.",self);
    }
    pub fn receive(self){

        let lru : HashMap<String,i64> = storage::get_lru();
        if !lru.contains_key(&self.data)  {
            
        }
    }
}
//Impl for GossipPullrqMsg:new()-initialization,send() & receive()
impl GossipPullrqMsg{
    pub fn send_new() -> Self{
        GossipPullrqMsg{
            src : Node::default(),
            des : Node::default(),
            versioninfo : Versioninfo::new("****",1),
        }
    }
    pub fn receive_new(src:Node,des:Node,versioninfo:Versioninfo) -> Self{ 
        GossipPullrqMsg{
            src,
            des,
            versioninfo,
        }
    }
    pub fn send(&mut self,_des:Node,_versioninfo:Versioninfo){//the destination is provided by Mod-filter
        self.des = _des;
        self.versioninfo = _versioninfo;
//      des_ip = filter::filter(_des);
//      super::comm::GossipSend(); 
        println!("#GossipPullrqMsg#:{:?} is sent.",self);
    }
    pub fn receive(self){

        //judge by versioninfo.
            
    }
}

//Impl for GossipPullrpMsg:new()-initialization,send() & receive()
impl GossipPullrpMsg{
    pub fn send_new() -> Self{
        GossipPullrpMsg{
            src : Node::default(),
            des : Node::default(),
            data : String::from("Default Data"),
        }
    }
     pub fn receive_new(src:Node,des:Node,data:String) -> Self{ 
        GossipPullrpMsg{
            src,
            des,
            data,
        }
    }
    pub fn send(&mut self,_des:Node,_data:String){//the destination is provided by Mod-filter
        self.des = _des;
        self.data = _data;
//      des_ip = filter::filter(_des);
//      super::comm::GossipSend(); 
        println!("#GossipPullrpMsg#:{:?} is sent.",self);
    }
    pub fn receive(self){

        //send data to the super.
    
    }
}

//Impl for HeartBeatMsg:new()-initialization,send() & receive()
impl HeartBeatMsg{
    pub fn send_new() -> Self{
        HeartBeatMsg{
            src : Node::default(),
            des : Node::default(),
            timestamp: 0,
        }
    }
     pub fn receive_new(src:Node,des:Node,timestamp:i64) -> Self{ 
        HeartBeatMsg{
            src,
            des,
            timestamp,
        }
    }
    pub fn send(&mut self,_des:Node){//the destination is provided by Mod-filter
        self.des = _des;
        let utctime = Utc::now();
        let iutctime = utctime.timestamp_millis();
        self.timestamp = iutctime;
//        super::comm::HeartBeatSend();  
    }
    pub fn receive(self){
        //change neighbor
        storage::alive_neighbor(self.src);
    }
}