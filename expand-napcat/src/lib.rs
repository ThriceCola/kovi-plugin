use kovi::log::error;
use kovi::serde_json::json;
use kovi::tokio::sync::oneshot;
use kovi::{
    bot::{
        message::Segment,
        runtimebot::{rand_echo, RuntimeBot},
        ApiReturn, SendApi,
    },
    Message,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NapcatStatus {
    pub status: i64,
    #[serde(rename = "extStatus")]
    pub ext_status: i64,
    #[serde(rename = "batteryStatus")]
    pub battery_status: i64,
}

pub trait NapcatApi {
    fn set_qq_avatar(
        &self,
        file: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_group_system_msg(
        &self,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_file(
        &self,
        file_id: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn download_file(
        &self,
        url: &str,
        thread_count: u8,
        headers: Option<&str>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn forward_friend_single_msg(
        &self,
        user_id: i64,
        message_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn forward_group_single_msg(
        &self,
        group_id: i64,
        message_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn set_msg_emoji_like(&self, message_id: &str, emoji_id: &str);

    fn set_msg_emoji_like_return(
        &self,
        message_id: &str,
        emoji_id: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn mark_private_msg_as_read(&self, user_id: i64);

    fn mark_group_msg_as_read(&self, group_id: i64);

    fn mark_private_msg_as_read_return(
        &self,
        user_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn mark_group_msg_as_read_return(
        &self,
        group_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_robot_uin_range(
        &self,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn set_online_status(&self, status: i32);
}


impl NapcatApi for RuntimeBot {
    async fn set_qq_avatar(&self, file: &str) -> Result<ApiReturn, ApiReturn> {
        let send_api = SendApi::new(
            "set_qq_avatar",
            json!({
                "file": file,
            }),
            &rand_echo(),
        );

        send_and_return(&self, send_api).await
    }
}

pub type Node = Segment;

pub trait LagrangeVec {
    /// 为了方便使用，这个 trait 可以给 Vec<Segment> 便捷使用
    ///
    /// # Examples
    ///
    /// ```
    /// let nodes = Vec::new()
    ///     .add_forward_node("10000", "测试", Message::from("some"))
    ///     .add_forward_node("10000", "测试2", Message::from("some"));
    /// let res = bot.send_forward_msg(nodes).await.unwrap();
    /// let resid = res.data.as_str().unwrap();
    ///
    /// bot.send_private_msg(bot.main_admin, Message::new().add_forward_resid(resid));
    /// ```
    fn add_forward_node(self, uin: &str, name: &str, content: Message) -> Vec<Node>;
}

impl LagrangeVec for Vec<Node> {
    fn add_forward_node(mut self, uin: &str, name: &str, content: Message) -> Vec<Node> {
        self.push(Segment {
            type_: "node".to_string(),
            data: json!({
                "name": name,
                "uin": uin,
                "content": content,
            }),
        });
        self
    }
}

pub trait LagrangeMessage {
    fn add_forward_resid(self, resid: &str) -> Message;
}

impl LagrangeMessage for Message {
    fn add_forward_resid(mut self, resid: &str) -> Message {
        self.push(Segment {
            type_: "forward".to_string(),
            data: json!({
                "id": resid,
            }),
        });
        self
    }
}

async fn send_and_return(bot: &RuntimeBot, send_api: SendApi) -> Result<ApiReturn, ApiReturn> {
    #[allow(clippy::type_complexity)]
    let (api_tx, api_rx): (
        oneshot::Sender<Result<ApiReturn, ApiReturn>>,
        oneshot::Receiver<Result<ApiReturn, ApiReturn>>,
    ) = oneshot::channel();
    bot.api_tx.send((send_api, Some(api_tx))).await.unwrap();
    match api_rx.await {
        Ok(v) => v,
        Err(e) => {
            error!("{e}");
            panic!()
        }
    }
}
