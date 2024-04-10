bilibili 开放平台 第三方SDK

# 说明

该项目由个人维护，如果接口有变更，您可以使用github提交pr，或给我发邮件。我会尽可能的维护该项目

如果你熟悉github和Rust，你应该知道怎么找我

# 使用方法

参见 examples 文件夹下的两个项目以及测试用例

## 请求接口部分
```rust
pub use bilibili_sdk::{
    ResponseData, // http 整个的响应结构体
    Data, // http 整个的响应结构体里的data字段
    GameInfo, // data里面的game_info字段
    WebsocketInfo, // data里面的websocket_info字段
    AnchorInfo, // data里面的anchor_info字段
    BilibiliSDK, Config // 就是SDK和配置呗
};
use bilibili_sdk::{};
let config: Config = Config {
    access_secret_key: "xxxxxxxxxx".to_owned(), // 替换成你自己的
    access_key_id: "xxxxxxxxxx".to_owned(),// 替换成你自己的
    ..Default::default()
};
let app_id: u64 = 1234567890123;// 替换成你自己的
let code: String = "xxxxxxxxx".to_owned();// 替换成你自己的
let sdk = BilibiliSDK::new(config);
let game_id = "abc".to_string();
async {
    let result = sdk.start(code, app_id).await; // 开始接口
    let result = sdk.heartbeat(game_id.clone()).await; // 心跳接口
    let result = sdk.batch_heartbeat(vec![game_id.clone()]).await; // 批量心跳接口
    let result = sdk.end(app_id, game_id.clone()).await; // 结束接口
};
````

## proto 解析
```rust

use bilibili_sdk::{
    Proto, // Proto 解析
    Operation, // Proto里的operation的枚举
    num_2_operation // 将整型转换为枚举的函数，应该用不上
};

// 这是解析长链接消息需要的结构体
use bilibili_sdk::{
    LiveEventData, // webscoket里面的完整结构体
    DM, // DM 结构体
    SendGift, // 礼物消息的结构体
    SuperChat, // 醒目留言消息的结构体
    SuperChatDelete, // 醒目留言删除事件消息的结构体
    Guard, // 大航海消息的结构体
    Like, // 点赞消息的结构体
    ComboInfo, // 礼物连击消息的结构体
    LiveCmd, // CMD的枚举 根据枚举匹配对应的解析
    MatchedData, // 匹配到的数据枚举{"cmd":"xx","data":"就是这里的部分，用枚举表示了"}
    msg_parser // 一个解析消息的助手函数
};

// 打包心跳包
let heartbeat = Proto::pack(None,Operation::Heartbeat,vec![]);

// 打包鉴权包
let auth = Proto::pack(None,Operation::Auth,vec![]);

// 解析proto消息
let data : Vec<u8> = vec![0, 0, 0, 21, 0, 16, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 104, 101, 108, 108, 111].to_vec();
let proto = Proto::unpack(None, data); // String::from_utf8(proto.body) 输出 "hello"
```