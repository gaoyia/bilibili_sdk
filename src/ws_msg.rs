use serde::{ Deserialize, Serialize};
use serde_json::Value;
use crate::AnchorInfo;
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LiveCmd {
    LiveOpenPlatformDm,
    LiveOpenPlatformSendGift,
    LiveOpenPlatformSuperChat,
    LiveOpenPlatformSuperChatDelete,
    LiveOpenPlatformGuard,
    LiveOpenPlatformLike,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(untagged)]
pub enum MatchedData {
    DM(DM),
    SendGift(SendGift),
    SuperChat(SuperChat),
    SuperChatDelete(SuperChatDelete),
    Guard(Guard),
    Like(Like),
}

// 顶层结构体，包含cmd和data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LiveEventData {
    pub cmd: LiveCmd,
    pub data: Value, // 根据cmd对应的数据
}

// 弹幕接收事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DM {
    pub room_id: u64, // 直播间ID
    pub open_id: String, // 用户唯一标识
    pub uname: String, // 用户昵称
    pub msg: String, // 弹幕内容
    pub msg_id: String, // 消息唯一id
    pub fans_medal_level: u8, // 对应房间勋章信息
    pub fans_medal_name: String, // 粉丝勋章名
    pub fans_medal_wearing_status: bool, // 该房间粉丝勋章佩戴情况
    pub guard_level: u8, // 对应房间大航海等级
    pub timestamp: u64, // 弹幕发送时间秒级时间戳
    pub uface: String, // 用户头像
    pub emoji_img_url: String, // 表情包图片地址
    pub dm_type: u8, // 弹幕类型 0：普通弹幕 1：表情包弹幕
}

// 送礼物事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendGift {
    pub room_id: u64, // 直播间ID
    pub open_id: String, // 用户唯一标识
    pub uname: String, // 送礼用户昵称
    pub gift_id: u64, // 道具id
    pub gift_name: String, // 道具名
    pub gift_num: u64, // 赠送道具数量
    pub price: u64, // 礼物单价(1000 = 1元 = 10电池)
    pub fans_medal_level: u64, // 实际收礼人的勋章信息
    pub fans_medal_name: String, // 粉丝勋章名
    pub fans_medal_wearing_status: bool, // 该房间粉丝勋章佩戴情况
    pub guard_level: u8, // room_id对应的大航海等级
    pub timestamp: u64, // 收礼时间秒级时间戳
    pub msg_id: String, // 消息唯一id
    pub anchor_info: AnchorInfo, // 收礼主播信息
    pub gift_icon: String, // 道具icon
    pub combo_gift: bool, // 是否是combo道具
    pub combo_info: ComboInfo, // 连击信息
}

// 超级聊天事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SuperChat {
    pub room_id: u64, // 直播间ID
    pub open_id: String, // 购买用户唯一标识
    pub uname: String, // 购买的用户昵称
    pub message_id: u64, // 留言id
    pub message: String, // 留言内容
    pub rmb: f64, // 支付金额(元)
    pub timestamp: u64, // 赠送时间秒级
    pub start_time: u64, // 生效开始时间
    pub end_time: u64, // 生效结束时间
    pub guard_level: u8, // 对应房间大航海等级
    pub fans_medal_level: u64, // 对应房间勋章信息
    pub fans_medal_name: String, // 对应房间勋章名字
    pub fans_medal_wearing_status: bool, // 该房间粉丝勋章佩戴情况
}

// 超级聊天删除事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SuperChatDelete {
    pub room_id: u64, // 直播间ID
    pub message_ids: Vec<u64>, // 留言ID列表
}

// 大航海事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Guard {
    pub user_info: UserInfo, // 用户信息
    pub guard_level: u8, // 对应的大航海等级
    pub guard_num: u64, // 大航海数量
    pub guard_unit: String, // 大航海单位(个月)
    pub price: u64, // 价格
    pub fans_medal_level: u64, // 粉丝勋章等级
    pub fans_medal_name: String, // 粉丝勋章名
    pub fans_medal_wearing_status: bool, // 该房间粉丝勋章佩戴情况
    pub timestamp: u64, // 时间戳
    pub room_id: u64, // 直播间ID
    pub msg_id: Option<String>, // 消息唯一ID
}

// 点赞事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Like {
    pub uname: String, // 主播昵称
    pub open_id: String, // 用户唯一标识
    pub uface: String, // 用户头像
    pub like_text: String, // 点赞文本
    pub like_count: u64, // 点赞数量
    pub timestamp: u64, // 时间戳
    pub msg_id: String, // 消息唯一ID
    pub room_id: u64, // 直播间ID
}

// 用户信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub open_id: String, // 用户唯一标识
    pub uname: String, // 用户昵称
    pub uface: String, // 用户头像
}

// 连击信息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComboInfo {
    pub combo_base_num: u64, // 每次连击赠送的道具数量
    pub combo_count: u64, // 连击次数
    pub combo_id: String, // 连击id
    pub combo_timeout: u64, // 连击有效期秒
}

pub fn msg_parser(json_str:&str) -> MatchedData {
    let data: LiveEventData = serde_json::from_str(json_str).unwrap();
    match data.cmd {
        LiveCmd::LiveOpenPlatformDm => {
            let dm: DM = serde_json::from_value(data.data).unwrap();
            return MatchedData::DM(dm)
        }
        LiveCmd::LiveOpenPlatformSendGift=> {
            let sg: SendGift = serde_json::from_value(data.data).unwrap();
            return MatchedData::SendGift(sg)
        }
        LiveCmd::LiveOpenPlatformSuperChat=> {
            let sc: SuperChat = serde_json::from_value(data.data).unwrap();
            return MatchedData::SuperChat(sc)
        }
        LiveCmd::LiveOpenPlatformSuperChatDelete=> {
            let scd: SuperChatDelete = serde_json::from_value(data.data).unwrap();
            return MatchedData::SuperChatDelete(scd)
        }
        LiveCmd::LiveOpenPlatformGuard=> {
            let g: Guard = serde_json::from_value(data.data).unwrap();
            return MatchedData::Guard(g)
        }
        LiveCmd::LiveOpenPlatformLike=> {
            let l: Like = serde_json::from_value(data.data).unwrap();
            return MatchedData::Like(l)
        }
    };
}