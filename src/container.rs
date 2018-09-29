use std;
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
//Labels, HostConfig
pub struct Container {
    pub id: String,
    pub image: String,
    pub status: String,
    pub command: String,
    pub created: u64,
    pub names: Vec<String>,
    pub ports: Vec<Port>,
    #[serde(rename = "SizeRW")]
    pub size_rw: Option<u64>, // I guess it is optional on Mac.
    pub size_root_fs: Option<u64>,
    pub labels: Option<HashMap<String, String>>,
    pub host_config: HostConfig
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Port {
    #[serde(rename = "IP")]
    pub ip: Option<String>,
    pub private_port: u64,
    pub public_port: Option<u64>,
    #[serde(rename = "type")]
    pub ty: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostConfig {
    pub network_mode: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInfo {
    pub app_armor_profile: String,
    pub args: Vec<String>,
    pub config: Config,
    pub created: String,
    pub driver: String,
    // ExecIDs
    // GraphDriver
    // HostConfig
    pub hostname_path: String,
    pub hosts_path: String,
    pub id: String,
    pub image: String,
    pub log_path: String,
    pub mount_label: String,
    pub mounts: Vec<Mount>,
    pub name: String,
    pub network_settings: NetworkSettings,
    pub path: String,
    pub process_label: String,
    pub resolv_conf_path: String,
    pub restart_count: u64,
    pub state: State,
}

/// This type represents a `struct{}` in the Go code.
pub type UnspecifiedObject = HashMap<String, String>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub attach_stderr: bool,
    pub attach_stdin: bool,
    pub attach_stdout: bool,
    // TODO: Verify that this is never just a `String`.
    // pub Cmd: Vec<String>,
    #[serde(rename = "Domainname")]
    pub domain_name: String,
    // TODO: The source says `Option<String>` but I've seen
    // `Option<Vec<String>>` on the wire.  Ignore until we figure it out.
    // pub Entrypoint: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
    pub exposed_ports: Option<HashMap<String, UnspecifiedObject>>,
    pub hostname: String,
    pub image: String,
    pub labels: HashMap<String, String>,
    // TODO: We don't know exacly what this vec contains.
    // pub OnBuild: Option<Vec<???>>,
    pub open_stdin: bool,
    pub stdin_once: bool,
    pub tty: bool,
    pub lsnr: String,
    pub volumes: Option<HashMap<String, UnspecifiedObject>>,
    pub working_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mount {
    // Name (optional)
    // Driver (optional)
    pub source: String,
    pub destination: String,
    pub mode: String,
    #[serde(rename = "RW")]
    pub rw: bool,
    pub propogration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkSettings {
    pub bridge: String,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    pub gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6_address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6_prefix_len: u32,
    pub hairpin_mode: bool,
    #[serde(rename = "IPAddress")]
    pub ip_address: String,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: u32,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6_gateway: String,
    #[serde(rename = "LinkLocalIPv6Address")]
    pub link_local_ipv6_address: String,
    #[serde(rename = "LinkLocalIPv6PrefixLen")]
    pub link_local_ipv6_prefix_len: u32,
    pub mac_address: String,
    pub networks: HashMap<String, Network>,
    pub ports: Option<HashMap<String, Option<Vec<PortMapping>>>>,
    #[serde(rename = "SandboxID")]
    pub sandbox_id: String,
    pub sandbox_key: String,
    // These two are null in the current output.
    // pub SecondaryIPAddresses: ,
    // pub SecondaryIPv6Addresses: ,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Network {
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: String,
    pub gateway: String,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6_address: String,
    #[serde(rename = "GlobalIPv6PrefixLen")]
    pub global_ipv6_prefix_len: u32,
    // pub IPAMConfig: ,
    #[serde(rename = "IPAddress")]
    pub ip_address: String,
    #[serde(rename = "IPPrefixLen")]
    pub ip_prefix_len: u32,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6_gateway: String,
    // pub Links:
    pub mac_address: String,
    #[serde(rename = "NetworkID")]
    pub network_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PortMapping {
    pub host_ip: String,
    pub host_port: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct State {
    pub status: String,
    pub running: bool,
    pub paused: bool,
    pub restarting: bool,
    #[serde(rename = "OOMKilled")]
    pub oom_killed: bool,
    pub dead: bool,
    // I don't know whether PIDs can be negative here.  They're normally
    // positive, but sometimes negative PIDs are used in certain APIs.
    pub pid: i64,
    pub exit_code: i64,
    pub error: String,
    pub started_at: String,
    pub finished_at: String
}

impl std::fmt::Display for Container {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.id)
    }
}

impl std::fmt::Display for ContainerInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.id)
    }
}
