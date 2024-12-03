# kovi-plugin-expand-napcat

Kovi 的 Napcta Api 拓展插件。

Napcta Api文档不齐全，如果出错请及时提交 issue 。

使用 `cargo kovi add expand-napcat -p <PLUGIN_NAME>` or `cargo add kovi-plugin-expand-napcat -p <PLUGIN_NAME>` 添加此拓展。

懒得写了，直接看 napcat 的文档就行了: [napcat](https://napcat.napneko.icu/) 。

合并转发例子：

```rust
use kovi::{Message, PluginBuilder as p};
// 两个 trait，第一个用于 RuntimeBot，第二个用于 Vec
use kovi_plugin_expand_napcat::{NapCatApi, NapCatVec};

#[kovi::plugin]
async fn main() {
    let bot = p::get_runtime_bot();

    let mut nodes = Vec::new();

    nodes.push_node_from_id("10000");

    bot.send_private_msg(bot.main_admin, nodes);

    // 伪造
    let fake_nodes = Vec::new().add_fake_node_from_content("10000", "测试用户", Message::from("测试"));

    bot.send_private_msg(bot.main_admin, fake_nodes);
}
```
