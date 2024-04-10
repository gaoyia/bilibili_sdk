//! bilibili 开放平台 第三方SDK
//! 
//! # 说明
//! 
//! 该项目由个人维护，如果接口有变更，您可以使用github提交pr，或给我发邮件。我会尽可能的维护该项目
//! 
//! 如果你熟悉github和Rust，你应该知道怎么找我
//! 
//! # 使用方法
//! 
//! 参见 examples 文件夹下的两个项目以及测试用例
//! 
//! ## 请求接口部分
//! ```rust
//! pub use bilibili_sdk::{
//!     ResponseData, // http 整个的响应结构体
//!     Data, // http 整个的响应结构体里的data字段
//!     GameInfo, // data里面的game_info字段
//!     WebsocketInfo, // data里面的websocket_info字段
//!     AnchorInfo, // data里面的anchor_info字段
//!     BilibiliSDK, Config // 就是SDK和配置呗
//! };
//! use bilibili_sdk::{};
//! let config: Config = Config {
//!     access_secret_key: "xxxxxxxxxx".to_owned(), // 替换成你自己的
//!     access_key_id: "xxxxxxxxxx".to_owned(),// 替换成你自己的
//!     ..Default::default()
//! };
//! let app_id: u64 = 1234567890123;// 替换成你自己的
//! let code: String = "xxxxxxxxx".to_owned();// 替换成你自己的
//! let sdk = BilibiliSDK::new(config);
//! let game_id = "abc".to_string();
//! async {
//!     let result = sdk.start(code, app_id).await; // 开始接口
//!     let result = sdk.heartbeat(game_id.clone()).await; // 心跳接口
//!     let result = sdk.batch_heartbeat(vec![game_id.clone()]).await; // 批量心跳接口
//!     let result = sdk.end(app_id, game_id.clone()).await; // 结束接口
//! };
//! ````
//! 
//! ## proto 解析
//! ```rust
//! 
//! use bilibili_sdk::{
//!     Proto, // Proto 解析
//!     Operation, // Proto里的operation的枚举
//!     num_2_operation // 将整型转换为枚举的函数，应该用不上
//! };
//! 
//! // 这是解析长链接消息需要的结构体
//! use bilibili_sdk::{
//!     LiveEventData, // webscoket里面的完整结构体
//!     DM, // DM 结构体
//!     SendGift, // 礼物消息的结构体
//!     SuperChat, // 醒目留言消息的结构体
//!     SuperChatDelete, // 醒目留言删除事件消息的结构体
//!     Guard, // 大航海消息的结构体
//!     Like, // 点赞消息的结构体
//!     ComboInfo, // 礼物连击消息的结构体
//!     LiveCmd, // CMD的枚举 根据枚举匹配对应的解析
//!     MatchedData, // 匹配到的数据枚举{"cmd":"xx","data":"就是这里的部分，用枚举表示了"}
//!     msg_parser // 一个解析消息的助手函数
//! };
//! 
//! // 打包心跳包
//! let heartbeat = Proto::pack(None,Operation::Heartbeat,vec![]);
//! 
//! // 打包鉴权包
//! let auth = Proto::pack(None,Operation::Auth,vec![]);
//! 
//! // 解析proto消息
//! let data : Vec<u8> = vec![0, 0, 0, 21, 0, 16, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 104, 101, 108, 108, 111].to_vec();
//! let proto = Proto::unpack(None, data); // String::from_utf8(proto.body) 输出 "hello"
//! ```


use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest;
use md5;
use serde_json::json;
use uuid::Uuid;
use ring::hmac;
mod pack;
mod res_msg;
mod ws_msg;
pub use pack::{Proto, Operation, num_2_operation};
pub use res_msg::{ResponseData, Data, GameInfo, WebsocketInfo, AnchorInfo};
pub use ws_msg::{LiveEventData, DM,SendGift, SuperChat, SuperChatDelete, Guard, Like, ComboInfo,LiveCmd,MatchedData,msg_parser};
pub struct BilibiliSDK {
    access_key_id: String,
    access_secret_key: String,
    base_url: String,
}

#[derive(Default)]
pub struct Config {
    pub access_key_id: String,
    pub access_secret_key: String,
    pub base_url: Option<String>,
}

impl BilibiliSDK  {
    pub fn new(config:Config) -> Self {
        BilibiliSDK {
            access_key_id: config.access_key_id,
            access_secret_key: config.access_secret_key,
            base_url: config.base_url.unwrap_or("https://live-open.biliapi.com".to_string()),
        }
    }
    pub async fn request(&self, path:&str,body:&str) -> reqwest::Result<reqwest::Response> {
        let ts = get_now_timestamp(true).to_string();
        let md5_str = format!("{:x}",md5::compute(body));
        let uuid_str = Uuid::new_v4().to_string();
        let headers:BTreeMap<&str, String> = [
            ("x-bili-accesskeyid", self.access_key_id.to_owned()),
            ("x-bili-content-md5", md5_str),
            ("x-bili-signature-method", "HMAC-SHA256".to_owned()),
            ("x-bili-signature-nonce", uuid_str),
            ("x-bili-signature-version", "1.0".to_owned()),
            ("x-bili-timestamp", ts),
        ].into();
        let authorization = generate_signature(headers.clone(),&self.access_secret_key);
        let client = reqwest::Client::new();
        let mut req = client.post(format!("{}{}", self.base_url, path))
            .header(reqwest::header::ACCEPT, "application/json")
            .header(reqwest::header::CONTENT_TYPE, "application/json");
            for (key, value) in headers.iter()  {
                req = req.header(*key, value);
            }
            req = req.header(reqwest::header::AUTHORIZATION, authorization);
            let res = req.body(body.to_string()).send().await;
        res
    }

    pub async fn start(&self,code: String,app_id: u64)-> reqwest::Result<reqwest::Response>{
        let body = json!({
            "code": code,
            "app_id": app_id,
        });
        let res = self.request("/v2/app/start",body.to_string().as_str()).await;
        res
    }

    pub async fn end(&self,app_id: u64,game_id: String)-> reqwest::Result<reqwest::Response>{
        let body = json!({
            "app_id": app_id,
            "game_id": game_id,
        });
        let res = self.request("/v2/app/end",body.to_string().as_str()).await;
        res
    }

    pub async fn heartbeat(&self,game_id: String)-> reqwest::Result<reqwest::Response>{
        let body = json!({
            "game_id": game_id
        });
        let res = self.request("/v2/app/heartbeat",body.to_string().as_str()).await;
        res
    }
    pub async fn batch_heartbeat(&self,game_ids: Vec<String>)-> reqwest::Result<reqwest::Response>{
        let body = json!({
            "game_ids": game_ids
        });
        let str= &body.to_string();
        println!("批量心跳请求消息：{}",str);
        let res = self.request("/v2/app/batchHeartbeat",str).await;
        res
    }
}

pub fn get_now_timestamp(ms: bool) -> u64 {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    
    // 获取毫秒级时间戳
    if ms {
        return since_epoch.as_secs() * 1000 + since_epoch.subsec_millis() as u64;
    } else {
        return since_epoch.as_secs()
    }
}
pub fn generate_signature(header: BTreeMap<&str,String>,access_secret_key:&str) -> String {
    let mut header_str = String::new();
    let mut i = 0;
    for (key, value) in header.iter() {
        i+=1;
        if key.starts_with("x-bili-") {
            header_str.push_str(key);
            header_str.push(':');
            header_str.push_str(value);
            // 如果不是最后一个键值对，则添加换行符
            if i < header.len() {
                header_str.push('\n');
            }
        }
    }
    let key_sequence = hmac::Key::new(hmac::HMAC_SHA256, access_secret_key.as_bytes());
    let authorization = hex::encode(hmac::sign(&key_sequence, header_str.as_bytes()));
    authorization
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign() {
        let demo_access_secret_key = "JzOzZfSHeYYnAMZ";
        let header = [
            ("x-bili-accesskeyid", "xxxx".to_owned()),
            ("x-bili-content-md5", "fa6837e35b2f591865b288dfd859ce9d".to_owned()),
            ("x-bili-signature-method", "HMAC-SHA256".to_owned()),
            ("x-bili-signature-nonce", "ad184c09-095f-91c3-0849-230dd3744045".to_owned()),
            ("x-bili-signature-version", "1.0".to_owned()),
            ("x-bili-timestamp", "1624594467".to_owned()),
        ];
        let btree_map = BTreeMap::from(header);
        let result = generate_signature(btree_map, demo_access_secret_key);
        assert_eq!(result, "a81c50234b6bbf15bc56e387ee4f19c6f871af2f70b837dc56db16517d4a341f".to_owned());
    }
}
