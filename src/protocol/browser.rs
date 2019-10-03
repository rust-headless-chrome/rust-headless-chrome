use serde::{Deserialize, Serialize};

use crate::protocol::JsUInt;

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
        width: Option<JsUInt>,
        /// THe window height in pixels.
        height: Option<JsUInt>,
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

impl Into<methods::Bounds> for Bounds {
    fn into(self) -> methods::Bounds {
        match self {
            Self::Minimized => methods::Bounds {
                left: None,
                top: None,
                width: None,
                height: None,
                window_state: WindowState::Minimized,
            },
            Self::Maximized => methods::Bounds {
                left: None,
                top: None,
                width: None,
                height: None,
                window_state: WindowState::Maximized,
            },
            Self::Fullscreen => methods::Bounds {
                left: None,
                top: None,
                width: None,
                height: None,
                window_state: WindowState::Fullscreen,
            },
            Self::Normal {
                left,
                top,
                width,
                height,
            } => methods::Bounds {
                left,
                top,
                width,
                height,
                window_state: WindowState::Normal,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
}

#[derive(Clone, Debug)]
pub struct CurrentBounds {
    pub left: JsUInt,
    pub top: JsUInt,
    pub width: JsUInt,
    pub height: JsUInt,
    pub state: WindowState,
}

impl From<methods::Bounds> for CurrentBounds {
    fn from(bounds: methods::Bounds) -> Self {
        Self {
            left: bounds.left.unwrap(),
            top: bounds.top.unwrap(),
            width: bounds.width.unwrap(),
            height: bounds.height.unwrap(),
            state: bounds.window_state,
        }
    }
}

pub mod methods {
    use crate::protocol::Method;

    use super::{Deserialize, JsUInt, Serialize, WindowState};

    #[derive(Serialize, Debug)]
    pub struct GetVersion {}

    #[derive(Deserialize, Clone, Debug)]
    #[serde(rename_all = "camelCase")]
    /// Version information returned by `Browser.getVersion`
    pub struct VersionInformationReturnObject {
        /// Protocol version
        pub protocol_version: String,
        /// Product version
        pub product: String,
        /// Product revision
        pub revision: String,
        /// User-Agent
        pub user_agent: String,
        /// V8 version.
        pub js_version: String,
    }

    impl Method for GetVersion {
        const NAME: &'static str = "Browser.getVersion";
        type ReturnObject = VersionInformationReturnObject;
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Bounds {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub left: Option<JsUInt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub top: Option<JsUInt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub width: Option<JsUInt>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub height: Option<JsUInt>,
        pub window_state: WindowState,
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SetWindowBounds {
        pub window_id: crate::protocol::WindowId,
        pub bounds: Bounds,
    }

    #[derive(Debug, Deserialize)]
    pub struct SetWindowBoundsReturnObject {}

    impl Method for SetWindowBounds {
        const NAME: &'static str = "Browser.setWindowBounds";
        type ReturnObject = SetWindowBoundsReturnObject;
    }

    #[derive(Debug, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GetWindowForTarget<'a> {
        pub target_id: &'a crate::protocol::target::TargetId,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct GetWindowForTargetReturnObject {
        pub window_id: crate::protocol::WindowId,
        pub bounds: crate::protocol::browser::methods::Bounds,
    }

    impl<'a> Method for GetWindowForTarget<'a> {
        const NAME: &'static str = "Browser.getWindowForTarget";
        type ReturnObject = GetWindowForTargetReturnObject;
    }
}
