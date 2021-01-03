mod config;
use futures::TryStreamExt;

use egg_mode::{stream::StreamMessage, KeyPair, Token};

#[tokio::main]
async fn main() {
    let config = config::Config::from_env().unwrap();
    println!("Ctrl-C to quit\n");

    let stream = egg_mode::stream::filter()
        .follow(&[1283657064410017793])
        .start(&Token::Access {
            consumer: KeyPair::new(config.api_key, config.api_secret),
            access: KeyPair::new(config.access_token, config.access_secret),
        })
        .try_for_each(|m| {
            if let StreamMessage::Tweet(tweet) = m {
                print!("{:?}\n", tweet)
            } else {
                println!("{:?}", m);
            }
            futures::future::ok(())
        });
    if let Err(e) = stream.await {
        println!("Stream error: {}", e);
        println!("Disconnected")
    }
}
