use dns;
use std::net;

/// A DNS response.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Response
{
    pub answers: Vec<Record>,
    pub nameservers: Vec<Record>,
    pub additional: Vec<Record>,
}

/// Any type of DNS record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record
{
    pub name: String,
    pub class: dns::Class,
    pub ttl: u32,
    pub kind: RecordKind,
}

/// A specific DNS record variant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RecordKind
{
    A(net::Ipv4Addr),
    AAAA(net::Ipv6Addr),
    CNAME(String),
    MX {
        preference: u16,
        exchange: String,
    },
    NS(String),
    SRV {
        priority: u16,
        weight: u16,
        port: u16,
        target: String,
    },
    TXT(String),
    PTR(String),
    /// A record kind that hasn't been implemented by this library yet.
    Unimplemented(Vec<u8>),
}

impl Response
{
    pub fn from_packet(packet: &dns::Packet) -> Self {
        Response {
            answers: packet.answers.iter().map(Record::from_resource_record).collect(),
            nameservers: packet.nameservers.iter().map(Record::from_resource_record).collect(),
            additional: packet.additional.iter().map(Record::from_resource_record).collect(),
        }
    }

    pub fn records(&self) -> ::std::vec::IntoIter<&Record> {
        let records: Vec<_> = vec![&self.answers, &self.nameservers, &self.additional].into_iter().flat_map(|c| c.iter()).collect();
        records.into_iter()
    }

    pub fn is_empty(&self) -> bool {
        self.answers.is_empty() &&
            self.nameservers.is_empty() &&
            self.additional.is_empty()
    }
}

impl Record
{
    fn from_resource_record(rr: &dns::ResourceRecord) -> Self {
        Record {
            name: rr.name.to_string(),
            class: rr.cls,
            ttl: rr.ttl,
            kind: RecordKind::from_rr_data(&rr.data),
        }
    }
}

impl RecordKind
{
    fn from_rr_data(data: &dns::RRData) -> Self {
        use dns::RRData;

        match *data {
            RRData::A(ref addr) => RecordKind::A(addr.clone()),
            RRData::AAAA(ref addr) => RecordKind::AAAA(addr.clone()),
            RRData::CNAME(ref name) => RecordKind::CNAME(name.to_string()),
            RRData::MX { preference, ref exchange } => RecordKind::MX {
                preference,
                exchange: exchange.to_string(),
            },
            RRData::NS(ref name) => RecordKind::NS(name.to_string()),
            RRData::PTR(ref name) => RecordKind::PTR(name.to_string()),
            RRData::SRV { priority, weight, port, ref target } => RecordKind::SRV {
                priority, weight,
                port, target: target.to_string() },
            RRData::TXT(ref txt) => RecordKind::TXT(txt.to_owned()),
            RRData::SOA(..) =>
                RecordKind::Unimplemented("SOA record handling is not implemented".into()),
            RRData::Unknown(data) => RecordKind::Unimplemented(data.to_owned()),
        }
    }
}
