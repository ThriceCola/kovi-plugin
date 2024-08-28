pub use kovi::tokio;
use kovi::{
    log::info,
    utils::{load_json_data, save_json_data},
    PluginBuilder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    today: Vec<i64>,
    /// 储存秒级别时间戳
    data_time: u64,
    like_times: usize,
    msg: Msg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Msg {
    cmd: String,
    like: String,
    today: String,
    do_not_like_you: String,
}

#[kovi::plugin]
pub async fn main(mut p: PluginBuilder) {
    let mut path = p.get_data_path();
    path.push("config.json");

    let config: Config = {
        let config: Config = serde_json::from_value(json!({
            "today": [],
            "data_time": 1,
            "like_times": 10,
            "msg": {
                "cmd": "赞我",
                "like": "已为你点赞10次",
                "today": "今天赞过了，一边呆着去！",
                "do_not_like_you": "就不给你点，略略略"
            }
        }))
        .unwrap();

        let mut config: Config = match load_json_data(config.clone(), &path) {
            Ok(v) => v,
            Err(e) => {
                // 是json解析报错的话?
                if let Some(_parse_error) = e.downcast_ref::<serde_json::Error>() {
                    save_json_data(&config, &path).unwrap();
                    config
                } else {
                    panic!("{e}")
                }
            }
        };
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        if config.data_time / 86400 != now / 86400 {
            config.today = Vec::new();
            config.data_time = now;
            save_json_data(&config, &path).unwrap();
        }
        config
    };
    let msg = config.msg.clone();

    let config_arc = Arc::new(Mutex::new(config));
    let bot = p.build_runtime_bot();

    p.on_msg({
        let config_clone = Arc::clone(&config_arc);
        let path = path.clone();
        move |e| {
            if e.borrow_text() != Some(&msg.cmd) {
                return;
            }

            let mut config_lock = config_clone.lock().unwrap();

            if config_lock.today.contains(&e.user_id) {
                e.reply_and_quote(&msg.today);
                return;
            }

            match bot.send_like_return(e.user_id, 10) {
                Ok(_) => {
                    e.reply_and_quote(&msg.like);
                    config_lock.today.push(e.user_id);
                    save_json_data(&*config_lock, &path).unwrap();
                }
                Err(_) => e.reply_and_quote(&msg.do_not_like_you),
            };
        }
    });

    tokio::spawn({
        let config_clone_for_reset = Arc::clone(&config_arc);
        async move {
            loop {
                let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                let seconds_since_midnight = now.as_secs() % 86400;
                let seconds_until_midnight = 86400 - seconds_since_midnight;

                // 等待直到下一次午夜
                tokio::time::sleep(Duration::from_secs(seconds_until_midnight)).await;

                // 清空 `today`
                let mut config = config_clone_for_reset.lock().unwrap();
                config.today.clear();
                config.data_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                info!("like插件正在清理");
                save_json_data(&*config, &path).unwrap();
            }
        }
    });
}
