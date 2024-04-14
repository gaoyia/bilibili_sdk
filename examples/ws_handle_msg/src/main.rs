use bilibili_sdk::{BilibiliSDK, Config, Guard, Like, LiveEventData, Proto, ResponseData, SendGift, SuperChat, SuperChatDelete, DM};
use futures_util::{SinkExt, StreamExt as _};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let config: Config = Config {
        access_key_id: "xxxxxx".to_owned(),// 替换成你自己的
        access_secret_key: "xxxxxx".to_owned(), // 替换成你自己的
        ..Default::default()
    };
    let app_id: u64 = 1234567892345;// 替换成你自己的
    let code: String = "xxxxxxx".to_owned();// 替换成你自己的
    let sdk = BilibiliSDK::new(config);
    println!("{:?}",sdk);
    // 第一步发起开始请求，获取房间信息
    let result = sdk.start(code, app_id).await;
    if result.is_ok() {
        let res = result.unwrap().json::<ResponseData>().await.unwrap();
        println!("res{:?}",res);
        if res.code == 0 {
            // 如果没有错误码，返回成功
            let game_id = &res.data.clone().unwrap().game_info.unwrap().game_id;
            let auth_body = &res.data.clone().unwrap().websocket_info.unwrap().auth_body;
            let url = &res.data.clone().unwrap().websocket_info.unwrap().wss_link[0];
            let atuh_proto = Proto::pack(None,bilibili_sdk::Operation::Auth,auth_body.clone().into_bytes());
            let heartbeat = Proto::pack(None,bilibili_sdk::Operation::Heartbeat,vec![]).unwrap();
            let (mut ws_stream, _) = connect_async(url)
                .await
                .expect("Failed to connect");
            // 发送auth信息
            let msg = Message::Binary(atuh_proto.unwrap());
            // 如果需要，可以回复 pong 消息
            if let Err(_e) = ws_stream.send(msg).await {
            }
            loop {
                tokio::select! {
                    // 发送心跳消息
                    _ = tokio::time::sleep(Duration::from_secs(20)) => {
                        let _ = sdk.heartbeat(game_id.to_string()); // 这个我估计应该在游戏客户端执行
                        if let Err(e) = ws_stream.send(Message::Binary(heartbeat.clone())).await {
                            println!("ws-心跳发送失败");
                        }
                    },
                    // 接收并处理消息
                    Some(msg) = ws_stream.next() => {
                        match msg.unwrap() {
                            Message::Text(text) => {
                                println!("Received text message: {}", text);
                                // 在这里处理文本消息
                            },
                            Message::Binary(bin) => {
                                let msg = Proto::unpack(None,bin);
                                let msg = msg.unwrap();
                                println!("{:#?}",&msg);
                                match msg.operation {
                                    bilibili_sdk::Operation::HeartbeatReply => {
                                        println!("心跳回复")
                                    },
                                    bilibili_sdk::Operation::SendSmsReply => {
                                        println!("收到消息");
                                        let json_str = String::from_utf8(msg.body).unwrap();
                                        let data: LiveEventData = serde_json::from_str(&json_str).unwrap();
                                        match data.cmd {
                                            bilibili_sdk::LiveCmd::LiveOpenPlatformDm => {
                                                let dm: DM = serde_json::from_value(data.data).unwrap();
                                                print!("收到弹幕消息：{:?}",dm);
                                            },
                                            bilibili_sdk::LiveCmd::LiveOpenPlatformSendGift => {
                                                let gift: SendGift = serde_json::from_value(data.data).unwrap();
                                                print!("收到礼物消息：{:?}",gift);
                                            },
                                            bilibili_sdk::LiveCmd::LiveOpenPlatformSuperChat => {
                                                let sc: SuperChat = serde_json::from_value(data.data).unwrap();
                                                print!("收到超级聊天消息：{:?}",sc);
                                            },
                                            bilibili_sdk::LiveCmd::LiveOpenPlatformSuperChatDelete => {
                                                let scd: SuperChatDelete = serde_json::from_value(data.data).unwrap();
                                                print!("收到超级聊天删除消息：{:?}",scd);
                                            },
                                            bilibili_sdk::LiveCmd::LiveOpenPlatformGuard => {
                                                let guard: Guard = serde_json::from_value(data.data).unwrap();
                                                print!("收到guard消息：{:?}",guard);
                                            },
                                            bilibili_sdk::LiveCmd::LiveOpenPlatformLike => {
                                                let like: Like = serde_json::from_value(data.data).unwrap();
                                                print!("收到like消息：{:?}",like);
                                            },
                                        }
                                    },
                                    bilibili_sdk::Operation::AuthReply => {
                                        println!("权限消息的回复")
                                    },
                                    _ => {

                                    }
                                }
                                // 在这里处理二进制消息
                            },
                            Message::Ping(data) | Message::Pong(data) => {
                                // 如果需要，可以回复 pong 消息
                                if let Err(_e) = ws_stream.send(Message::Pong(data)).await {
                                }
                            },
                            Message::Close(close_frame) => {
                                println!("Received close frame: {:?}", close_frame);
                                // 可能需要关闭连接或进行其他清理操作
                                break;
                            },
                            _ => {
                                println!("_");
                                break;
                            },
                        }
                    },
                }
            }
        }
    }
}




















