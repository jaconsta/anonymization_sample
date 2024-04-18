use std::collections::HashMap;
use std::str;

use crate::lib::event_based::sample_events::sample_cyphers::decrypt_message;

// Event type
#[derive(PartialEq, Debug, Clone)]
pub enum ET {
    S(String),
    I(usize),
}

impl Default for ET {
    fn default() -> Self {
        ET::S(String::from(""))
    }
}

impl ET {
    pub fn as_string(&self) -> String {
        match self {
            ET::S(s) => String::from(s),
            ET::I(s) => format!("{}", s),
        }
    }
}

pub fn event_stream<'a>() -> Vec<HashMap<&'a str, ET>> {
    let mut events: Vec<HashMap<&'a str, ET>> = Vec::new();

    let aes_key_lead_id_12: [u8; 32] = [
        249, 47, 138, 163, 124, 118, 232, 4, 165, 46, 211, 212, 83, 254, 103, 73, 143, 125, 122,
        141, 176, 2, 14, 135, 189, 190, 180, 80, 0, 14, 114, 20,
    ];

    // Event 1
    // Uncomment to re-generate the cyphers. Represent the database insertion.
    // let msg = encrypt_message(&aes_key_lead_id_12, &event_nonce, "Casey");
    // println!("Casey: {:?}", msg);
    // let msg = encrypt_message(&aes_key_lead_id_12, &event_nonce, "David");
    // println!("David: {:?}", msg);
    // let msg = encrypt_message(&aes_key_lead_id_12, &event_nonce, "444-2951");
    // println!("444-2951: {:?}", msg);
    // println!("");
    let event_nonce = "9ff9-4887-b6c";
    let cypher = [
        54, 236, 139, 228, 114, 103, 102, 20, 126, 116, 102, 78, 196, 240, 208,
    ];
    let first_name = decrypt_message(&aes_key_lead_id_12, &event_nonce, &cypher);
    let first_name = str::from_utf8(&first_name).unwrap();
    let cypher = [
        49, 236, 142, 232, 111, 213, 146, 132, 124, 46, 139, 152, 71, 76, 187,
    ];
    let last_name = decrypt_message(&aes_key_lead_id_12, &event_nonce, &cypher);
    let last_name = str::from_utf8(&last_name).unwrap();
    let cypher = [
        65, 185, 204, 172, 57, 146, 138, 192, 85, 136, 162, 46, 157, 210, 108, 94, 215, 217,
    ];
    let phone_number = decrypt_message(&aes_key_lead_id_12, &event_nonce, &cypher);
    let phone_number = str::from_utf8(&phone_number).unwrap();
    let event: HashMap<&'a str, ET> = HashMap::from([
        ("lead-id", ET::I(12)),
        ("event-id", ET::I(0)),
        ("event-type", ET::S(String::from("lead-initialized"))),
        ("first-name", ET::S(String::from(first_name))),
        ("last-name", ET::S(String::from(last_name))),
        ("phone-number", ET::S(String::from(phone_number))),
        ("timestamp", ET::S(String::from("2020-05-20T09:52:55.95Z"))),
    ]);
    events.push(event);

    let event = HashMap::from([
        ("lead-id", ET::I(12)),
        ("event-id", ET::I(1)),
        ("event-type", ET::S(String::from("contacted"))),
        ("timestamp", ET::S(String::from("2020-05-20T12:32:08.24Z"))),
    ]);
    events.push(event);

    let event = HashMap::from([
        ("lead-id", ET::I(12)),
        ("event-id", ET::I(2)),
        ("event-type", ET::S(String::from("followup-set"))),
        (
            "followup-on",
            ET::S(String::from("2020-05-27T12:00:00.00Z")),
        ),
        ("timestamp", ET::S(String::from("2020-05-20T12:32:08.24Z"))),
    ]);

    let event_nonce = "ede4-41ef-bd2";
    // Uncomment to re-generate the cyphers. Represent the database insertion.
    // let msg = encrypt_message(&aes_key_lead_id_12, &event_nonce, "Casey");
    // println!("Casey: {:?}", &msg);
    // let msg = msg.iter().map(|x| x.to_string()).collect::<Vec<String>>();
    // let msg = msg.join::<&str>(",");
    // println!("Casey (as str): {}", &msg);
    // let msg = encrypt_message(&aes_key_lead_id_12, &event_nonce, "Davis");
    // let msg = str::from_utf8(&msg).unwrap();
    // println!("Davis: {:?}", msg);
    // let msg = encrypt_message(&aes_key_lead_id_12, &event_nonce, "555-8101");
    // let msg = str::from_utf8(&msg).unwrap();
    // println!("555-8101: {:?}", msg);
    //
    // let cypher = [
    //     213, 251, 189, 88, 55, 81, 88, 94, 223, 172, 193, 62, 12, 128, 231,
    // ];
    // Demonstration where the cypher is stringified
    let cypher = "213,251,189,88,55,81,88,94,223,172,193,62,12,128,231".split(",");
    let cypher: Vec<u8> = cypher
        .into_iter()
        .map(|n| n.parse::<u8>().unwrap())
        .collect();
    let first_name = decrypt_message(&aes_key_lead_id_12, &event_nonce, &cypher);
    let first_name = str::from_utf8(&first_name).unwrap();

    let cypher = [
        210, 251, 184, 84, 61, 229, 174, 175, 88, 221, 29, 48, 35, 219, 51,
    ];
    let last_name = decrypt_message(&aes_key_lead_id_12, &event_nonce, &cypher);
    let last_name = str::from_utf8(&last_name).unwrap();
    let cypher = [
        163, 175, 251, 16, 118, 188, 127, 21, 230, 154, 85, 34, 187, 246, 228, 207, 206, 87,
    ];
    let phone_number = decrypt_message(&aes_key_lead_id_12, &event_nonce, &cypher);
    let phone_number = str::from_utf8(&phone_number).unwrap();
    events.push(event);
    let event = HashMap::from([
        ("lead-id", ET::I(12)),
        ("event-id", ET::I(3)),
        ("event-type", ET::S(String::from("contact-details-updated"))),
        ("first-name", ET::S(String::from(first_name))),
        ("last-name", ET::S(String::from(last_name))),
        ("phone-number", ET::S(String::from(phone_number))),
        ("timestamp", ET::S(String::from("2020-05-20T12:32:08.24Z"))),
    ]);

    events.push(event);
    let event = HashMap::from([
        ("lead-id", ET::I(12)),
        ("event-id", ET::I(4)),
        ("event-type", ET::S(String::from("contacted"))),
        ("timestamp", ET::S(String::from("2020-05-27T12:02:12.51Z"))),
    ]);

    events.push(event);
    let event = HashMap::from([
        ("lead-id", ET::I(12)),
        ("event-id", ET::I(5)),
        ("event-type", ET::S(String::from("order-submitted"))),
        (
            "payment-deadline",
            ET::S(String::from("2020-05-30T12:02:12.51Z")),
        ),
        ("timestamp", ET::S(String::from("2020-05-27T12:02:12.51Z"))),
    ]);

    events.push(event);
    let event = HashMap::from([
        ("lead-id", ET::I(12)),
        ("event-id", ET::I(6)),
        ("event-type", ET::S(String::from("payment-confirmed"))),
        ("status", ET::S(String::from("converted"))),
        ("timestamp", ET::S(String::from("2020-05-27T12:38:44.12Z"))),
    ]);
    events.push(event);

    return events;
}

pub mod sample_cyphers {
    use aes::Aes256;
    use ccm::{
        aead::{generic_array::GenericArray, Aead, KeyInit, OsRng},
        consts::{U10, U13},
        Ccm,
    };
    use std::str;

    pub fn encrypt_message(aes_key: &[u8; 32], nonce_bytes: &str, message: &str) -> Vec<u8> {
        pub type Aes256Ccm = Ccm<Aes256, U10, U13>;
        let nonce = GenericArray::from_slice(nonce_bytes.as_bytes()); // 13-bytes; unique per message
        let key = Aes256Ccm::new_from_slice(aes_key).unwrap();

        let cypherbytes = key.encrypt(nonce, message.as_ref()).unwrap();
        cypherbytes
    }
    pub fn decrypt_message(aes_key: &[u8; 32], nonce_bytes: &str, cypher: &[u8]) -> Vec<u8> {
        pub type Aes256Ccm = Ccm<Aes256, U10, U13>;
        let nonce = GenericArray::from_slice(nonce_bytes.as_bytes()); // 13-bytes; unique per message
        let key = Aes256Ccm::new_from_slice(aes_key).unwrap();

        let cyphertext = key.decrypt(nonce, cypher.as_ref()).unwrap();
        cyphertext
    }

    fn always_new_key() {
        // AES-256-CCM type with tag and nonce size equal to 10 and 13 bytes respectively
        pub type Aes256Ccm = Ccm<Aes256, U10, U13>;

        let key = Aes256Ccm::generate_key(&mut OsRng);
        // println!("{:?}", &key.as_slice());
        let cipher = Aes256Ccm::new(&key);
        let nonce = GenericArray::from_slice(b"unique nonce."); // 13-bytes; unique per message

        let ciphertext = cipher
            .encrypt(nonce, b"plaintext message".as_ref())
            .unwrap();
        let plaintext = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
        assert_eq!(&plaintext, b"plaintext message");
    }

    fn from_pre_generated_key() {
        println!("Load from a pre-generate key");
        pub type Aes256Ccm = Ccm<Aes256, U10, U13>;
        let nonce = GenericArray::from_slice(b"unique nonce."); // 13-bytes; unique per message

        let aes_key_slice1: [u8; 32] = [
            46, 19, 165, 132, 9, 66, 37, 191, 170, 49, 98, 68, 210, 129, 112, 184, 94, 122, 176,
            252, 235, 38, 20, 172, 73, 81, 97, 82, 162, 218, 68, 119,
        ];
        let key2 = Aes256Ccm::new_from_slice(&aes_key_slice1).unwrap();
        // let ciphertext = key2.encrypt(nonce, b"plaintext message".as_ref()).unwrap();

        let cipherbytes: Vec<u8> = vec![
            107, 114, 171, 71, 64, 151, 82, 102, 208, 37, 57, 255, 81, 77, 56, 165, 161, 220, 164,
            113, 50, 24, 127, 94, 70, 154, 102,
        ];
        let bytestext2 = key2.decrypt(nonce, cipherbytes.as_ref()).unwrap();
        let plaintext2 = str::from_utf8(&bytestext2).unwrap();
        assert_eq!(&bytestext2, b"plaintext message");
        assert_eq!(&plaintext2, &"plaintext message");
    }

    pub fn cypher_messages() {
        always_new_key();
        from_pre_generated_key();
    }
}
