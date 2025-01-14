use kovi::bot::runtimebot::send_api_request_with_response;
use kovi::serde_json::json;
use kovi::{
    bot::{
        message::Segment,
        runtimebot::{rand_echo, RuntimeBot},
        ApiReturn, SendApi,
    },
    Message,
};
use std::path::Path;

pub trait LagrangeApi {
    fn fetch_custom_face(
        &self,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_friend_msg_history(
        &self,
        user_id: i64,
        message_id: i64,
        count: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_group_msg_history(
        &self,
        group_id: i64,
        message_id: i64,
        count: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn send_forward_msg(
        &self,
        messages: Vec<Segment>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn send_group_forward_msg(
        &self,
        group_id: i64,
        messages: Vec<Segment>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn send_private_forward_msg(
        &self,
        user_id: i64,
        messages: Vec<Segment>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn upload_group_file(
        &self,
        group_id: i64,
        file: &Path,
        name: &str,
        folder: Option<&str>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn upload_private_file(
        &self,
        user_id: i64,
        file: &Path,
        name: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_group_root_files(
        &self,
        group_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_group_files_by_folder(
        &self,
        group_id: i64,
        folder_id: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_group_file_url(
        &self,
        group_id: i64,
        file_id: &str,
        busid: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn friend_poke(
        &self,
        user_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn group_poke(
        &self,
        group_id: i64,
        user_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn friend_poke_return(
        &self,
        user_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn group_poke_return(
        &self,
        group_id: i64,
        user_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn set_group_reaction(
        &self,
        group_id: i64,
        message_id: i64,
        code: &str,
        is_add: bool,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;
}

impl LagrangeApi for RuntimeBot {
    fn fetch_custom_face(
        &self,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new("fetch_custom_face", json!({}), &rand_echo());

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn get_friend_msg_history(
        &self,
        user_id: i64,
        message_id: i64,
        count: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "get_friend_msg_history",
            json!({
                "user_id": user_id,
                "message_id": message_id,
                "count": count
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }
    fn get_group_msg_history(
        &self,
        group_id: i64,
        message_id: i64,
        count: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "get_group_msg_history",
            json!({
                "group_id": group_id,
                "message_id": message_id,
                "count": count
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    /// 构造合并转发
    ///
    /// 注意是构造，不是发送
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
    fn send_forward_msg(
        &self,
        messages: Vec<Node>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "send_forward_msg",
            json!({
                "messages": messages
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn send_group_forward_msg(
        &self,
        group_id: i64,
        messages: Vec<Node>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "send_group_forward_msg",
            json!({
                "group_id": group_id,
                "messages": messages
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn send_private_forward_msg(
        &self,
        user_id: i64,
        messages: Vec<Segment>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "send_private_forward_msg",
            json!({
                "user_id": user_id,
                "messages": messages
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn upload_group_file(
        &self,
        group_id: i64,
        file: &Path,
        name: &str,
        folder: Option<&str>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let params = match folder {
            Some(folder) => json!({
                "group_id": group_id,
                "file": file,
                "name": name,
                "folder": folder,
            }),
            None => json!({
                "group_id": group_id,
                "file": file,
                "name": name,
            }),
        };

        let send_api = SendApi::new("upload_group_file", params, &rand_echo());

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn upload_private_file(
        &self,
        user_id: i64,
        file: &Path,
        name: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "upload_private_file",
            json!({
                "user_id": user_id,
                "file": file,
                "name": name,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn get_group_root_files(
        &self,
        group_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "get_group_root_files",
            json!({
                "group_id": group_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn get_group_files_by_folder(
        &self,
        group_id: i64,
        folder_id: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "get_group_files_by_folder",
            json!({
                "group_id": group_id,
                "folder_id": folder_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn get_group_file_url(
        &self,
        group_id: i64,
        file_id: &str,
        busid: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "get_group_file_url",
            json!({
                "group_id": group_id,
                "file_id": file_id,
                "busid": busid,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn friend_poke(
        &self,
        user_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "friend_poke",
            json!({
                "user_id": user_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn group_poke(
        &self,
        group_id: i64,
        user_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "group_poke",
            json!({
                "group_id": group_id,
                "user_id": user_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn friend_poke_return(
        &self,
        user_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "friend_poke",
            json!({
                "user_id": user_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn group_poke_return(
        &self,
        group_id: i64,
        user_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "group_poke",
            json!({
                "group_id": group_id,
                "user_id": user_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn set_group_reaction(
        &self,
        group_id: i64,
        message_id: i64,
        code: &str,
        is_add: bool,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "set_group_reaction",
            json!({
                "group_id": group_id,
                "message_id": message_id,
                "reaction_id": code,
                "is_add": is_add,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }
}

pub type Node = Segment;

pub trait LagrangeVec {
    /// 为了方便使用，这个 trait 可以给 Vec<Segment> 便捷
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
    fn add_forward_node(self, uin: &str, name: &str, content: Message) -> Vec<Segment>;

    fn push_forward_node(&mut self, uin: &str, name: &str, content: Message);
}

impl LagrangeVec for Vec<Segment> {
    fn add_forward_node(mut self, uin: &str, name: &str, content: Message) -> Vec<Segment> {
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
    fn push_forward_node(&mut self, uin: &str, name: &str, content: Message) {
        self.push(Segment {
            type_: "node".to_string(),
            data: json!({
                "name": name,
                "uin": uin,
                "content": content,
            }),
        });
    }
}

pub trait LagrangeMessage {
    fn add_forward_resid(self, resid: &str) -> Message;

    fn push_forward_resid(&mut self, resid: &str);
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

    fn push_forward_resid(&mut self, resid: &str) {
        self.push(Segment {
            type_: "forward".to_string(),
            data: json!({
                "id": resid,
            }),
        });
    }
}
