use dotenv::dotenv;

use mylogger::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    mylogger::init();
    // module_test();
    // config_test();

    client_test_3().await?;
    Ok(())
}

fn module_test() {
    info!("from main");
    azd::azd_info();
}

fn config_test() {
    dotenv().ok();
    let config = azd::config::AzdUdpConfig::from_env().unwrap();

    info!("{:?}", config);
}

async fn client_test_3() -> anyhow::Result<()> {
    // [Rust でのEthernet/IPパケットキャプチャ 実践編 - Qiita](https://qiita.com/m10i/items/f8d3db359f150aafc83b)

    Ok(())
}

async fn client_test() -> anyhow::Result<()> {
    // 読み出しができない
    use tokio::net::UdpSocket;

    let sock = UdpSocket::bind("192.168.1.3:44818").await?;
    info!("make socket");
    let remote_addr = "192.168.1.2:44818";
    sock.connect(remote_addr).await?;
    info!("connected");

    let mut read_buf: azd::data_structure::ReceiveBuff = [0; 56];
    loop {
        info!("loop");
        let _len = sock.recv(&mut read_buf).await?;
        info!("receive {:?}", read_buf);

        // let len = sock.send_to(&buf[..len], addr).await?;
        // println!("{:?} bytes sent", len);
    }
    Ok(())
}

// async fn client_test_2() -> anyhow::Result<()> {
//     // タグデータ通信はできないのでは？
//     use rseip::client::ab_eip::*;
//     use rseip::precludes::*;

//     let mut client = AbEipClient::new_host_lookup("192.168.1.2")
//         .await?
//         .with_connection_path(PortSegment::default());
//     info!("connected");

//     let tag = EPath::parse_tag("test_car1_x")?;
//     info!("read tag...");
//     let value: TagValue<i32> = client.read_tag(tag.clone()).await?;
//     info!("tag value: {:?}", value);

//     Ok(())
// }
