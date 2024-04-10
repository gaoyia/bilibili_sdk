use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ProtoError<'a>(&'a str);

impl fmt::Display for ProtoError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self)
    }
}

impl Error for ProtoError<'_> {}

#[derive(Debug)]
pub struct ProtoConfig {
    pub header_len:Option<u16>, //配置项-固定
    pub max_body:Option<u32>, // 配置项
}

#[derive(Debug)]
pub struct Proto {
    pub packet_len:u32 , // 由数据决定
    pub version:u16, // 由数据决定
    pub operation:Operation, // 由数据决定 Operation：消息的类型：
    pub sequence:u32, // 保留字段，可以忽略
    pub body:Vec<u8>, // 由数据决定
}
#[derive(Debug)]
pub enum  Operation {
    Heartbeat = 2, // 客户端发送的心跳包(30秒发送一次)
    HeartbeatReply = 3, // 服务器收到心跳包的回复
    SendSmsReply = 5, // 服务器推送的弹幕消息包
    Auth = 7, // 客户端发送的鉴权包(客户端发送的第一个包)
    AuthReply = 8, // 服务器收到鉴权包后的回复
}

impl Operation {
    // 为每个操作类型关联一个整数值
    fn value(&self) -> u32 {
        match self {
            Operation::Heartbeat => 2,
            Operation::HeartbeatReply => 3,
            Operation::SendSmsReply => 5,
            Operation::Auth => 7,
            Operation::AuthReply => 8,
        }
    }
}
pub fn num_2_operation(num: u32)-> Operation {
    match num {
        2 => Operation::Heartbeat,
        3 => Operation::HeartbeatReply,
        5 => Operation::SendSmsReply,
        7 => Operation::Auth,
        8 => Operation::AuthReply,
        _ => todo!()
    }
}
impl Proto {
    pub fn pack(config: Option<ProtoConfig>,operation: Operation,body: Vec<u8>) -> Result<Vec<u8>,String> {
        let (config_header_len,config_max_body) = match config {
            Some(config) => (config.header_len.unwrap_or(16),config.max_body.unwrap_or(2048)),
            None => (16,2048),
        };
        let packet_len = (body.len() + config_header_len as usize) as u32;
        if packet_len > config_max_body {
            return Err("超出最大长度限制".to_string());
        }
        let version:u16 = 0;
        let sequence:u32 = 0;
        let mut buffer:Vec<u8> = vec![0; packet_len as usize];
        buffer[0..4].copy_from_slice(&packet_len.to_be_bytes());
        buffer[4..6].copy_from_slice(&config_header_len.to_be_bytes());
        buffer[6..8].copy_from_slice(&version.to_be_bytes());
        buffer[8..12].copy_from_slice(&operation.value().to_be_bytes());
        buffer[12..16].copy_from_slice(&sequence.to_be_bytes());
        buffer[16..].copy_from_slice(&body);
        return Ok(buffer);
    }
    pub fn unpack(config: Option<ProtoConfig>, data:Vec<u8>) -> Result<Proto, String> {
        let (config_header_len,config_max_body) = match config {
            Some(config) => (config.header_len.unwrap_or(16),config.max_body.unwrap_or(2048)),
            None => (16,2048),
        };
        let data_len:u32 = data.len() as u32;
        if data_len  < (config_header_len as u32)  {
            // 数据字节数小于必须的头字节数
            return Err("data corruption".to_string());
        }
        let packet_len:u32 = u32::from_be_bytes(data[0..4].try_into().map_err(|_| "bytes length Error")?);
        let header_len:u16 = u16::from_be_bytes(data[4..6].try_into().map_err(|_| "bytes length Error")?);
        let version:u16 = u16::from_be_bytes(data[6..8].try_into().map_err(|_| "bytes length Error")?);
        let operation:u32 = u32::from_be_bytes(data[8..12].try_into().map_err(|_| "bytes length Error")?);
        let sequence:u32 = u32::from_be_bytes(data[12..16].try_into().map_err(|_| "bytes length Error")?);
        let body:Vec<u8> = data[16..].to_vec();
        let body_len:u32 = packet_len - (header_len as u32);

        if header_len != config_header_len {
            // 数据提供的包头的长度和配置的头长度不一致 依据文档目前固定位16
            return Err(format!("The length of the packet header provided by the data is different from that configured. now header_len: {}", header_len));
        }

        if body_len + header_len as u32 != packet_len  {
            // body字节数加body字节数不等于总长度
            return Err(format!("body bytes plus body bytes does not equal the total length. now packet_len: {}.header_len:{}.body_len:{}", packet_len, header_len, body_len));
        }
        
        if packet_len > config_max_body {
            // 数据损坏：包长度超出最大长度限制
            return Err("data corruption: The packet length exceeds the maximum length limit.".to_string());
        }
        
        if version == 0 {
            return Ok(Proto {
                packet_len, // 总长度
                version:0,
                operation:num_2_operation(operation),
                sequence, // 忽略
                body:body,
            })
        } else if version == 2 {
            return Err(format!("Error: 群里的大佬说：脱裤子放屁。"));
        } else {
            return Err(format!("version Error"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_heartbeat() {
        // 打包心跳包
        let heartbeat = Proto::pack(None,Operation::Heartbeat,vec![]);
        if let Ok(heartbeat) = heartbeat {
            assert_eq!(vec![0, 0, 0, 16, 0, 16, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0], heartbeat);
        } else {
            panic!("{}", heartbeat.unwrap_err());
        }
    }
    #[test]
    fn pack_auth() {
        // 打包鉴权包
        let auth = Proto::pack(None,Operation::Auth,vec![]);
        if let Ok(auth) = auth {
            assert_eq!(vec![0, 0, 0, 16, 0, 16, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0], auth);
        } else {
            panic!("{}", auth.unwrap_err());
        }
    }

    #[test]
    fn unpack() {
        let data : Vec<u8> = vec![0, 0, 0, 21, 0, 16, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 104, 101, 108, 108, 111].to_vec();
        let proto = Proto::unpack(None, data);
        // 解析数据，如果解析成功返回proto的引用
        let res = match proto {
            Ok(proto) => proto,
            Err(e) => panic!("{}", e),
        };
        // 使用 from_utf8 方法将 Vec<u8> 转换为 String
        let body_string = match String::from_utf8(res.body) {
            Ok(string) => string,
            Err(e) => format!("Conversion error: {}", e),
        };
        assert_eq!(body_string,"hello");
    }
}