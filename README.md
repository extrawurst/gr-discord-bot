# gameroasters discord bot

company internal discord bot: [extrawurst/gr-discord-bot](https://hub.docker.com/repository/docker/extrawurst/gr-discord-bot)

![demo](demo.gif)

## how to run

follow the setup instructions here: https://developers.facebook.com/blog/post/2020/09/30/build-discord-bot-with-rust-and-serenity/

```sh
docker run -e DISCORD_TOKEN=<TOKEN> extrawurst/gr-discord-bot
```

then the bot can be triggered in any channel it has access to:

`!help` will answer:

```
try:
!help
!standup
```

`!standup` is used in our standup channel and prints:

```
-------------------------------------
👌            04.10.2020            👌
-------------------------------------
```
