mod config;
use futures::{executor::block_on, TryStreamExt};

use egg_mode::{stream::StreamMessage, KeyPair, Token};
use teloxide::prelude::*;
#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    let config = config::Config::from_env().unwrap();
    let bot = Bot::from_env();
    println!("Ctrl-C to quit\n");

    let holo_member: &[u64] = &[
        1283657064410017793, // gura
        997786053124616192,  //fubuki
        996645451045617664,  //matsuri
    ];

    bot.send_message(-1001288036225, "Hello world!")
        .send()
        .await
        .unwrap();

    let stream = egg_mode::stream::filter()
        .follow(holo_member)
        .start(&Token::Access {
            consumer: KeyPair::new(config.api_key, config.api_secret),
            access: KeyPair::new(config.access_token, config.access_secret),
        })
        .try_for_each(|m| {
            if let StreamMessage::Tweet(tweet) = m {
                match &tweet.user {
                    Some(user) => {
                        if holo_member.contains(&user.id) {
                            block_on(
                                bot.send_message(-1001288036225, format!("{}", tweet.text))
                                    .send(),
                            )
                            .unwrap();
                            print!("{:?}", tweet)
                        }
                    }
                    None => (),
                }
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
