use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};

pub struct MessageInput {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
    pub attachments: Vec<EmailAttachment>,
}

pub struct EmailAttachment {
    filename: String,
    content_type: String,
    data: Vec<u8>,
}

fn build_email_message(input: MessageInput) -> String {
    let boundary = format!(
        "----=_Boundary_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );

    let mut msg = String::new();
    msg.push_str(&format!("From: {}\r\n", input.from));
    msg.push_str(&format!("To: {}\r\n", input.to));
    msg.push_str(&format!("Subject: {}\r\n", input.subject));
    msg.push_str("MIME-Version: 1.0\r\n");

    if input.attachments.is_empty() {
        msg.push_str("Content-Type: text/plain; charset=\"UTF-8\"\r\n\r\n");
        msg.push_str(input.body.as_str());
        println!("message {}", msg);
        return msg;
    }

    msg.push_str(&format!(
        "Content-Type: multipart/mixed; boundary=\"{}\"\r\n\r\n",
        boundary
    ));
    msg.push_str(&format!("--{}\r\n", boundary));
    msg.push_str("Content-Type: text/plain; charset=\"UTF-8\"\r\n\r\n");
    msg.push_str(input.body.as_str());
    msg.push_str("\r\n");

    for attachment in input.attachments {
        let encoded_data = base64::engine::general_purpose::STANDARD.encode(&attachment.data);
        msg.push_str(&format!("--{}\r\n", boundary));
        msg.push_str(&format!(
            "Content-Type: {}; name=\"{}\"\r\n",
            attachment.content_type, attachment.filename
        ));
        msg.push_str(&format!(
            "Content-Disposition: attachment; filename=\"{}\"\r\n",
            attachment.filename
        ));
        msg.push_str("Content-Transfer-Encoding: base64\r\n\r\n");
        for chunk in encoded_data.as_bytes().chunks(76) {
            msg.push_str(&format!("{}\r\n", std::str::from_utf8(chunk).unwrap()));
        }
    }
    msg.push_str(&format!("--{}--\r\n", boundary));
    println!("message built {}", msg);
    msg
}

pub fn build_encoded_email_message(input: MessageInput) -> String {
    let message = build_email_message(input);
    encode_string_to_base64url(&message)
}

fn encode_string_to_base64url(input: &str) -> String {
    const CUSTOM_ENGINE: engine::GeneralPurpose =
        engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
    CUSTOM_ENGINE.encode(input)
}
