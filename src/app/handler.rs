extern crate ws;

use self::ws::ErrorKind;
use self::ws::{listen, CloseCode, Handler, Handshake, Message, Request, Response, Result, Sender};
use app::message_manager;
use std::borrow::Cow;
use std::boxed::Box;
use std::str;
use std::sync::mpsc::Sender as SysSender;

struct WsHandler {
    out: Sender,
    login: String,
    system: SysSender<message_manager::MessageContainer>,
}

impl Handler for WsHandler {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        let messagestring = try!(msg.as_text().map(|i| i.to_string()));
        let meta = message_manager::MessageMeta {
            name: self.login.clone(),
            room: "first_room".to_string(),
        };
        let system_message = message_manager::MessageContainer {
            meta: meta,
            message: message_manager::Content::Message(messagestring),
        };
        match self.system.send(system_message) {
            Ok(_) => return Ok(()),
            Err(errval) => {
                return Err(ws::Error {
                    kind: ErrorKind::Custom(Box::new(errval)),
                    details: Cow::from("some shit happens".to_string()),
                })
            }
        }
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!(
            "WebSocket closing for ({:?}) {}, {}",
            code, reason, self.login
        );
        let meta = message_manager::MessageMeta {
            name: self.login.clone(),
            room: "first_room".to_string(),
        };
        let system_message = message_manager::MessageContainer {
            meta: meta,
            message: message_manager::Content::Close,
        };
        self.system.send(system_message).expect("can't send message");
    }

    fn on_request(&mut self, req: &Request) -> Result<Response> {
        let mut login_found = false;
        for i in req.headers() {
            if i.0 != "login" {
                continue;
            };
            let res = match str::from_utf8(i.1.as_slice()) {
                Err(_) => panic!("fucking error"),
                Ok(some) => some.to_string(),
            };
            self.login = res.clone();
            login_found = true;
        }
        if !login_found {
            println!("Connection refused");
            let mut resp: Response = Response::from_request(req).unwrap();
            resp.set_status(403);
            resp.set_reason("Not logged in, bitch!");
            return Ok(resp);
        }
        Response::from_request(req)
    }

    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        if let Some(addr) = try!(shake.remote_addr()) {
            println!("Connection with {} now open", addr);
        }
        println!("Connection login {:?}", self.login);
        let meta = message_manager::MessageMeta {
            name: self.login.clone(),
            room: "first_room".to_string(),
        };
        let system_message = message_manager::MessageContainer {
            meta: meta,
            message: message_manager::Content::Start(self.out.clone()),
        };
        match self.system.send(system_message) {
            Ok(_) => return Ok(()),
            Err(errval) => {
                return Err(ws::Error {
                    kind: ErrorKind::Custom(Box::new(errval)),
                    details: Cow::from("some shit happens".to_string()),
                })
            }
        }
    }
}

pub fn start(addres: String, sender: SysSender<message_manager::MessageContainer>) -> Result<()> {
    listen(addres, |out| WsHandler {
        out: out,
        login: "".to_string(),
        system: sender.clone(),
    })
}
