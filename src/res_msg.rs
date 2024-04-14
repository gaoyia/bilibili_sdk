use serde::{Deserialize, Serialize};

// 定义整个响应数据的结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseData {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>, // 如果请求失败会没有data字段
    pub request_id:String
}

// 定义包含场次信息的结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_game_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_info: Option<GameInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub websocket_info: Option<WebsocketInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor_info: Option<AnchorInfo>,
}

// 定义场次信息的结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameInfo {
    pub game_id: String,
}

// 定义长连信息的结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebsocketInfo {
    pub auth_body: String,
    pub wss_link: Vec<String>,
}

// 定义主播信息的结构体
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnchorInfo {
    pub room_id: u64,
    pub uname: String,
    pub uface: String,
    pub open_id: String,
}
