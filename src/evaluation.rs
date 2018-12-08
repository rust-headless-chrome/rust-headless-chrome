use super::errors::*;
use super::chrome::{Chrome};
use cdp;
use cdp::{SerializeCdpCommand};


fn evaluate_monkey() -> Result<()> {
//    env_logger::init();
    let chrome = &mut Chrome::new()?;

    let response = chrome.call_method::<cdp::target::GetTargetsResponse>(&cdp::target::GetTargetsCommand {})?;

    let default_target = &response.target_infos[0];
    let response: cdp::target::AttachToTargetResponse = chrome.call_method(&cdp::target::AttachToTargetCommand {
        target_id: default_target.target_id.clone(),
        flatten: Some(false)
    })?;
    let session_id = response.session_id;

    let comm = &cdp::page::EnableCommand {};
    let method = json!({"method": comm.command_name(), "id":9999, "params": comm});
    let message_str = serde_json::to_string(&method).unwrap();
    trace!("sending message: {:#?}", &message_str);


    let _response: cdp::target::SendMessageToTargetResponse = chrome.call_method(&cdp::target::SendMessageToTargetCommand {
        message: std::borrow::Cow::Borrowed(&message_str),
        target_id: Some(default_target.target_id.clone()),
        session_id: Some(session_id.clone()),
    })?;
    eprintln!("{:#?}", _response);
    let comm = cdp::page::NavigateCommand {
        url: std::borrow::Cow::Borrowed("https://wikipedia.org"),
        referrer: None,
        transition_type: None,
        frame_id: None
    };
    let method = json!({"method": comm.command_name(), "id":99999, "params": comm});
    let message_str = serde_json::to_string(&method).unwrap();
    trace!("sending message: {:#?}", &message_str);
    let _response: cdp::target::SendMessageToTargetResponse = chrome.call_method(&cdp::target::SendMessageToTargetCommand {
        message: std::borrow::Cow::Borrowed(&message_str),
        target_id: Some(default_target.target_id.clone()),
        session_id: Some(session_id.clone()),
    })?;

    return Ok(());
}

// core loop: while true, get batch, run batch, output results
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _ = super::evaluate_monkey().unwrap();
        println!("asdf");
    }
}
