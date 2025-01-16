use anyhow::Result;

pub fn log_send(kind: &str, name: &str, tag: &str, msg: &str, err: Result<()>) {
    let msg = if msg.is_empty() { "  " } else { msg };

    match err {
        Ok(()) => log::info!(
            "[{} / {} / {}] - {} - successfully sent!",
            kind,
            name,
            tag,
            msg
        ),
        Err(err) => log::error!(
            "[{} / {} / {}] - {}({}) - failed to send! ",
            kind,
            name,
            tag,
            msg,
            err,
        ),
    }
}
