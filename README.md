# icmp-packet-capture

icmp-packet-capture は、ネットワーク上を流れるパケットをキャプチャして、ICMP パケットの内容を解析・表示するためのツールです。

## 特徴

- Ethernet フレームの解析
- IPv4 パケットの抽出と処理
- ICMP パケットの内容表示
- エラー処理と詳細なログ出力によるデバッグ支援

## 必要条件

- Rust (最新版推奨)
- Linux 環境  
  ※ ネットワークデバイスに対するアクセスが必要なため、実行時に十分な権限が必要です。

## インストール

1. リポジトリをクローンします。

   ```sh
   git clone https://example.com/your-repo/icmp-packet-capture.git
   cd icmp-packet-capture
   ```

2. ビルドします。  

   ```sh
   cargo build --release
   ```

## 使い方

管理者権限で実行し、パケットをキャプチャします。  

```sh
sudo ./target/release/icmp-packet-capture <ネットワークインタフェース名>
```

例えば、以下のように実行します。  

```sh
sudo ./target/release/icmp-packet-capture eth0
```

キャプチャしたパケットの送信元　IP アドレスと ICMP パケットの内容が表示されます。  
出力例は以下です。  

```
Source address: 192.168.3.16, ICMP packet: IcmpPacket { icmp_type : IcmpType(8), icmp_code : IcmpCode(0), checksum : 19712,  }
```