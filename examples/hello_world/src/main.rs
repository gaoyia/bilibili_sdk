use bilibili_sdk::{BilibiliSDK, Config, ResponseData};

#[tokio::main]
async fn main() {
    let config: Config = Config {
        access_secret_key: "xxxxxxxxxx".to_owned(), // 替换成你自己的
        access_key_id: "xxxxxxxxxx".to_owned(),// 替换成你自己的
        ..Default::default()
    };
    let app_id: u64 = 1234567890123;// 替换成你自己的
    let code: String = "xxxxxxxxx".to_owned();// 替换成你自己的
    let sdk = BilibiliSDK::new(config);

    // 调用开始接口
    let result = sdk.start(code, app_id).await;
    if result.is_ok() {
        let res = result.unwrap().json::<ResponseData>().await.unwrap();
        if res.code == 0 {
            println!(" 开始的返回: {:#?}", res);
            let game_id = res.data.unwrap().game_info.unwrap().game_id;
            // 循环三次 心跳
            for _ in 0..3 {
                tokio::time::sleep(std::time::Duration::from_secs(15)).await;
                let result = sdk.heartbeat(game_id.clone()).await;
                if result.is_ok() {
                    let res = result.unwrap().json::<ResponseData>().await.unwrap();
                    println!("心跳的返回： {:?}", res);
                } else {
                    println!("{:?}", result.unwrap_err());
                }
            }
            // 循环三次 批量心跳
            for _ in 0..3 {
                tokio::time::sleep(std::time::Duration::from_secs(15)).await;
                let result = sdk.batch_heartbeat(vec![game_id.clone()]).await;
                if result.is_ok() {
                    let res = result.unwrap().json::<ResponseData>().await.unwrap();
                    println!("批量心跳的返回： {:?}", res);
                } else {
                    println!("{:?}", result.unwrap_err());
                }
            }
            // 调用结束接口
            let result = sdk.end(app_id, game_id).await;
            if result.is_ok() {
                let res = result.unwrap().json::<ResponseData>().await.unwrap();
                println!("结束的返回： {:?}", res);
            } else {
                println!("{:?}", result.unwrap_err());
            }
        }
        
    } else {
        println!("{:?}", result.unwrap_err());
    }
}
