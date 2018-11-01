//The module defines data structures, 
//which declares message types, 
//and encapsulates logical operations.
//
//
//
//
use::storage;
use::filter;
use::storage::Node;

//The four structure are used for message transmission.
pub struct Short_msg{
    src:Node,
    des:Node,
    data:String,
    flag:bool,
}
pub struct List_msg{
    src:Node,
    des:Node,
    is_request:bool,
    List:Vec<Node>,
}
pub struct Gossip_msg{
    src:Node,
    des:Node,
    data:String,
}
pub struct HeartBeat_msg{
    src:Node,
    des:Node,
    is_alive:bool,
}

//Impl for Short_msg:new()-initialization, send() & receive()
// **Short_msg has been abandoned by now. Maybe it will be used for error retransmission
impl Short_msg{
    pub fn new( ) -> Self{
        Short_msg{
            src : Node::default(),
            des : Node::default(),
            data: String::from(""),
            flag: false,
        }
    }
    pub fn send(&self,target:&Node){//the destination is provided by Mod-filter
        self.des = target;
        self.flag = false;
        des_ip = filter::filter(target);
        super::comm::ShortSend();    
    }
    pub fn receive(&self){
        if self.flag == false {
            let rec_emp_msg = Short_msg{
                src : self.des,
                des : self.src,
                flag : true,
            };
            des_ip = filter::filter(rec_emp_msg.des);
            super::comm::ShortSend();    
        }
    }
}

//Impl for List_msg:new()-initialization,send() & receive()
impl List_msg{
    pub fn new() -> Self{ 
        List_msg{
            src : Node::default(),
            des : Node::default(),
            is_request: true,
            List:Vec::new(),
        }
    }
    pub fn send(&mut self,target:&Node,if_request:bool){//the destination is provided by Mod-filter
        self.des = target;
        self.is_request = if_request;
        if if_request == false {
        	self.List = storage::get_list();
        }
        des_ip = filter::filter(target);
        super::comm::ListSend();    //Unfinished
    }
    pub fn receive(&self){
        //change local file through is_request
        let rec_msg = List_msg::new();
        if rec_msg.is_request == ture  {
        	let alivelist = storage::get_list();
        	let re_msg = List_msg::new();
        	re_msg.send(rec_msg.src,false);
        }
        else {
        	//compare
        	storage::write_list(rec_msg.List);
        	let random_node = filter::random_filter();
        	let random_msg = List_msg::new();
        	random_msg.send(random_node,false);
        }
    }
}

//Impl for Gossip_msg:new()-initialization,send() & receive()
impl Gossip_msg{
    pub fn new() -> Self{
        Gossip_msg{
            src : Node::default(),
            des : Node::default(),
            data: String::from(""),
        }
    }
    pub fn send(&mut self,target:&Node,Data:String){//the destination is provided by Mod-filter
        self.des = target;
        self.data = Data;
        des_ip = filter::filter(target);
        super::comm::GossipSend(); 
    }
    pub fn receive(&self){
        //change local file
    }
}

//Impl for HeartBeat_msg:new()-initialization,send() & receive()
impl HeartBeat_msg{
    pub fn new() -> Self{
        HeartBeat_msg{
            src : Node::default(),
            des : Node::default(),
            is_alive: true,
        }
    }
    pub fn send(&mut self,target:&Node,if_alive:bool){//the destination is provided by Mod-filter
        self.des = target;
        self.is_alive = if_alive;
        des_ip = filter::filter(target);
        super::comm::HeartBeatSend();  
    }
    pub fn receive(&self){
        //change local file through is_alive
        let rec_msg = HeartBeat_msg::new();
        if rec_msg.is_alive == ture {
        	storage::alive_deal(rec_msg.src);
        }
        else {
        	storage::dead_deal(rec_msg.src);
        }
    }
}