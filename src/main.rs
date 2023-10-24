use pnet::datalink; // 用于获取网络接口信息
use std::process::Command; // 用于调用外部命令

fn main() {
    // 获取所有网络接口的信息
    let interfaces = datalink::interfaces();

    // 查找具有指定 IP 地址的网络接口
    let ip_to_find = "198.18.0.1";
    for interface in interfaces {
        for ip in interface.ips {
            if ip.ip().to_string() == ip_to_find {
                // 当找到正确的接口时，打印其名称
                println!("Found the interface: {}", interface.name);

                // 构建用于更改 MTU 的命令
                let output = Command::new("sudo")
                    .arg("ip")
                    .arg("link")
                    .arg("set")
                    .arg(interface.name)
                    .arg("mtu")
                    .arg("1400")
                    .output()
                    .expect("Failed to execute command");

                // 打印命令输出（如果有的话）
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("stdout: {}", stdout);
                println!("stderr: {}", stderr);

                return;
            }
        }
    }

    println!("No interface found with IP {}", ip_to_find);
}
