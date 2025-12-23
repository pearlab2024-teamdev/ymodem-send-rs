# ymodem-sender-rs

>[!note]
> このブランチはシリアル通信のみだったものを拡張し、tokio::AsyncRead/tokioAsyncWriteを実装しているものであれば通信可能にしたものである。


## 概要

ymodemにしたがってデータを送信するcrate。

非同期通信制御と同期通信制御の2つを持ち、デフォルトでは同期通信となる。

非同期通信機能を使用する場合は以下のように指定すること。
`feature`指定がなければ、動機通信機能のみ有効となる。

```toml
ymodem-send-rs = { git = "https://github.com/PEARLabo/ymodem-send-rs", features= ["async"]}
```
