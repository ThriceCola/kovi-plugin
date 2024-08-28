# kovi-plugin-like

kovi 的点赞插件，可在config.json里配置。

使用 ```cargo kovi add like``` or ```cargo kovi add kovi-plugin-like``` 添加此插件。（记得挂载哦）

下面是配置文件介绍

```rust
let config = serde_json::from_value(json!({
    "today": [], //储存今天赞过的人
    "data_time": 1, //储存数据保存的时间戳 无需在意这个
    "like_times": 10, //点赞次数
    "msg": {
        "cmd": "赞我", //触发命令
        "like": "已为你点赞10次", //成功
        "today": "今天赞过了，一边呆着去！", //赞过了
        "do_not_like_you":"就不给你点，略略略" //意外出错
    }
}))
```