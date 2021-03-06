use std;
use rmps;

// -------------------------------------------------------------- //
// This file is copy-pasted from the synac common repository.     //
// Reason is because crates.io doesn't want to publish crates     //
// with a git repository.                                         //
// And common doesn't deserve it's own repo.                      //
// -------------------------------------------------------------- //

use std::collections::HashMap;
use std::io;

pub const DEFAULT_PORT:   u16 = 8439;
pub const RSA_LENGTH:     u32 = 3072;
pub const TYPING_TIMEOUT: u8  = 10;

pub const LIMIT_USER_NAME:    usize = 128;
pub const LIMIT_CHANNEL_NAME: usize = 128;
pub const LIMIT_MESSAGE:      usize = 16384;

pub const LIMIT_BULK:         usize = 64;

pub const ERR_ALREADY_EXISTS:     u8 = 0;
pub const ERR_LIMIT_REACHED:      u8 = 1;
pub const ERR_LOGIN_BANNED:       u8 = 2;
pub const ERR_LOGIN_BOT:          u8 = 3;
pub const ERR_LOGIN_INVALID:      u8 = 4;
pub const ERR_MAX_CONN_PER_IP:    u8 = 5;
pub const ERR_MISSING_FIELD:      u8 = 6;
pub const ERR_MISSING_PERMISSION: u8 = 7;
pub const ERR_SELF_PM:            u8 = 8;
pub const ERR_UNKNOWN_BOT:        u8 = 9;
pub const ERR_UNKNOWN_CHANNEL:    u8 = 10;
pub const ERR_UNKNOWN_MESSAGE:    u8 = 11;
pub const ERR_UNKNOWN_USER:       u8 = 12;

pub const PERM_READ:              u8 = 1;
pub const PERM_WRITE:             u8 = 1 << 1;

pub const PERM_MANAGE_CHANNELS:   u8 = 1 << 2;
pub const PERM_MANAGE_MESSAGES:   u8 = 1 << 3;
pub const PERM_MANAGE_MODES:      u8 = 1 << 4;

pub const PERM_ALL: u8 = PERM_READ | PERM_WRITE | PERM_MANAGE_CHANNELS | PERM_MANAGE_MESSAGES | PERM_MANAGE_MODES;

// TYPES
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Channel {
    pub default_mode_bot:  u8,
    pub default_mode_user: u8,
    pub id: usize,
    pub name: String,
    pub private: bool
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Message {
    pub author: usize,
    pub channel: usize,
    pub id: usize,
    pub text: Vec<u8>,
    pub timestamp: i64,
    pub timestamp_edit: Option<i64>
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct User {
    pub admin: bool,
    pub ban: bool,
    pub bot: bool,
    pub id: usize,
    pub modes: HashMap<usize, u8>,
    pub name: String
}

// CLIENT PACKETS
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ChannelCreate {
    pub default_mode_bot:  u8,
    pub default_mode_user: u8,
    pub name: String,
    pub recipient: Option<usize>
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ChannelDelete {
    pub id: usize
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ChannelUpdate {
    pub inner: Channel
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Command {
    pub args: Vec<String>,
    pub recipient: usize
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Login {
    pub bot: bool,
    pub name: String,
    pub password: Option<String>,
    pub token: Option<String>
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoginUpdate {
    pub name: Option<String>,
    pub password_current: Option<String>,
    pub password_new: Option<String>,
    pub reset_token: bool
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MessageCreate {
    pub channel: usize,
    pub text: Vec<u8>
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MessageDelete {
    pub id: usize
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MessageDeleteBulk {
    pub channel: usize,
    pub ids: Vec<usize>
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MessageList {
    pub after: Option<usize>,
    pub before: Option<usize>,
    pub channel: usize,
    pub limit: usize
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MessageUpdate {
    pub id: usize,
    pub text: Vec<u8>
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Typing {
    pub channel: usize
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserUpdate {
    pub admin: Option<bool>,
    pub ban: Option<bool>,
    pub channel_mode: Option<(usize, Option<u8>)>,
    pub id: usize
}

// SERVER PACKETS
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ChannelDeleteReceive {
    pub inner: Channel
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ChannelReceive {
    pub inner: Channel
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CommandReceive {
    pub args: Vec<String>,
    pub author: usize
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoginSuccess {
    pub created: bool,
    pub id: usize,
    pub token: String
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MessageDeleteReceive {
    pub id: usize
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MessageReceive {
    pub inner: Message,
    pub new: bool
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TypingReceive {
    pub author: usize,
    pub channel: usize
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UserReceive {
    pub inner: User
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Packet {
    /// an error was received. see ERR_* variables.
    Err(u8),
    /// you are ratelimited for X seconds
    RateLimited(u64),

    /// create a new channel
    ChannelCreate(ChannelCreate),
    /// delete a channel
    ChannelDelete(ChannelDelete),
    /// edit a channel
    ChannelUpdate(ChannelUpdate),
    /// send a bot command
    Command(Command),
    /// actually log in. this is required before anything else.
    Login(Login),
    /// update your login credentials. reset_token is ignored and treated as true if password is set.
    LoginUpdate(LoginUpdate),
    /// send a new message
    MessageCreate(MessageCreate),
    /// delete a message
    MessageDelete(MessageDelete),
    /// delete a bunch of messages
    MessageDeleteBulk(MessageDeleteBulk),
    /// list `limit` most recent messages, optionally before/after a value
    MessageList(MessageList),
    /// update a message
    MessageUpdate(MessageUpdate),
    /// send a typing indicator. timeouts after TYPING_TIMEOUT seconds.
    Typing(Typing),
    /// update a user (for login info, see LoginUpdate)
    UserUpdate(UserUpdate),

    /// a channel was deleted
    ChannelDeleteReceive(ChannelDeleteReceive),
    /// a channel was created/edited/initially sent
    ChannelReceive(ChannelReceive),
    /// a command was received (bot only)
    CommandReceive(CommandReceive),
    /// login was successful. save the token and use that next time.
    LoginSuccess(LoginSuccess),
    /// a message was deleted
    MessageDeleteReceive(MessageDeleteReceive),
    /// a message list operation was finished
    MessageListReceived,
    /// a message was created/edited/initially sent
    MessageReceive(MessageReceive),
    /// a typing event was received. timeout after TYPING_TIMEOUT seconds.
    TypingReceive(TypingReceive),
    /// a user was created/edited
    UserReceive(UserReceive)
}

pub fn serialize(packet: &Packet) -> Result<Vec<u8>, rmps::encode::Error> {
    rmps::to_vec(&packet)
}
pub fn deserialize(buf: &[u8]) -> Result<Packet, rmps::decode::Error> {
    rmps::from_slice(buf)
}
pub fn deserialize_stream<T: io::Read>(buf: T) -> Result<Packet, rmps::decode::Error> {
    rmps::from_read(buf)
}
pub fn encode_u16(input: u16) -> [u8; 2] {
    [
        (input >> 8)  as u8,
        (input % 256) as u8
    ]
}
pub fn decode_u16(bytes: &[u8]) -> u16 {
    assert_eq!(bytes.len(), 2);

    ((bytes[0] as u16) << 8) + bytes[1] as u16
}

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    DecodeError(#[cause] rmps::decode::Error),
    #[fail(display = "{}", _0)]
    EncodeError(#[cause] rmps::encode::Error),
    #[fail(display = "{}", _0)]
    IoError(#[cause] std::io::Error),

    #[fail(display = "Packet size must fit in an u16")]
    PacketTooBigError
}

impl From<rmps::decode::Error> for Error {
    fn from(err: rmps::decode::Error) -> Self {
        Error::DecodeError(err)
    }
}
impl From<rmps::encode::Error> for Error {
    fn from(err: rmps::encode::Error) -> Self {
        Error::EncodeError(err)
    }
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

pub fn read<T: io::Read>(reader: &mut T) -> Result<Packet, Error> {
    let mut buf = [0; 2];
    reader.read_exact(&mut buf)?;

    let size = decode_u16(&buf) as usize;
    let mut buf = vec![0; size];
    reader.read_exact(&mut buf)?;

    Ok(deserialize(&buf)?)
}
pub fn write<T: io::Write>(writer: &mut T, packet: &Packet) -> Result<(), Error> {
    let buf = serialize(packet)?;
    if buf.len() > std::u16::MAX as usize {
        return Err(Error::PacketTooBigError);
    }
    let size = encode_u16(buf.len() as u16);
    writer.write_all(&size)?;
    writer.write_all(&buf)?;
    writer.flush()?;

    Ok(())
}
