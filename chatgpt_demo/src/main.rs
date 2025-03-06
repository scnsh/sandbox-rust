use anyhow::Result;
use dialoguer::Input;
use reqwest::{Client, RequestBuilder};

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
// 役割を表す列挙型
enum Role {
    System,    // システムの役割
    User,      // ユーザーの役割
    Assistant, // アシスタントの役割
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
// メッセージを表す構造体
struct Message {
    role: Role,        // メッセージの役割
    content: String,   // メッセージの内容
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
// リクエストボディを表す構造体
struct RequestBody {
    model: String,         // 使用するモデル名
    messages: Vec<Message>, // メッセージのリスト
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
// 選択肢を表す構造体
struct Choice {
    index: u64,          // 選択肢のインデックス
    message: Message,    // 選択されたメッセージ
    finish_reason: String, // 終了理由
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
// 使用量を表す構造体
struct Usage {
    prompt_tokens: u64,     // プロンプトのトークン数
    completion_tokens: u64, // 完了のトークン数
    total_tokens: u64,      // 合計トークン数
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
// レスポンスボディを表す構造体
struct ResponseBody {
    id: String,            // レスポンスのID
    object: String,        // オブジェクトの種類
    created: u64,          // 作成日時
    choices: Vec<Choice>,  // 選択肢のリスト
    usage: Usage,          // 使用量の情報
}

// 共通のヘッダーを設定する関数
fn common_header(api_key: &str) -> RequestBuilder {
    let api_key_field = format!("Bearer {}", api_key);

    Client::new()
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", api_key_field.as_str())
}

// APIにクエリを送信する非同期関数
async fn query(api_key: &str, input_messages: &[Message]) -> Result<Message> {
    // APIリクエストを送信し、レスポンスを受け取る
    let mut response_body = common_header(api_key)
        // リクエストボディをJSON形式で設定
        .json(&RequestBody {
            model: "gpt-4-turbo".to_string(), // 使用するモデル名を指定
            messages: Vec::from(input_messages), // メッセージのリストを設定
        })
        // リクエストを送信
        .send()
        .await?
        // レスポンスをResponseBody構造体にデシリアライズ
        .json::<ResponseBody>()
        .await?;

    // 最初の選択肢のメッセージを取得
    let res = response_body.choices.remove(0).message;

    Ok(res)
}

// メイン関数
#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let api_key = std::env::var("OPENAI_API_KEY")?;

    let mut messages = vec![Message {
        role: Role::System,
        content: "You are a helpful assistant that can answer questions and help with tasks.".to_string(),
    }];

    loop {
        let input = Input::new()
            .with_prompt("You") // ユーザーに "You" というプロンプトを表示
            .interact_text()    // ユーザーからのテキスト入力を待機
            .unwrap_or_else(|_| "quit".to_string()); // 入力に失敗した場合は "quit" をデフォルト値として使用

        if input == "quit" {
            break;
        }

        messages.push(Message {
            role: Role::User,
            content: input,
        });

        let response = query(&api_key, &messages).await?;

        println!("ChatGPT: {}", response.content);

        messages.push(response);
    }

    Ok(())
}
