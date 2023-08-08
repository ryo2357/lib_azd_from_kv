# azd_from_kv

azd のモータドライバを KV7500 から Ethernet/IP で稼働させる

KV7500 にたいしては TCP/IP の上位リンクで指示を出す

## usage

.env ファイルに KV のアドレスを記載。ポートは上位リンク専用のポート番号

```Dotenv
AzdFromKvConfig_address=192.168.0.10:8501
```

```rust
use dotenv::dotenv;
use azd_from_kv::{AzdFromKvConfig, AzdKvDirectClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AzdFromKvConfig::from_env().unwrap();
    info!("設定取得完了");
    let mut azd = AzdKvDirectClient::create(config).await?;
    info!("クライアント作成完了");

    // 動作
    azd.throw_command_direct_move(9000, 500).await?;
    info!("動作指令完了");
    azd.wait_start_move().await?;
    azd.throw_command_direct_move_trigger_off(9000, 500).await?;
    info!("トリガーオフ");
    azd.check_finish_move().await?;
    info!("動作完了");

    azd.throw_command_direct_move(0, 4000).await?;
    info!("基準位置復帰動作指令完了");
    azd.wait_start_move().await?;
    azd.throw_command_direct_move_trigger_off(0, 2000).await?;
    info!("トリガーオフ");
    azd.check_finish_move().await?;
    info!("動作完了");

    Ok(())
}

```
