
use futures::Stream;
use tokio_core::reactor::{Core, Handle};
use telegram_bot;
use telegram_bot::*;

pub struct Bot {
    api:        telegram_bot::Api,
}

pub fn do_cmd(cmd: &str, api: Api, message: Message, _handle: &Handle) {
    match cmd {
        "/price" => api.spawn(message.text_reply( format!("price!") )),
        _ => ()
    }
}

pub fn build(token: &str) -> Result<(), telegram_bot::Error> {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let api = Api::configure(token).build(core.handle())?;
    // Fetch new updates via long poll method
    let future = api.stream().for_each(|update| {

        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {

            if let MessageKind::Text {ref data, ..} = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                do_cmd(data.as_str(), api.clone(), message.clone(), &handle);

                // Answer message with "Hi".
                api.spawn(message.text_reply(
                    format!("Hi, {}! You just wrote '{}'", &message.from.first_name, data)
                ));
            }
        }

        Ok(())
    });

    Ok(core.run(future)?)
}
