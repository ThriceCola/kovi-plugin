use kovi::bot::runtimebot::{rand_echo, send_api_request_with_response};
use kovi::bot::SendApi;
use kovi::serde_json::{self, json};
use kovi::{
    bot::{message::Segment, runtimebot::RuntimeBot, ApiReturn},
    Message,
};
use serde::{Deserialize, Serialize};

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct NapcatStatus {
//     pub status: i64,
//     #[serde(rename = "extStatus")]
//     pub ext_status: i64,
//     #[serde(rename = "batteryStatus")]
//     pub battery_status: i64,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvitedRequest {
    pub request_id: i64,
    pub invitor_uin: i64,
    pub invitor_nick: String,
    pub group_id: i64,
    pub group_name: String,
    pub checked: bool,
    pub actor: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinRequest {
    pub request_id: i64,
    pub requester_uin: i64,
    pub requester_nick: String,
    pub message: String,
    pub group_id: i64,
    pub group_name: String,
    pub checked: bool,
    pub actor: i64,
}

impl InvitedRequest {
    pub fn new(
        request_id: i64,
        invitor_uin: i64,
        invitor_nick: String,
        group_id: i64,
        group_name: String,
        checked: bool,
        actor: i64,
    ) -> Self {
        Self {
            request_id,
            invitor_uin,
            invitor_nick,
            group_id,
            group_name,
            checked,
            actor,
        }
    }
}

impl JoinRequest {
    pub fn new(
        request_id: i64,
        requester_uin: i64,
        requester_nick: String,
        message: String,
        group_id: i64,
        group_name: String,
        checked: bool,
        actor: i64,
    ) -> Self {
        Self {
            request_id,
            requester_uin,
            requester_nick,
            message,
            group_id,
            group_name,
            checked,
            actor,
        }
    }
}

pub trait NapCatApi {
    fn set_qq_profile(
        &self,
        nickname: &str,
        company: &str,
        email: &str,
        college: &str,
        personal_note: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_online_clients(
        &self,
        no_cache: bool,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn mark_msg_as_read(
        &self,
        message_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn send_group_forward_msg(
        &self,
        group_id: i64,
        nodes: Vec<Node>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn send_private_forward_msg(
        &self,
        user_id: i64,
        nodes: Vec<Node>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_group_msg_history(
        &self,
        group_id: i64,
        message_seq: Option<i64>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn ocr_image(
        &self,
        image: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_group_system_msg(
        &self,
        invited_requests: Vec<InvitedRequest>,
        join_requests: Vec<JoinRequest>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_essence_msg_list(
        &self,
        group_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn set_group_portrait(
        &self,
        group_id: i64,
        file: &str,
        cache: bool,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn set_essence_msg(
        &self,
        message_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn delete_essence_msg(
        &self,
        message_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn set_group_sign(
        &self,
        group_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn send_group_notice(
        &self,
        group_id: i64,
        content: &str,
        image: Option<&str>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_group_notice(
        &self,
        group_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn upload_group_file(
        &self,
        group_id: i64,
        file: &str,
        name: &str,
        folder: Option<&str>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn delete_group_file(
        &self,
        group_id: i64,
        file_id: &str,
        busid: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn create_group_file_folder(
        &self,
        group_id: i64,
        name: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn delete_group_folder(
        &self,
        group_id: i64,
        folder_id: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_group_file_system_info(
        &self,
        group_id: i64,
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
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn upload_private_file(
        &self,
        user_id: i64,
        file: &str,
        name: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn download_file(
        &self,
        url: &str,
        thread_count: i64,
        headers: Vec<String>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn ark_share_peer(
        &self,
        user_id: Option<&str>,
        phone_number: Option<&str>,
        group_id: Option<&str>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn ark_share_group(
        &self,
        group_id: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_robot_uin_range(
        &self,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn set_online_status(
        &self,
        status: i64,
        ext_status: i64,
        battery_status: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_friends_with_category(
        &self,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn set_qq_avatar(
        &self,
        file: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_file(
        &self,
        file_id: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn forward_friend_single_msg(
        &self,
        user_id: i64,
        message: Vec<Node>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn forward_group_single_msg(
        &self,
        group_id: i64,
        message: Vec<Node>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn set_msg_emoji_like(
        &self,
        message_id: i64,
        emoji_id: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn translate_en2zh(
        &self,
        words: Vec<String>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn send_forward_msg(
        &self,
        message_type: &str,
        user_id: Option<i64>,
        group_id: Option<i64>,
        message: Vec<Node>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn send_forward_msg_to_user(
        &self,
        user_id: i64,
        message: Vec<Node>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn send_forward_msg_to_group(
        &self,
        group_id: i64,
        message: Vec<Node>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn mark_private_msg_as_read(
        &self,
        user_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn mark_group_msg_as_read(
        &self,
        group_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_friend_msg_history(
        &self,
        user_id: i64,
        message_seq: &str,
        count: i64,
        reverse_order: bool,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn create_collection(
        &self,
        brief: &str,
        raw_data: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    // get_collection_list

    fn set_self_longnick(
        &self,
        long_nick: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    // get_self_longnick

    // get_self_profile

    // set_self_profile

    // get_friend_profile

    fn get_friend_list(
        &self,
        no_cache: bool,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_recent_contact(
        &self,
        count: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn mark_all_as_read(
        &self,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_profile_like(
        &self,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn fetch_custom_face(
        &self,
        message_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn fetch_emoji_like(
        &self,
        message_id: i64,
        emoji_id: &str,
        emoji_type: &str,
        group_id: Option<i64>,
        user_id: Option<i64>,
        count: Option<i64>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    // set_input_status

    fn get_group_info_ex(
        &self,
        group_id: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    // get_group_ignore_add_request

    fn _del_group_notice(
        &self,
        group_id: &str,
        notice_id: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    // fetch_user_profile_like

    fn friend_poke(
        &self,
        user_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn group_poke(
        &self,
        group_id: i64,
        user_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn nc_get_packet_status(
        &self,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn nc_get_user_status(
        &self,
        user_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn nc_get_rkey(&self)
        -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;

    fn get_group_shut_list(
        &self,
        group_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send;
}

impl NapCatApi for RuntimeBot {
    fn nc_get_user_status(
        &self,
        user_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "nc_get_user_status",
            json!({
                "user_id": user_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn nc_get_rkey(
        &self,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new("nc_get_rkey", json!({}), &rand_echo());

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn nc_get_packet_status(
        &self,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new("nc_get_packet_status", json!({}), &rand_echo());

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn get_group_shut_list(
        &self,
        group_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "get_group_shut_list",
            json!({
                "group_id": group_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn _del_group_notice(
        &self,
        group_id: &str,
        notice_id: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "_del_group_notice",
            json!({
                "group_id": group_id,
                "notice_id": notice_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn get_friend_list(
        &self,
        no_cache: bool,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "get_friend_list",
            json!({
                "no_cache": no_cache,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn fetch_emoji_like(
        &self,
        message_id: i64,
        emoji_id: &str,
        emoji_type: &str,
        group_id: Option<i64>,
        user_id: Option<i64>,
        count: Option<i64>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let mut map = serde_json::Map::new();

        map.insert("message_id".to_string(), json!(message_id));
        map.insert("emojiId".to_string(), json!(emoji_id));
        map.insert("emojiType".to_string(), json!(emoji_type));

        if let Some(gid) = group_id {
            map.insert("group_id".to_string(), json!(gid));
        }

        if let Some(uid) = user_id {
            map.insert("user_id".to_string(), json!(uid));
        }

        if let Some(i) = count {
            map.insert("count".to_string(), json!(i));
        }

        let send_api = SendApi::new(
            "send_forward_msg",
            serde_json::Value::Object(map),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn create_collection(
        &self,
        brief: &str,
        raw_data: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "create_collection",
            json!({
                "brief": brief,
                "rawData": raw_data,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn get_group_info_ex(
        &self,
        group_id: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        {
            let send_api = SendApi::new(
                "get_group_info_ex",
                json!({
                    "group_id": group_id
                }),
                &rand_echo(),
            );

            send_api_request_with_response(&self.api_tx, send_api)
        }
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

    fn mark_private_msg_as_read(
        &self,
        user_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "mark_private_msg_as_read",
            json!({
                "user_id": user_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn mark_group_msg_as_read(
        &self,
        group_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "mark_group_msg_as_read",
            json!({
                "group_id": group_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn get_recent_contact(
        &self,
        count: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "get_recent_contact",
            json!({
                "count": count,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn get_profile_like(
        &self,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new("get_profile_like", json!({}), &rand_echo());

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn fetch_custom_face(
        &self,
        count: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "fetch_custom_face",
            json!({
                "count": count,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn mark_all_as_read(
        &self,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new("_mark_all_as_read", json!({}), &rand_echo());

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn set_self_longnick(
        &self,
        long_nick: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "set_self_longnick",
            json!({
                "longNick": long_nick,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn get_friend_msg_history(
        &self,
        user_id: i64,
        message_seq: &str,
        count: i64,
        reverse_order: bool,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "get_friend_msg_history",
            json!({
                "user_id": user_id,
                "message_seq": message_seq,
                "count": count,
                "reverseOrder": reverse_order,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn send_forward_msg(
        &self,
        message_type: &str,
        user_id: Option<i64>,
        group_id: Option<i64>,
        message: Vec<Node>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let mut map = serde_json::Map::new();

        map.insert("message_type".to_string(), json!(message_type));

        if let Some(uid) = user_id {
            map.insert("user_id".to_string(), json!(uid));
        }

        if let Some(gid) = group_id {
            map.insert("group_id".to_string(), json!(gid));
        }

        map.insert("message".to_string(), json!(message));

        let send_api = SendApi::new(
            "send_forward_msg",
            serde_json::Value::Object(map),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn send_forward_msg_to_user(
        &self,
        user_id: i64,
        message: Vec<Node>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "send_forward_msg",
            json!({
                "message_type": "private",
                "user_id": user_id,
                "message": message,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn send_forward_msg_to_group(
        &self,
        group_id: i64,
        message: Vec<Node>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "send_forward_msg",
            json!({
                "message_type": "group",
                "group_id": group_id,
                "message": message,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn translate_en2zh(
        &self,
        words: Vec<String>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "translate_en2zh",
            json!({
                "words": words,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn ark_share_peer(
        &self,
        user_id: Option<&str>,
        phone_number: Option<&str>,
        group_id: Option<&str>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let mut map = serde_json::Map::new();

        if let Some(uid) = user_id {
            map.insert("user_id".to_string(), json!(uid));
        }
        if let Some(phone) = phone_number {
            map.insert("phoneNumber".to_string(), json!(phone));
        }
        if let Some(gid) = group_id {
            map.insert("group_id".to_string(), json!(gid));
        }

        let send_api = SendApi::new("ArkSharePeer", serde_json::Value::Object(map), &rand_echo());

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn ark_share_group(
        &self,
        group_id: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "ArkShareGroup",
            json!({
                "group_id": group_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn set_msg_emoji_like(
        &self,
        message_id: i64,
        emoji_id: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "set_msg_emoji_like",
            json!({
                "message_id": message_id,
                "emoji_id": emoji_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn forward_group_single_msg(
        &self,
        group_id: i64,
        message: Vec<Node>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "forward_group_single_msg",
            json!({
                "group_id": group_id,
                "message": message,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn forward_friend_single_msg(
        &self,
        user_id: i64,
        message: Vec<Node>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "forward_friend_single_msg",
            json!({
                "user_id": user_id,
                "message": message,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn get_file(
        &self,
        file_id: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "get_file",
            json!({
                "file_id": file_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn set_qq_avatar(
        &self,
        file: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "set_qq_avatar",
            json!({
                "file": file,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }
    fn get_friends_with_category(
        &self,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new("get_friends_with_category", json!({}), &rand_echo());

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn set_online_status(
        &self,
        status: i64,
        ext_status: i64,
        battery_status: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "set_online_status",
            json!({
                "status": status,
                "ext_status": ext_status,
                "battery_status": battery_status,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn get_robot_uin_range(
        &self,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new("get_robot_uin_range", json!({}), &rand_echo());

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn download_file(
        &self,
        url: &str,
        thread_count: i64,
        headers: Vec<String>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "download_file",
            json!({
                "url": url,
                "thread_count": thread_count,
                "headers": headers,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn upload_private_file(
        &self,
        user_id: i64,
        file: &str,
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
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "get_group_file_url",
            json!({
                "group_id": group_id,
                "file_id": file_id,
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

    fn get_group_file_system_info(
        &self,
        group_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "get_group_file_system_info",
            json!({
                "group_id": group_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn delete_group_folder(
        &self,
        group_id: i64,
        folder_id: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "delete_group_folder",
            json!({
                "group_id": group_id,
                "folder_id": folder_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn create_group_file_folder(
        &self,
        group_id: i64,
        name: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "create_group_file_folder",
            json!({
                "group_id": group_id,
                "name": name,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn delete_group_file(
        &self,
        group_id: i64,
        file_id: &str,
        busid: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "delete_group_file",
            json!({
                "group_id": group_id,
                "file_id": file_id,
                "busid": busid,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn set_group_sign(
        &self,
        group_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "send_group_sign",
            json!({
                "group_id": group_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn delete_essence_msg(
        &self,
        message_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "delete_essence_msg",
            json!({
                "message_id": message_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn set_essence_msg(
        &self,
        message_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "set_essence_msg",
            json!({
                "message_id": message_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn get_essence_msg_list(
        &self,
        group_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "get_essence_msg_list",
            json!({
                "group_id": group_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn get_group_system_msg(
        &self,
        invited_requests: Vec<InvitedRequest>,
        join_requests: Vec<JoinRequest>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "get_group_system_msg",
            json!({
                "invited_requests": invited_requests,
                "join_requests": join_requests,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn ocr_image(
        &self,
        image: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "ocr_image",
            json!({
                "image": image,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn get_group_msg_history(
        &self,
        group_id: i64,
        message_seq: Option<i64>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = if let Some(message_seq) = message_seq {
            SendApi::new(
                "get_group_msg_history",
                json!({
                    "group_id": group_id,
                    "message_seq": message_seq,
                }),
                &rand_echo(),
            )
        } else {
            SendApi::new(
                "get_group_msg_history",
                json!({
                    "group_id": group_id,
                }),
                &rand_echo(),
            )
        };

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn send_private_forward_msg(
        &self,
        user_id: i64,
        nodes: Vec<Node>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "send_private_forward_msg",
            json!({
                "user_id": user_id,
                "message": nodes,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn send_group_forward_msg(
        &self,
        group_id: i64,
        nodes: Vec<Node>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "send_group_forward_msg",
            json!({
                "group_id": group_id,
                "message": nodes,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn mark_msg_as_read(
        &self,
        message_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "mark_msg_as_read",
            json!({
                "message_id": message_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn get_online_clients(
        &self,
        no_cache: bool,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "get_online_clients",
            json!({
                "no_cache": no_cache,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn set_qq_profile(
        &self,
        nickname: &str,
        company: &str,
        email: &str,
        college: &str,
        personal_note: &str,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "set_qq_profile",
            json!({
                "nickname": nickname,
                "company": company,
                "email": email,
                "college": college,
                "personal_note": personal_note,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn upload_group_file(
        &self,
        group_id: i64,
        file: &str,
        name: &str,
        folder: Option<&str>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = if let Some(folder) = folder {
            SendApi::new(
                "upload_group_file",
                json!({
                    "group_id": group_id,
                    "file": file,
                    "name": name,
                    "folder":folder
                }),
                &rand_echo(),
            )
        } else {
            SendApi::new(
                "upload_group_file",
                json!({
                    "group_id": group_id,
                    "file": file,
                    "name": name,
                }),
                &rand_echo(),
            )
        };

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn get_group_notice(
        &self,
        group_id: i64,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "_get_group_notice",
            json!({
                "group_id": group_id,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn send_group_notice(
        &self,
        group_id: i64,
        content: &str,
        image: Option<&str>,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = SendApi::new(
            "_send_group_notice",
            json!({
                "group_id": group_id,
                "content": content,
                "image": image,
            }),
            &rand_echo(),
        );

        send_api_request_with_response(&self.api_tx, send_api)
    }

    fn set_group_portrait(
        &self,
        group_id: i64,
        file: &str,
        cache: bool,
    ) -> impl std::future::Future<Output = Result<ApiReturn, ApiReturn>> + Send {
        let send_api = if cache {
            SendApi::new(
                "set_group_portrait",
                json!({
                    "group_id": group_id,
                    "file": file,
                    "cache": 1,
                }),
                &rand_echo(),
            )
        } else {
            SendApi::new(
                "set_group_portrait",
                json!({
                    "group_id": group_id,
                    "file": file,
                    "cache": 0,
                }),
                &rand_echo(),
            )
        };

        send_api_request_with_response(&self.api_tx, send_api)
    }
}

pub type Node = Segment;

pub trait NapCatVec {
    /// 伪造合并转发消息, 无需伪造使用 add_forward_node()
    ///
    /// # Examples
    ///
    /// ```
    /// let nodes = Vec::new()
    ///     .add_fake_node_from_content("10000", "测试", Message::from("some"))
    ///     .add_fake_node_from_content("10000", "测试2", Message::from("some"));
    ///
    /// bot.send_private_msg(bot.main_admin, nodes);
    /// ```
    fn add_fake_node_from_content(
        self,
        user_id: &str,
        nickname: &str,
        content: Message,
    ) -> Vec<Node>;

    /// 添加一个伪造的合并转发消息节点，通过指定的消息ID。
    ///
    /// # Examples
    ///
    /// ```
    /// let mut nodes = Vec::new();
    ///
    /// node.push_fake_node_from_id("10000", "测试", "10000")
    ///
    /// bot.send_private_msg(bot.main_admin, nodes);
    /// ```
    fn add_fake_node_from_id(self, user_id: &str, nickname: &str, id: &str) -> Vec<Segment>;

    /// 伪造合并转发消息, 无需伪造使用 push_fake_forward_node()
    ///
    /// # Examples
    ///
    /// ```
    /// let mut nodes = Vec::new();
    ///
    /// node.push_fake_node_from_content("10000", "测试", Message::from("some"))
    ///
    /// bot.send_private_msg(bot.main_admin, nodes);
    /// ```
    fn push_fake_node_from_content(&mut self, user_id: &str, nickname: &str, content: Message);

    /// 在现有的 `Vec<Segment>` 中添加一个伪造的合并转发消息节点，通过指定的消息ID。
    ///
    /// # Examples
    ///
    /// ```
    /// let mut nodes = Vec::new();
    /// nodes.push_fake_node_from_id("10000", "测试", "10000");
    /// ```
    fn push_fake_node_from_id(&mut self, user_id: &str, nickname: &str, id: &str);

    /// 合并转发消息, 使用消息id，伪造请使用 add_fake_node_from_id()
    ///
    /// # Examples
    ///
    /// ```
    /// let nodes = Vec::new()
    ///     .add_node_from_id("10000")
    ///     .add_node_from_id("10001");
    ///
    /// bot.send_private_msg(bot.main_admin, nodes);
    /// ```
    fn add_node_from_id(self, id: &str) -> Vec<Node>;

    /// 合并转发消息, 使用消息内容，伪造请使用 add_fake_node_from_content()
    ///
    /// # Examples
    ///
    /// ```
    /// let nodes = Vec::new().add_node_from_content(Message::from("Hello, world!"));
    ///
    /// bot.send_private_msg(bot.main_admin, nodes);
    /// ```
    fn add_node_from_content(self, content: Message) -> Vec<Node>;

    /// 合并转发消息, 使用消息id，伪造请使用 push_fake_node_from_id()
    ///
    /// # Examples
    ///
    /// ```
    /// let mut nodes = Vec::new();
    ///
    /// node.push_node_from_id("10000")
    ///
    /// bot.send_private_msg(bot.main_admin, nodes);
    /// ```
    fn push_node_from_id(&mut self, id: &str);

    /// 在现有的 `Vec<Segment>` 中添加一个合并转发消息节点，使用消息内容，伪造请使用 push_fake_node_from_content()
    ///
    /// # Examples
    ///
    /// ```
    /// let mut nodes = Vec::new();
    /// nodes.push_node_from_content(Message::from("Hello, world!"));
    /// bot.send_private_msg(bot.main_admin, nodes);
    /// ```
    fn push_node_from_content(&mut self, content: Message);
}

impl NapCatVec for Vec<Node> {
    fn add_fake_node_from_content(
        mut self,
        user_id: &str,
        nickname: &str,
        content: Message,
    ) -> Vec<Segment> {
        self.push(Segment {
            type_: "node".to_string(),
            data: json!({
                "user_id": user_id,
                "nickname": nickname,
                "content": content,
            }),
        });
        self
    }
    fn push_fake_node_from_content(&mut self, user_id: &str, nickname: &str, content: Message) {
        self.push(Segment {
            type_: "node".to_string(),
            data: json!({
                "user_id": user_id,
                "nickname": nickname,
                "content": content,
            }),
        });
    }
    fn add_fake_node_from_id(mut self, user_id: &str, nickname: &str, id: &str) -> Vec<Segment> {
        self.push(Segment {
            type_: "node".to_string(),
            data: json!({
                "id": id,
                "user_id": user_id,
                "nickname": nickname,
            }),
        });
        self
    }

    fn push_fake_node_from_id(&mut self, user_id: &str, nickname: &str, id: &str) {
        self.push(Segment {
            type_: "node".to_string(),
            data: json!({
                "id": id,
                "user_id": user_id,
                "nickname": nickname,
            }),
        });
    }

    fn add_node_from_id(mut self, id: &str) -> Vec<Segment> {
        self.push(Segment {
            type_: "node".to_string(),
            data: json!({
                "id": id,
            }),
        });
        self
    }

    fn push_node_from_id(&mut self, id: &str) {
        self.push(Segment {
            type_: "node".to_string(),
            data: json!({
                "id": id,
            }),
        });
    }

    fn add_node_from_content(mut self, content: Message) -> Vec<Segment> {
        self.push(Segment {
            type_: "node".to_string(),
            data: json!({
                "content": content,
            }),
        });
        self
    }

    fn push_node_from_content(&mut self, content: Message) {
        self.push(Segment {
            type_: "node".to_string(),
            data: json!({
                "content": content,
            }),
        });
    }
}
