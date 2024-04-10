use bilibili_sdk::{msg_parser, MatchedData};
#[tokio::main]
async fn main() {
    let dm_str = r#"{
        "cmd":"LIVE_OPEN_PLATFORM_DM",
        "data":{
            "room_id":1,
            "uid":0,
            "open_id":"39b8fedb-60a5-4e29-ac75-b16955f7e632",
            "uname":"",
            "msg":"",
            "msg_id":"",
            "fans_medal_level":0,
            "fans_medal_name":"粉丝勋章名",
            "fans_medal_wearing_status": true,
            "guard_level":0,
            "timestamp":0,
            "uface":"",
            "emoji_img_url": "",
            "dm_type": 0
        }
    }"#;
    let msg = msg_parser(dm_str);
    match msg {
        MatchedData::DM(dm) => {
            println!("dm-{:#?}", dm);
        }
        MatchedData::SendGift(sg) => {
            println!("sg-{:#?}", sg);
        }
        MatchedData::SuperChat(sc) => {
            println!("sc-{:#?}", sc);
        }
        MatchedData::SuperChatDelete(scd)=> {
            println!("scd-{:#?}", scd);
        }
        MatchedData::Guard(guard)=> {
            println!("guard-{:#?}", guard);
        }
        MatchedData::Like(like)=> {
            println!("like-{:#?}", like);
        }
    }
}
