# kovi-plugin-expand-lagrange

Kovi 的 Api 拓展插件。

使用 ```cargo kovi add expand-lagrange -t <PLUGIN_NAME>``` or ```cargo add kovi-plugin-expand-lagrange -t <PLUGIN_NAME>``` 添加此拓展。

懒得写了，直接看 Lagrange 的文档就行了: [Largrange](https://lagrangedev.github.io/Lagrange.Doc/Lagrange.OneBot/API/Extend/)

合并转发例子：

```rust
use kovi::{Message, PluginBuilder as p};
// 三个 trait，第一个用于 RuntimeBot，第二个用于 Message，第三个用于 Vec
use kovi_plugin_expand_lagrange::{LagrangeApi, LagrangeMessage, LagrangeVec};

#[kovi::plugin]
async fn main() {
    let bot = p::get_runtime_bot();

    let nodes = Vec::new()
        .add_forward_node("10000", "测试", Message::from("some"))
        .add_forward_node("10000", "测试2", Message::from("some"));

    let res = bot.send_forward_msg(nodes).await.unwrap();
    let resid = res.data.as_str().unwrap();

    bot.send_private_msg(bot.main_admin, Message::new().add_forward_resid(resid));
}
```