pub use kovi::tokio;
use kovi::{
    async_move,
    log::info,
    utils::{load_json_data, save_json_data},
    PluginBuilder as p,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
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
async fn main() {
    let bot = p::get_runtime_bot();
    let mut path = bot.get_data_path();
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

    let msg = Arc::new(config.msg.clone());
    let path = Arc::new(path);
    let config_arc = Arc::new(Mutex::new(config));

    p::on_msg(async_move!(e; bot, msg, config_arc, path; {
        if e.borrow_text() != Some(&msg.cmd) {
            return;
        }

        {
            let config_lock = config_arc.lock().unwrap();

            if config_lock.today.contains(&e.user_id) {
                e.reply_and_quote(&msg.today);
                return;
            }
        }

        let res = bot.send_like_return(e.user_id, 10).await;

        {
            let mut config_lock = config_arc.lock().unwrap();

            match res {
                Ok(_) => {
                    e.reply_and_quote(&msg.like);
                    config_lock.today.push(e.user_id);
                    save_json_data(&*config_lock, &*path).unwrap();
                }
                Err(_) => e.reply_and_quote(&msg.do_not_like_you),
            };
        }
    }));

    p::cron("0 0 * * *", move || {
        let config_clone_for_reset = config_arc.clone();
        let path = path.clone();
        async move {
            // 清空 `today`
            let mut config = config_clone_for_reset.lock().unwrap();
            config.today.clear();
            config.data_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            info!("like插件正在清理");
            save_json_data(&*config, &*path).unwrap();
        }
    })
    .unwrap();
}
