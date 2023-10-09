use crate::protocol::cdp::{
    types::{Event, JsUInt},
    Browser,
    Network::{CookieParam, DeleteCookies},
    Page,
    Page::PrintToPDF,
    DOM::Node,
};

use serde::{Deserialize, Serialize};

use serde_json::Value;

pub type CallId = JsUInt;

use thiserror::Error;

use anyhow::Result;

type JsInt = i32;

#[derive(Deserialize, Debug, PartialEq, Clone, Error)]
#[error("Method call error {}: {}", code, message)]
pub struct RemoteError {
    pub code: JsInt,
    pub message: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct Response {
    #[serde(rename(deserialize = "id"))]
    pub call_id: CallId,
    pub result: Option<Value>,
    pub error: Option<RemoteError>,
}

pub fn parse_response<T>(response: Response) -> Result<T>
where
    T: serde::de::DeserializeOwned + std::fmt::Debug,
{
    if let Some(error) = response.error {
        return Err(error.into());
    }

    let result: T = serde_json::from_value(response.result.unwrap())?;

    Ok(result)
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Message {
    Event(Event),
    Response(Response),
    ConnectionShutdown,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransferMode {
    mode: String,
}

impl From<TransferMode> for Option<Page::PrintToPDFTransfer_modeOption> {
    fn from(val: TransferMode) -> Self {
        if val.mode == "base64" {
            Some(Page::PrintToPDFTransfer_modeOption::ReturnAsBase64)
        } else if val.mode == "stream" {
            Some(Page::PrintToPDFTransfer_modeOption::ReturnAsStream)
        } else {
            None
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PrintToPdfOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landscape: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_header_footer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_top: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_bottom: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_left: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_right: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_ranges: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_invalid_page_ranges: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_css_page_size: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_mode: Option<TransferMode>,
}

pub fn parse_raw_message(raw_message: &str) -> Result<Message> {
    Ok(serde_json::from_str::<Message>(raw_message)?)
}

#[derive(Clone, Debug)]
pub enum Bounds {
    Minimized,
    Maximized,
    Fullscreen,
    Normal {
        /// The offset from the left edge of the screen to the window in pixels.
        left: Option<JsUInt>,
        /// The offset from the top edge of the screen to the window in pixels.
        top: Option<JsUInt>,
        /// The window width in pixels.
        width: Option<f64>,
        /// THe window height in pixels.
        height: Option<f64>,
    },
}

impl Bounds {
    /// Set normal window state without setting any coordinates
    pub fn normal() -> Self {
        Self::Normal {
            left: None,
            top: None,
            width: None,
            height: None,
        }
    }
}

impl From<CookieParam> for DeleteCookies {
    fn from(v: CookieParam) -> Self {
        Self {
            name: v.name,
            url: v.url,
            domain: v.domain,
            path: v.path,
        }
    }
}

impl From<Bounds> for Browser::Bounds {
    fn from(val: Bounds) -> Self {
        match val {
            Bounds::Minimized => Browser::Bounds {
                left: None,
                top: None,
                width: None,
                height: None,
                window_state: Some(Browser::WindowState::Minimized),
            },
            Bounds::Maximized => Browser::Bounds {
                left: None,
                top: None,
                width: None,
                height: None,
                window_state: Some(Browser::WindowState::Maximized),
            },
            Bounds::Fullscreen => Browser::Bounds {
                left: None,
                top: None,
                width: None,
                height: None,
                window_state: Some(Browser::WindowState::Fullscreen),
            },
            Bounds::Normal {
                left,
                top,
                width,
                height,
            } => Browser::Bounds {
                left,
                top,
                width: width.map(|f| f as u32),
                height: height.map(|f| f as u32),
                window_state: Some(Browser::WindowState::Normal),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct CurrentBounds {
    pub left: JsUInt,
    pub top: JsUInt,
    pub width: f64,
    pub height: f64,
    pub state: Browser::WindowState,
}

impl From<Browser::Bounds> for CurrentBounds {
    fn from(bounds: Browser::Bounds) -> Self {
        Self {
            left: bounds.left.unwrap(),
            top: bounds.top.unwrap(),
            width: f64::from(bounds.width.unwrap()),
            height: f64::from(bounds.height.unwrap()),
            state: bounds.window_state.unwrap(),
        }
    }
}

impl Default for PrintToPDF {
    fn default() -> Self {
        PrintToPDF {
            display_header_footer: None,
            footer_template: None,
            header_template: None,
            ignore_invalid_page_ranges: None,
            landscape: None,
            margin_bottom: None,
            margin_left: None,
            margin_right: None,
            margin_top: None,
            page_ranges: None,
            paper_height: None,
            paper_width: None,
            prefer_css_page_size: None,
            print_background: None,
            scale: None,
            transfer_mode: None,
        }
    }
}

struct SearchVisitor<'a, F> {
    predicate: F,
    item: Option<&'a Node>,
}

impl<'a, F: FnMut(&Node) -> bool> SearchVisitor<'a, F> {
    fn new(predicate: F) -> Self {
        SearchVisitor {
            predicate,
            item: None,
        }
    }

    fn visit(&mut self, n: &'a Node) {
        if (self.predicate)(n) {
            self.item = Some(n);
        } else if self.item.is_none() {
            if let Some(c) = &n.children {
                c.iter().for_each(|n| self.visit(n));
            }
        }
    }
}

impl Node {
    /// Returns the first node for which the given closure returns true.
    ///
    /// Nodes are inspected breadth-first down their children.
    pub fn find<F: FnMut(&Self) -> bool>(&self, predicate: F) -> Option<&Self> {
        let mut s = SearchVisitor::new(predicate);
        s.visit(self);
        s.item
    }
}

#[cfg(test)]
mod tests {
    use log::trace;
    use serde_json::json;

    use super::*;

    #[test]
    fn pass_through_channel() {
        env_logger::try_init().unwrap_or(());

        let attached_to_target_json = json!({
            "method": "Target.attachedToTarget",
            "params": {
                "sessionId": "8BEF122ABAB0C43B5729585A537F424A",
                "targetInfo": {
                    "targetId": "26DEBCB2A45BEFC67A84012AC32C8B2A",
                    "type": "page",
                    "title": "",
                    "url": "about:blank",
                    "attached": true,
                    "browserContextId": "946423F3D201EFA1A5FCF3462E340C15"
                },
                "waitingForDebugger": false
            }
        });

        let _event: Message = serde_json::from_value(attached_to_target_json).unwrap();
    }

    #[test]
    fn parse_event_fully() {
        env_logger::try_init().unwrap_or(());

        let attached_to_target_json = json!({
            "method": "Target.attachedToTarget",
            "params": {
                "sessionId": "8BEF122ABAB0C43B5729585A537F424A",
                "targetInfo": {
                    "targetId": "26DEBCB2A45BEFC67A84012AC32C8B2A",
                    "type": "page",
                    "title": "",
                    "url": "about:blank",
                    "attached": true,
                    "browserContextId": "946423F3D201EFA1A5FCF3462E340C15"
                },
                "waitingForDebugger": false
            }
        });

        if let Ok(Event::AttachedToTarget(_)) = serde_json::from_value(attached_to_target_json) {
        } else {
            panic!("Failed to parse event properly");
        }

        let received_target_msg_event = json!({
            "method": "Target.receivedMessageFromTarget",
            "params": {
                "sessionId": "8BEF122ABAB0C43B5729585A537F424A",
                "message": "{\"id\":43473,\"result\":{\"data\":\"kDEgAABII=\"}}",
                "targetId": "26DEBCB2A45BEFC67A84012AC32C8B2A"
            }
        });
        let event: Event = serde_json::from_value(received_target_msg_event).unwrap();
        match event {
            Event::ReceivedMessageFromTarget(ev) => {
                trace!("{:?}", ev);
            }
            _ => panic!("bad news"),
        }
    }

    #[test]
    fn easy_parse_messages() {
        env_logger::try_init().unwrap_or(());

        let example_message_strings = [
            // browser method response:
            "{\"id\":1,\"result\":{\"browserContextIds\":[\"C2652EACAAA12B41038F1F2137C57A6E\"]}}",
            "{\"id\":2,\"result\":{\"targetInfos\":[{\"targetId\":\"225A1B90036320AB4DB2E28F04AA6EE0\",\"type\":\"page\",\"title\":\"\",\"url\":\"about:blank\",\"attached\":false,\"browserContextId\":\"04FB807A65CFCA420C03E1134EB9214E\"}]}}",
            "{\"id\":3,\"result\":{}}",
            // browser event:
            "{\"method\":\"Target.attachedToTarget\",\"params\":{\"sessionId\":\"8BEF122ABAB0C43B5729585A537F424A\",\"targetInfo\":{\"targetId\":\"26DEBCB2A45BEFC67A84012AC32C8B2A\",\"type\":\"page\",\"title\":\"\",\"url\":\"about:blank\",\"attached\":true,\"browserContextId\":\"946423F3D201EFA1A5FCF3462E340C15\"},\"waitingForDebugger\":false}}",
            // browser event which indicates target method response:
            "{\"method\":\"Target.receivedMessageFromTarget\",\"params\":{\"sessionId\":\"8BEF122ABAB0C43B5729585A537F424A\",\"message\":\"{\\\"id\\\":43473,\\\"result\\\":{\\\"data\\\":\\\"iVBORw0KGgoAAAANSUhEUgAAAyAAAAJYCAYAAACadoJwAAAMa0lEQVR4nO3XMQEAIAzAMMC/5+GiHCQK+nbPzCwAAIDAeR0AAAD8w4AAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABAxoAAAAAZAwIAAGQMCAAAkDEgAABII=\\\"}}\",\"targetId\":\"26DEBCB2A45BEFC67A84012AC32C8B2A\"}}"
        ];

        for msg_string in &example_message_strings {
            let _message: super::Message = parse_raw_message(msg_string).unwrap();
        }
    }
}
