use std::{collections::HashMap, str::FromStr};
use std::collections::VecDeque;
use dbus::arg;


use dbus::arg::{cast, prop_cast, PropMap, RefArg, Variant};

pub trait PropMapConvert: Sized {
    fn from_propmap(map: &PropMap) -> Self;
    fn to_propmap(&self) -> PropMap;
}

pub trait Enum: Sized {
    fn from_i32(num: i32) -> Self;
    fn to_i32(&self) -> i32;
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ConversionError {
    message: &'static str,
}

#[derive(Debug, Default)]
pub struct Connection {
    pub settings: ConnectionSettings,
    // x802: X802Settings,
    pub device: TypeSettings,
    pub ipv4: IPV4Settings,
    pub ipv6: IPV6Settings,
    // TODO check if x802 is actually even necessary?
    // TODO implement wifi security settings
}

impl Connection {
    pub fn convert_from_propmap(
        map: HashMap<
            std::string::String,
            HashMap<std::string::String, dbus::arg::Variant<Box<dyn RefArg>>>,
        >,
    ) -> Result<Self, ConversionError> {
        let mut settings: Option<ConnectionSettings> = None;
        // let mut x802: Option<X802Settings> = None;
        let mut device: Option<TypeSettings> = None;
        let mut ipv4: Option<IPV4Settings> = None;
        let mut ipv6: Option<IPV6Settings> = None;
        // dbg!(&map);
        for (category, submap) in map {
            match category.as_str() {
                "802-11-wireless" => {
                    device = Some(TypeSettings::WIFI(
                        Box::new(WifiSettings::from_propmap(&submap)),
                        Box::new(WifiSecuritySettings::from_propmap(&submap)),
                    ));
                }
                "802-3-ethernet" => {
                    device = Some(TypeSettings::ETHERNET(Box::new(
                        EthernetSettings::from_propmap(&submap),
                    )))
                }
                "vpn" => {
                    device = Some(TypeSettings::VPN(Box::new(VPNSettings::from_propmap(
                        &submap,
                    ))))
                }
                "ipv6" => ipv6 = Some(IPV6Settings::from_propmap(&submap)),
                "ipv4" => ipv4 = Some(IPV4Settings::from_propmap(&submap)),
                "connection" => settings = Some(ConnectionSettings::from_propmap(&submap)),
                // "802-1x" => x802 = Some(X802Settings::from_propmap(&submap)),
                _ => continue,
            }
        }
        if settings.is_none() | device.is_none() | ipv4.is_none() | ipv6.is_none() {
            return Err(ConversionError {
                message: "could not convert propmap",
            });
        }

        let settings = settings.unwrap();
        // let x802 = x802.unwrap();
        let device = device.unwrap();
        let ipv4 = ipv4.unwrap();
        let ipv6 = ipv6.unwrap();
        Ok(Self {
            settings,
            // x802,
            device,
            ipv4,
            ipv6,
        })
    }

    pub fn convert_to_propmap(&self) -> HashMap<String, PropMap> {
        let mut map = HashMap::new();
        map.insert("connection".into(), self.settings.to_propmap());
        match &self.device {
            TypeSettings::WIFI(wifi, wifisecurity) => {
                map.insert("802-11-wireless".into(), wifi.to_propmap());
                map.insert("802-11-wireless-security".into(), wifisecurity.to_propmap());
            }
            TypeSettings::ETHERNET(ethernet) => {
                map.insert("802-3-ethernet".into(), ethernet.to_propmap());
            }
            TypeSettings::VPN(vpn) => {
                map.insert("vpn".into(), vpn.to_propmap());
            }
            TypeSettings::None => (),
        };
        map.insert("ipv4".into(), self.ipv4.to_propmap());
        map.insert("ipv6".into(), self.ipv6.to_propmap());
        map
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub enum Trust {
    HOME,
    WORK,
    PUBLIC,
    #[default]
    DEFAULT,
}

impl FromStr for Trust {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Home" => Ok(Trust::HOME),
            "Work" => Ok(Trust::WORK),
            "Public" => Ok(Trust::PUBLIC),
            _ => Ok(Trust::DEFAULT),
        }
    }
}

impl ToString for Trust {
    fn to_string(&self) -> String {
        match self {
            Trust::HOME => String::from("Home"),
            Trust::WORK => String::from("Work"),
            Trust::PUBLIC => String::from("Public"),
            Trust::DEFAULT => String::from("null"),
        }
    }
}

impl Enum for Trust {
    fn from_i32(num: i32) -> Self {
        match num {
            0 => Trust::HOME,
            1 => Trust::WORK,
            2 => Trust::PUBLIC,
            _ => Trust::DEFAULT,
        }
    }

    fn to_i32(&self) -> i32 {
        match self {
            Trust::HOME => 0,
            Trust::WORK => 1,
            Trust::PUBLIC => 2,
            Trust::DEFAULT => 3,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub enum Mode {
    #[default]
    INFRASTRUCTURE,
    ADHOC,
    AP,
}

impl FromStr for Mode {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "adhoc" => Ok(Mode::ADHOC),
            "ap" => Ok(Mode::AP),
            _ => Ok(Mode::INFRASTRUCTURE),
        }
    }
}

impl ToString for Mode {
    fn to_string(&self) -> String {
        match self {
            Mode::ADHOC => String::from("adhoc"),
            Mode::AP => String::from("ap"),
            Mode::INFRASTRUCTURE => String::from("infrastructure"),
        }
    }
}

impl Enum for Mode {
    fn from_i32(num: i32) -> Self {
        match num {
            0 => Mode::INFRASTRUCTURE,
            1 => Mode::ADHOC,
            _ => Mode::AP,
        }
    }

    fn to_i32(&self) -> i32 {
        match self {
            Mode::INFRASTRUCTURE => 0,
            Mode::ADHOC => 1,
            Mode::AP => 2,
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Band {
    _5GHZ,
    _24GHZ,
    #[default]
    NONE,
}

impl FromStr for Band {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Band::_5GHZ),
            "bg" => Ok(Band::_24GHZ),
            _ => Ok(Band::NONE),
        }
    }
}

impl ToString for Band {
    fn to_string(&self) -> String {
        match self {
            Band::_5GHZ => String::from("bg"),
            Band::_24GHZ => String::from("a"),
            Band::NONE => String::from(""),
        }
    }
}

impl Enum for Band {
    fn from_i32(num: i32) -> Self {
        match num {
            0 => Band::_5GHZ,
            1 => Band::_24GHZ,
            _ => Band::NONE,
        }
    }

    fn to_i32(&self) -> i32 {
        match self {
            Band::_5GHZ => 0,
            Band::_24GHZ => 1,
            Band::NONE => 2,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub enum Duplex {
    HALF,
    #[default]
    FULL,
}

impl FromStr for Duplex {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "half" => Ok(Duplex::HALF),
            _ => Ok(Duplex::FULL),
        }
    }
}

impl ToString for Duplex {
    fn to_string(&self) -> String {
        match self {
            Duplex::HALF => String::from("half"),
            Duplex::FULL => String::from("full"),
        }
    }
}

impl Enum for Duplex {
    fn from_i32(num: i32) -> Self {
        match num {
            0 => Duplex::HALF,
            _ => Duplex::FULL,
        }
    }

    fn to_i32(&self) -> i32 {
        match self {
            Duplex::HALF => 0,
            Duplex::FULL => 1,
        }
    }
}

#[derive(Debug, Default)]
pub enum TypeSettings {
    WIFI(Box<WifiSettings>, Box<WifiSecuritySettings>),
    ETHERNET(Box<EthernetSettings>),
    VPN(Box<VPNSettings>),
    #[default]
    None,
}

impl ToString for TypeSettings {
    fn to_string(&self) -> String {
        match self {
            TypeSettings::WIFI(_, _) => String::from("wifi"),
            TypeSettings::ETHERNET(_) => String::from("ethernet"),
            TypeSettings::VPN(_) => String::from("vpn"),
            TypeSettings::None => String::from(""),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct EthernetSettings {
    pub auto_negotiate: bool,
    pub cloned_mac_address: String,
    pub duplex: Duplex,
    pub mtu: u32,
    pub name: String,
    pub speed: u32,
}

impl PropMapConvert for EthernetSettings {
    fn from_propmap(map: &PropMap) -> Self {
        let auto_negotiate: Option<&bool> = prop_cast(map, "auto-negotiate");
        let cloned_address_opt: Option<&String> = prop_cast(map, "cloned-mac-address");
        let cloned_mac_address = if let Some(cloned_address_opt) = cloned_address_opt {
            cloned_address_opt.clone()
        } else {
            String::from("")
        };
        let duplex_opt: Option<&String> = prop_cast(map, "mode");
        let duplex = if let Some(duplex_opt) = duplex_opt {
            Duplex::from_str(duplex_opt).ok().unwrap()
        } else {
            Duplex::FULL
        };
        let mtu: Option<&u32> = prop_cast(map, "mtu");
        let name_opt: Option<&String> = prop_cast(map, "name");
        let name = if let Some(name_opt) = name_opt {
            name_opt.clone()
        } else {
            String::from("")
        };
        let speed: Option<&u32> = prop_cast(map, "speed");
        Self {
            auto_negotiate: *auto_negotiate.unwrap_or(&true),
            cloned_mac_address,
            duplex,
            mtu: *mtu.unwrap_or(&0),
            name,
            speed: *speed.unwrap_or(&0),
        }
    }

    fn to_propmap(&self) -> PropMap {
        let mut map = PropMap::new();
        map.insert(
            "auto-negotiate".into(),
            Variant(Box::new(self.auto_negotiate)),
        );
        map.insert("duplex".into(), Variant(Box::new(self.duplex.to_i32())));
        map.insert("mtu".into(), Variant(Box::new(self.mtu)));
        map.insert("name".into(), Variant(Box::new(self.name.clone())));
        map.insert("speed".into(), Variant(Box::new(self.speed)));
        map
    }
}

#[derive(Debug, Clone)]
pub struct VPNSettings {
    pub data: HashMap<String, String>,
    pub name: String,
    pub persistent: bool,
    pub secrets: HashMap<String, String>,
    pub service_type: String,
    pub timeout: u32,
    pub user_name: String,
}

impl PropMapConvert for VPNSettings {
    fn from_propmap(map: &PropMap) -> Self {
        let data_opt: Option<&HashMap<String, String>> = prop_cast(map, "data");
        let data = if let Some(data_opt) = data_opt {
            data_opt.clone()
        } else {
            HashMap::new()
        };
        let name_opt: Option<&String> = prop_cast(map, "name");
        let name = if let Some(name_opt) = name_opt {
            name_opt.clone()
        } else {
            String::from("vpn")
        };
        let persistent: Option<&bool> = prop_cast(map, "persistent");
        let secrets_opt: Option<&HashMap<String, String>> = prop_cast(map, "secrets");
        let secrets = if let Some(secrets_opt) = secrets_opt {
            secrets_opt.clone()
        } else {
            HashMap::new()
        };
        let service_type_opt: Option<&String> = prop_cast(map, "service-type");
        let service_type = if let Some(service_type_opt) = service_type_opt {
            service_type_opt.clone()
        } else {
            String::from("")
        };
        let timeout: Option<&u32> = prop_cast(map, "timeout");
        let user_name_opt: Option<&String> = prop_cast(map, "user-name");
        let user_name = if let Some(user_name_opt) = user_name_opt {
            user_name_opt.clone()
        } else {
            String::from("")
        };
        Self {
            data,
            name,
            persistent: *persistent.unwrap_or(&false),
            secrets,
            service_type,
            timeout: *timeout.unwrap_or(&0),
            user_name,
        }
    }

    fn to_propmap(&self) -> PropMap {
        let mut map = PropMap::new();
        map.insert("data".into(), Variant(Box::new(self.data.clone())));
        map.insert("name".into(), Variant(Box::new(self.name.clone())));
        map.insert("persistent".into(), Variant(Box::new(self.persistent)));
        map.insert("secrets".into(), Variant(Box::new(self.secrets.clone())));
        map.insert(
            "service-type".into(),
            Variant(Box::new(self.service_type.clone())),
        );
        map.insert("timeout".into(), Variant(Box::new(self.timeout)));
        map.insert(
            "user-name".into(),
            Variant(Box::new(self.user_name.clone())),
        );
        map
    }
}

#[derive(Debug, Clone)]
pub struct WifiSettings {
    pub band: Band,
    pub channel: u32,
    pub cloned_mac_address: String,
    pub mode: Mode,
    pub mtu: u32,
    pub powersave: u32,
    pub rate: u32,
    pub ssid: Vec<u8>,
}

impl PropMapConvert for WifiSettings {
    fn from_propmap(map: &PropMap) -> Self {
        let mode_opt: Option<&String> = prop_cast(map, "mode");
        let mode = if let Some(mode_opt) = mode_opt {
            Mode::from_str(mode_opt.as_str()).ok().unwrap()
        } else {
            Mode::from_str("").ok().unwrap()
        };
        let channel_opt: Option<&u32> = prop_cast(map, "channel");
        let channel = *channel_opt.unwrap_or(&0);
        let band_opt: Option<&String> = prop_cast(map, "band");
        let band = if let Some(band_opt) = band_opt {
            Band::from_str(band_opt.as_str()).ok().unwrap()
        } else {
            Band::from_str("").ok().unwrap()
        };
        let cloned_address_opt: Option<&String> = prop_cast(map, "cloned-mac-address");
        let cloned_mac_address = if let Some(cloned_address_opt) = cloned_address_opt {
            cloned_address_opt.clone()
        } else {
            String::from("")
        };
        let mtu_opt: Option<&u32> = prop_cast(map, "mtu");
        let mtu = *mtu_opt.unwrap_or(&0);
        let powersave_opt: Option<&u32> = prop_cast(map, "powersave");
        let powersave = *powersave_opt.unwrap_or(&0);
        let rate_opt: Option<&u32> = prop_cast(map, "rate");
        let rate = *rate_opt.unwrap_or(&0);

        let ssid_opt: Option<&Vec<u8>> = prop_cast(map, "ssid");
        let ssid = if let Some(ssid_opt) = ssid_opt {
            ssid_opt.clone()
        } else {
            Vec::new()
        };
        Self {
            band,
            channel,
            cloned_mac_address,
            mode,
            mtu,
            powersave,
            rate,
            ssid,
        }
    }

    fn to_propmap(&self) -> PropMap {
        let mut map = PropMap::new();
        if self.band != Band::NONE {
            map.insert("band".into(), Variant(Box::new(self.band.to_string())));
        }
        map.insert("channel".into(), Variant(Box::new(self.channel)));
        map.insert("mode".into(), Variant(Box::new(self.mode.to_string())));
        map.insert("mtu".into(), Variant(Box::new(self.mtu)));
        map.insert("powersave".into(), Variant(Box::new(self.powersave)));
        map.insert("rate".into(), Variant(Box::new(self.rate)));
        map.insert("ssid".into(), Variant(Box::new(self.ssid.clone())));
        map
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct X802Settings {
    pub ca_cert: Vec<u8>,
    pub ca_cert_string: String,
    pub client_cert: Vec<u8>,
    pub domain_suffix: String,
    pub eap: Vec<String>,
    pub identity: String,
    pub pac_file: String,
    pub password: String,
    pub password_flags: u32,
    pub password_raw_flags: Vec<u8>,
}

// impl PropMapConvert for X802Settings {
//     fn from_propmap(map: &PropMap) -> Self {
//         let ca_cert: Vec<u8>;
//         let ca_cert_string: String;
//         let client_cert: Vec<u8>;
//         let domain_suffix: String;
//         let eap: Vec<String>;
//         let identity: String;
//         let pac_file: String;
//         let password: String;
//         let password_raw_flags: Vec<u8>;
//         let password_flags = prop_cast(&map, "password-flags");
//         let ca_cert_opt: Option<&Vec<u8>> = prop_cast(&map, "ca-cert");
//         if ca_cert_opt.is_none() {
//             ca_cert = Vec::new();
//         } else {
//             ca_cert = ca_cert_opt.unwrap().clone();
//         }
//         let ca_cert_string_opt: Option<&String> = prop_cast(&map, "ca-cert-string");
//         if ca_cert_string_opt.is_none() {
//             ca_cert_string = String::new();
//         } else {
//             ca_cert_string = ca_cert_string_opt.unwrap().clone();
//         }
//         let client_cert_opt: Option<&Vec<u8>> = prop_cast(&map, "client-cert");
//         if client_cert_opt.is_none() {
//             client_cert = Vec::new();
//         } else {
//             client_cert = client_cert_opt.unwrap().clone();
//         }
//         let domain_suffix_opt: Option<&String> = prop_cast(&map, "domain-suffix");
//         if domain_suffix_opt.is_none() {
//             domain_suffix = String::from("");
//         } else {
//             domain_suffix = domain_suffix_opt.unwrap().clone();
//         }
//         let eap_opt: Option<&Vec<String>> = prop_cast(&map, "eap");
//         if eap_opt.is_none() {
//             eap = Vec::new();
//         } else {
//             eap = eap_opt.unwrap().clone();
//         }
//         let identity_opt: Option<&String> = prop_cast(&map, "identity");
//         if identity_opt.is_none() {
//             identity = String::from("");
//         } else {
//             identity = identity_opt.unwrap().clone();
//         }
//         let pac_file_opt: Option<&String> = prop_cast(&map, "pac-file");
//         if pac_file_opt.is_none() {
//             pac_file = String::from("");
//         } else {
//             pac_file = pac_file_opt.unwrap().clone();
//         }
//         let password_opt: Option<&String> = prop_cast(&map, "password");
//         if password_opt.is_none() {
//             password = String::from("");
//         } else {
//             password = password_opt.unwrap().clone();
//         }
//         let password_raw_flags_opt: Option<&Vec<u8>> = prop_cast(&map, "password-raw-flags");
//         if password_raw_flags_opt.is_none() {
//             password_raw_flags = Vec::new();
//         } else {
//             password_raw_flags = password_raw_flags_opt.unwrap().clone();
//         }
//         Self {
//             ca_cert,
//             ca_cert_string,
//             client_cert,
//             domain_suffix,
//             eap,
//             identity,
//             pac_file,
//             password,
//             password_flags: *password_flags.unwrap_or_else(|| &0),
//             password_raw_flags,
//         }
//     }
// }

pub type AddressType = (String, u32, String, i64);

#[derive(Debug, Default)]
pub enum DNSMethod4 {
    #[default]
    AUTO,
    MANUAL,
    LINKLOCAL,
    SHARED,
    DISABLED,
}

impl FromStr for DNSMethod4 {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(DNSMethod4::AUTO),
            "manual" => Ok(DNSMethod4::MANUAL),
            "link-local" => Ok(DNSMethod4::LINKLOCAL),
            "shared" => Ok(DNSMethod4::SHARED),
            _ => Ok(DNSMethod4::DISABLED),
        }
    }
}

impl ToString for DNSMethod4 {
    fn to_string(&self) -> String {
        match self {
            DNSMethod4::AUTO => String::from("auto"),
            DNSMethod4::MANUAL => String::from("manual"),
            DNSMethod4::LINKLOCAL => String::from("link-local"),
            DNSMethod4::SHARED => String::from("shared"),
            DNSMethod4::DISABLED => String::from("disabled"),
        }
    }
}

impl Enum for DNSMethod4 {
    fn from_i32(num: i32) -> Self {
        match num {
            0 => DNSMethod4::AUTO,
            1 => DNSMethod4::MANUAL,
            2 => DNSMethod4::LINKLOCAL,
            3 => DNSMethod4::SHARED,
            _ => DNSMethod4::DISABLED,
        }
    }

    fn to_i32(&self) -> i32 {
        match self {
            DNSMethod4::AUTO => 0,
            DNSMethod4::MANUAL => 1,
            DNSMethod4::LINKLOCAL => 2,
            DNSMethod4::SHARED => 3,
            DNSMethod4::DISABLED => 4,
        }
    }
}

#[derive(Debug, Default)]
pub enum DNSMethod6 {
    #[default]
    AUTO,
    DHCP,
    MANUAL,
    LINKLOCAL,
    SHARED,
    DISABLED,
}

impl FromStr for DNSMethod6 {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(DNSMethod6::AUTO),
            "dhcp" => Ok(DNSMethod6::DHCP),
            "manual" => Ok(DNSMethod6::MANUAL),
            "link-local" => Ok(DNSMethod6::LINKLOCAL),
            "shared" => Ok(DNSMethod6::SHARED),
            _ => Ok(DNSMethod6::DISABLED),
        }
    }
}

impl ToString for DNSMethod6 {
    fn to_string(&self) -> String {
        match self {
            DNSMethod6::AUTO => String::from("auto"),
            DNSMethod6::DHCP => String::from("dhcp"),
            DNSMethod6::MANUAL => String::from("manual"),
            DNSMethod6::LINKLOCAL => String::from("link-local"),
            DNSMethod6::SHARED => String::from("shared"),
            DNSMethod6::DISABLED => String::from("disabled"),
        }
    }
}

impl Enum for DNSMethod6 {
    fn from_i32(num: i32) -> Self {
        match num {
            0 => DNSMethod6::AUTO,
            1 => DNSMethod6::DHCP,
            2 => DNSMethod6::MANUAL,
            3 => DNSMethod6::LINKLOCAL,
            4 => DNSMethod6::SHARED,
            _ => DNSMethod6::DISABLED,
        }
    }

    fn to_i32(&self) -> i32 {
        match self {
            DNSMethod6::AUTO => 0,
            DNSMethod6::DHCP => 1,
            DNSMethod6::MANUAL => 2,
            DNSMethod6::LINKLOCAL => 3,
            DNSMethod6::SHARED => 4,
            DNSMethod6::DISABLED => 5,
        }
    }
}

#[derive(Debug, Default)]
pub struct IPV4Settings {
    pub address_data: Vec<AddressType>,
    pub dns: Vec<u32>,
    pub dns_options: Vec<String>,
    pub dns_priority: i32,
    pub dns_search: Vec<String>,
    pub gateway: String,
    pub ignore_auto_dns: bool,
    pub ignore_auto_routes: bool,
    pub may_fail: bool,
    pub method: DNSMethod4,
    pub never_default: bool,
    pub route_data: Vec<AddressType>,
}

impl PropMapConvert for IPV4Settings {
    fn from_propmap(map: &PropMap) -> Self {
        let address_data = get_addresses(map, "address-data");
        let dns_opt: Option<&Vec<u32>> = prop_cast(map, "dns");
        let dns = if let Some(dns_opt) = dns_opt {
            dns_opt.clone()
        } else {
            Vec::new()
        };
        let dns_options_opt: Option<&Vec<String>> = prop_cast(map, "dns-options");
        let dns_options = if let Some(dns_options_opt) = dns_options_opt {
            dns_options_opt.clone()
        } else {
            Vec::new()
        };
        let dns_priority = *prop_cast(map, "dns-priority").unwrap_or(&0);
        let dns_search_opt: Option<&Vec<String>> = prop_cast(map, "dns-search");
        let dns_search = if let Some(dns_search_opt) = dns_search_opt {
            dns_search_opt.clone()
        } else {
            Vec::new()
        };
        let gateway_opt: Option<&String> = prop_cast(map, "gateway");
        let gateway = if let Some(gateway_opt) = gateway_opt {
            gateway_opt.clone()
        } else {
            String::from("")
        };
        let ignore_auto_dns = *prop_cast(map, "ignore-auto-dns").unwrap_or(&false);
        let ignore_auto_dns_routes = *prop_cast(map, "ignore-auto-routes").unwrap_or(&false);
        let may_fail = *prop_cast(map, "may-fail").unwrap_or(&true);
        let method_opt: Option<&String> = prop_cast(map, "method");
        let dns_method = if let Some(method_opt) = method_opt {
            DNSMethod4::from_str(method_opt.as_str()).unwrap()
        } else {
            DNSMethod4::DISABLED
        };
        let never_default = *prop_cast(map, "never-default").unwrap_or(&true);
        let route_data = get_addresses(map, "route-data");
        Self {
            address_data,
            dns,
            dns_options,
            dns_priority,
            dns_search,
            gateway,
            ignore_auto_dns,
            ignore_auto_routes: ignore_auto_dns_routes,
            may_fail,
            method: dns_method,
            never_default,
            route_data,
        }
    }

    fn to_propmap(&self) -> PropMap {
        let mut map = PropMap::new();
        map.insert(
            "address-data".into(),
            Variant(Box::new(self.address_data.clone())),
        );
        map.insert("dns".into(), Variant(Box::new(self.dns.clone())));
        map.insert(
            "dns-options".into(),
            Variant(Box::new(self.dns_options.clone())),
        );
        map.insert("dns-priority".into(), Variant(Box::new(self.dns_priority)));
        map.insert(
            "dns-search".into(),
            Variant(Box::new(self.dns_search.clone())),
        );
        if !self.address_data.is_empty() {
            map.insert("gateway".into(), Variant(Box::new(self.gateway.clone())));
        }
        map.insert(
            "ignore-auto-dns".into(),
            Variant(Box::new(self.ignore_auto_dns)),
        );
        map.insert(
            "ignore-auto-routes".into(),
            Variant(Box::new(self.ignore_auto_routes)),
        );
        map.insert("may-fail".into(), Variant(Box::new(self.may_fail)));
        map.insert("method".into(), Variant(Box::new(self.method.to_string())));
        map.insert(
            "never-default".into(),
            Variant(Box::new(self.never_default)),
        );
        map.insert(
            "route-data".into(),
            Variant(Box::new(self.route_data.clone())),
        );
        map
    }
}

#[derive(Debug, Default)]
pub enum IPV6PrivacyMode {
    DISABLED,
    ENABLEDPEFERPUBLIC,
    ENABLEDPEFERTEMPORARY,
    #[default]
    UNKNOWN,
}

impl FromStr for IPV6PrivacyMode {
    type Err = ConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "disabled" => Ok(IPV6PrivacyMode::DISABLED),
            "enabled-prefer-public" => Ok(IPV6PrivacyMode::ENABLEDPEFERPUBLIC),
            "enabled-prefer-temporary" => Ok(IPV6PrivacyMode::ENABLEDPEFERTEMPORARY),
            _ => Ok(IPV6PrivacyMode::UNKNOWN),
        }
    }
}

impl ToString for IPV6PrivacyMode {
    fn to_string(&self) -> String {
        match self {
            IPV6PrivacyMode::UNKNOWN => String::from("unknown"),
            IPV6PrivacyMode::DISABLED => String::from("disabled"),
            IPV6PrivacyMode::ENABLEDPEFERPUBLIC => String::from("enabled-prefer-public"),
            IPV6PrivacyMode::ENABLEDPEFERTEMPORARY => String::from("enabled-prefer-temporary"),
        }
    }
}

impl Enum for IPV6PrivacyMode {
    fn from_i32(num: i32) -> Self {
        match num {
            -1 => IPV6PrivacyMode::UNKNOWN,
            0 => IPV6PrivacyMode::DISABLED,
            1 => IPV6PrivacyMode::ENABLEDPEFERPUBLIC,
            _ => IPV6PrivacyMode::ENABLEDPEFERTEMPORARY,
        }
    }

    fn to_i32(&self) -> i32 {
        match self {
            IPV6PrivacyMode::UNKNOWN => -1,
            IPV6PrivacyMode::DISABLED => 0,
            IPV6PrivacyMode::ENABLEDPEFERPUBLIC => 1,
            IPV6PrivacyMode::ENABLEDPEFERTEMPORARY => 2,
        }
    }
}

#[derive(Debug, Default)]
pub struct IPV6Settings {
    pub address_data: Vec<AddressType>,
    pub dns: Vec<Vec<u8>>,
    pub dns_options: Vec<String>,
    pub dns_priority: i32,
    pub dns_search: Vec<String>,
    pub gateway: String,
    pub ignore_auto_dns: bool,
    pub ignore_auto_routes: bool,
    pub ip6_privacy: IPV6PrivacyMode,
    pub may_fail: bool,
    pub method: DNSMethod6,
    pub never_default: bool,
    pub route_data: Vec<AddressType>,
}

impl PropMapConvert for IPV6Settings {
    fn from_propmap(map: &PropMap) -> Self {
        let address_data = get_addresses(map, "address-data");
        let dns_opt: Option<&Vec<Vec<u8>>> = prop_cast(map, "dns");
        // dbg!(map);
        let dns = if let Some(dns_opt) = dns_opt {
            dns_opt.clone()
        } else {
            Vec::new()
        };
        let dns_options_opt: Option<&Vec<String>> = prop_cast(map, "dns-options");
        let dns_options = if let Some(dns_options_opt) = dns_options_opt {
            dns_options_opt.clone()
        } else {
            Vec::new()
        };
        let dns_priority = *prop_cast(map, "dns-priority").unwrap_or(&0);
        let dns_search_opt: Option<&Vec<String>> = prop_cast(map, "dns-search");
        let dns_search = if let Some(dns_search_opt) = dns_search_opt {
            dns_search_opt.clone()
        } else {
            Vec::new()
        };
        let gateway_opt: Option<&String> = prop_cast(map, "gateway");
        let gateway = if let Some(gateway_opt) = gateway_opt {
            gateway_opt.clone()
        } else {
            String::from("")
        };
        let ignore_auto_dns = *prop_cast(map, "ignore-auto-dns").unwrap_or(&false);
        let ignore_auto_routes = *prop_cast(map, "ignore-auto-routes").unwrap_or(&false);
        let ipv6_privacy = IPV6PrivacyMode::from_i32(*prop_cast(map, "ip6-privacy").unwrap_or(&-1));
        let may_fail = *prop_cast(map, "may-fail").unwrap_or(&true);
        let method_opt: Option<&String> = prop_cast(map, "method");
        let dns_method = if let Some(method_opt) = method_opt {
            DNSMethod6::from_str(method_opt.as_str()).unwrap()
        } else {
            DNSMethod6::DISABLED
        };
        let never_default = *prop_cast(map, "never-default").unwrap_or(&true);
        let route_data = get_addresses(map, "route-data");
        Self {
            address_data,
            dns,
            dns_options,
            dns_priority,
            dns_search,
            gateway,
            ignore_auto_dns,
            ignore_auto_routes,
            ip6_privacy: ipv6_privacy,
            may_fail,
            method: dns_method,
            never_default,
            route_data,
        }
    }

    fn to_propmap(&self) -> PropMap {
        let mut map = PropMap::new();
        map.insert(
            "address-data".into(),
            Variant(Box::new(self.address_data.clone())),
        );
        map.insert("dns".into(), Variant(Box::new(self.dns.clone())));
        map.insert(
            "dns-options".into(),
            Variant(Box::new(self.dns_options.clone())),
        );
        map.insert("dns-priority".into(), Variant(Box::new(self.dns_priority)));
        map.insert(
            "dns-search".into(),
            Variant(Box::new(self.dns_search.clone())),
        );
        if !self.address_data.is_empty() {
            map.insert("gateway".into(), Variant(Box::new(self.gateway.clone())));
        }
        map.insert(
            "ignore-auto-dns".into(),
            Variant(Box::new(self.ignore_auto_dns)),
        );
        map.insert(
            "ignore-auto-routes".into(),
            Variant(Box::new(self.ignore_auto_routes)),
        );
        map.insert(
            "ip6-privacy".into(),
            Variant(Box::new(self.ip6_privacy.to_i32())),
        );
        map.insert("may-fail".into(), Variant(Box::new(self.may_fail)));
        map.insert("method".into(), Variant(Box::new(self.method.to_string())));
        map.insert(
            "never-default".into(),
            Variant(Box::new(self.never_default)),
        );
        map.insert(
            "route-data".into(),
            Variant(Box::new(self.route_data.clone())),
        );
        map
    }
}

fn get_addresses(map: &PropMap, address_type: &'static str) -> Vec<AddressType> {
    let mut address_data: Vec<AddressType> = Vec::new();
    let test = map.get(address_type);
    if let Some(asdf) = test {
        let any = asdf.0.as_any();
        dbg!("Type: {:?}", any.type_id());

        let option1: Option<&Vec<VecDeque<Box<dyn RefArg>>>> = cast::<Vec<VecDeque<Box<dyn RefArg>>>>(&asdf.0);
        let option2: Option<&Vec<AddressType>> = cast(&asdf.0);
        dbg!(option1);
        dbg!(option2);
    }

    let asdffd: Option<&VecDeque<Box<dyn RefArg>>> = prop_cast(map, address_type);
    let asedf  = if let Some(qwer) = asdffd {
        for x in qwer {
            let option1 = cast::<VecDeque<Box<dyn RefArg>>>(x);
            if option1.is_none() {
                continue;
            }
            let option1 = option1.unwrap();
            let address = cast::<String>(&option1[0]).unwrap().clone();
            let prefix = *cast::<u32>(&option1[1]).unwrap();
            let gateway = cast::<String>(&option1[2]).unwrap().clone();
            let metric = *cast::<i64>(&option1[3]).unwrap();
            address_data.push((address, prefix, gateway, metric));
        }
    } else {
        return address_data;
    };
    dbg!(asedf);

    for x in asdffd.unwrap() {
        let option = cast::<PropMap>(x);
        dbg!(option);
    }


    let address_data_opt: Option<&Vec<PropMap>> = prop_cast(map, address_type);
    if address_data_opt.is_some() {
        for entry in address_data_opt.unwrap() {
            let address_opt: Option<&String> = prop_cast(entry, "address");
            let prefix_length_opt: Option<&u32> = prop_cast(entry, "prefix");
            let gateway_opt: Option<&String> = prop_cast(entry, "gateway");
            let metric_opt: Option<&i64> = prop_cast(entry, "metric");
            let address = if let Some(address_opt) = address_opt {
                address_opt.clone()
            } else {
                String::from("")
            };
            let prefix = if let Some(prefix_length_opt) = prefix_length_opt {
                *prefix_length_opt
            } else {
                0
            };
            let gateway = gateway_opt.cloned().unwrap_or(String::from(""));
            let metric = metric_opt.cloned().unwrap_or(-1);
            address_data.push((address, prefix, gateway, metric))
        }
    }
    address_data
}

#[derive(Debug, Default)]
pub struct ConnectionSettings {
    pub autoconnect: bool,
    pub autoconnect_priority: i32,
    pub id: String,
    pub metered: i32,
    pub device_type: String,
    pub uuid: String,
    pub zone: Trust,
}

impl PropMapConvert for ConnectionSettings {
    fn from_propmap(map: &PropMap) -> Self {
        let autoconnect = prop_cast(map, "autoconnect");
        let autoconnect_priority = prop_cast(map, "autoconnect-priority");
        let id_opt: Option<&String> = prop_cast(map, "id");
        let id = if let Some(id_opt) = id_opt {
            id_opt.clone()
        } else {
            String::from("")
        };
        let metered = prop_cast(map, "metered");
        let zone_opt: Option<&String> = prop_cast(map, "trust");
        let zone = if let Some(zone_opt) = zone_opt {
            Trust::from_str(zone_opt.as_str()).ok().unwrap()
        } else {
            Trust::from_str("").ok().unwrap()
        };

        let uuid_opt: Option<&String> = prop_cast(map, "uuid");
        let uuid = if let Some(uuid_opt) = uuid_opt {
            uuid_opt.clone()
        } else {
            String::from("")
        };
        let device_type_opt: Option<&String> = prop_cast(map, "type");
        let device_type = if let Some(device_type_opt) = device_type_opt {
            device_type_opt.clone()
        } else {
            String::from("")
        };
        Self {
            autoconnect: *autoconnect.unwrap_or(&false),
            autoconnect_priority: *autoconnect_priority.unwrap_or(&-1),
            id,
            metered: *metered.unwrap_or(&0),
            device_type,
            uuid,
            zone,
        }
    }

    fn to_propmap(&self) -> PropMap {
        let mut map = PropMap::new();
        map.insert("autoconnect".into(), Variant(Box::new(self.autoconnect)));
        map.insert(
            "autoconnect-priority".into(),
            Variant(Box::new(self.autoconnect_priority)),
        );
        map.insert("id".into(), Variant(Box::new(self.id.clone())));
        map.insert("metered".into(), Variant(Box::new(self.metered)));
        map.insert("type".into(), Variant(Box::new(self.device_type.clone())));
        map.insert("uuid".into(), Variant(Box::new(self.uuid.clone())));
        map.insert("zone".into(), Variant(Box::new(self.zone.to_i32())));
        map
    }
}

#[derive(Debug, Default, Clone)]
pub enum SecretSettingsFlag {
    #[default]
    NONE,
    AgentOwned,
    NotSaved,
    NotRequired,
}

impl Enum for SecretSettingsFlag {
    fn from_i32(num: i32) -> Self {
        match num {
            0 => SecretSettingsFlag::NONE,
            1 => SecretSettingsFlag::AgentOwned,
            2 => SecretSettingsFlag::NotSaved,
            _ => SecretSettingsFlag::NotRequired,
        }
    }

    fn to_i32(&self) -> i32 {
        match self {
            SecretSettingsFlag::NONE => 0,
            SecretSettingsFlag::AgentOwned => 1,
            SecretSettingsFlag::NotSaved => 2,
            SecretSettingsFlag::NotRequired => 3,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum WEPKeyType {
    #[default]
    UNKNOWN,
    KEY,
    PASSPHRASE,
}

impl Enum for WEPKeyType {
    fn from_i32(num: i32) -> Self {
        match num {
            0 => WEPKeyType::UNKNOWN,
            1 => WEPKeyType::KEY,
            _ => WEPKeyType::PASSPHRASE,
        }
    }

    fn to_i32(&self) -> i32 {
        match self {
            WEPKeyType::UNKNOWN => 0,
            WEPKeyType::KEY => 1,
            WEPKeyType::PASSPHRASE => 2,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum KeyManagement {
    NONE,
    IEEE8021X,
    WPANONE,
    #[default]
    WPAPSK,
    WPAEAP,
}

impl KeyManagement {
    fn from_str(s: &str) -> KeyManagement {
        match s {
            "none" => KeyManagement::NONE,
            "ieee8021x" => KeyManagement::IEEE8021X,
            "wpa-none" => KeyManagement::WPANONE,
            "wpa-psk" => KeyManagement::WPAPSK,
            "wpa-eap" => KeyManagement::WPAEAP,
            _ => KeyManagement::WPAPSK,
        }
    }
}

impl ToString for KeyManagement {
    fn to_string(&self) -> String {
        match self {
            KeyManagement::NONE => String::from("none"),
            KeyManagement::IEEE8021X => String::from("ieee8021x"),
            KeyManagement::WPANONE => String::from("wpa-none"),
            KeyManagement::WPAPSK => String::from("wpa-psk"),
            KeyManagement::WPAEAP => String::from("wpa-eap"),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct WifiSecuritySettings {
    pub authentication_algorithm: String,
    pub group: Vec<String>,
    pub key_management: KeyManagement,
    pub leap_password: String,
    pub leap_password_flags: SecretSettingsFlag,
    pub leap_username: String,
    pub pairwise: Vec<String>,
    pub proto: Vec<String>,
    pub psk: String,
    pub psk_flags: SecretSettingsFlag,
    pub wep_key_flags: SecretSettingsFlag,
    pub wep_key_type: WEPKeyType,
    pub wep_key0: String,
    pub wep_key1: String,
    pub wep_key2: String,
    pub wep_key3: String,
    pub wep_tx_keyidx: u32,
}

impl PropMapConvert for WifiSecuritySettings {
    fn from_propmap(map: &PropMap) -> Self {
        let authentication_algorithm_opt: Option<&String> = prop_cast(map, "auth-alg");
        let authentication_algorithm =
            if let Some(authentication_algorithm_opt) = authentication_algorithm_opt {
                authentication_algorithm_opt.clone()
            } else {
                String::from("")
            };
        let group_opt: Option<&Vec<String>> = prop_cast(map, "group");
        let group = if let Some(group_opt) = group_opt {
            group_opt.clone()
        } else {
            Vec::new()
        };
        let key_management_opt: Option<&String> = prop_cast(map, "key-mgmt");
        let key_management =
            KeyManagement::from_str(key_management_opt.unwrap_or(&String::from("wpa-psk")));
        let leap_password_opt: Option<&String> = prop_cast(map, "leap-password");
        let leap_password = if let Some(leap_password_opt) = leap_password_opt {
            leap_password_opt.clone()
        } else {
            String::from("")
        };
        let leap_password_flags_opt: Option<&u32> = prop_cast(map, "leap-password-flags");
        let leap_password_flags =
            SecretSettingsFlag::from_i32(*leap_password_flags_opt.unwrap_or(&0) as i32);
        let leap_username_opt: Option<&String> = prop_cast(map, "leap-username");
        let leap_username = if let Some(leap_username_opt) = leap_username_opt {
            leap_username_opt.clone()
        } else {
            String::from("")
        };
        let pairwise_opt: Option<&Vec<String>> = prop_cast(map, "pairwise");
        let pairwise = if let Some(pairwise_opt) = pairwise_opt {
            pairwise_opt.clone()
        } else {
            Vec::new()
        };
        let proto_opt: Option<&Vec<String>> = prop_cast(map, "proto");
        let proto = if let Some(proto_opt) = proto_opt {
            proto_opt.clone()
        } else {
            Vec::new()
        };
        let psk_opt: Option<&String> = prop_cast(map, "psk");
        let psk = if let Some(psk_opt) = psk_opt {
            psk_opt.clone()
        } else {
            String::from("")
        };
        let _psk_flags_opt: Option<&u32> = prop_cast(map, "psk-flags");
        let psk_flags = SecretSettingsFlag::from_i32(*_psk_flags_opt.unwrap_or(&0) as i32);
        let _wep_key_flags_opt: Option<&u32> = prop_cast(map, "wep-key-flags");
        let wep_key_flags = SecretSettingsFlag::from_i32(*_wep_key_flags_opt.unwrap_or(&0) as i32);
        let wep_key_type_opt: Option<&u32> = prop_cast(map, "wep-key-type");
        let wep_key_type = WEPKeyType::from_i32(*wep_key_type_opt.unwrap_or(&0) as i32);
        let wep_key0_opt: Option<&String> = prop_cast(map, "wep-key0");
        let wep_key0 = if let Some(wep_key0_opt) = wep_key0_opt {
            wep_key0_opt.clone()
        } else {
            String::from("")
        };
        let wep_key1_opt: Option<&String> = prop_cast(map, "wep-key1");
        let wep_key1 = if let Some(wep_key1_opt) = wep_key1_opt {
            wep_key1_opt.clone()
        } else {
            String::from("")
        };
        let wep_key2_opt: Option<&String> = prop_cast(map, "wep-key2");
        let wep_key2 = if let Some(wep_key2_opt) = wep_key2_opt {
            wep_key2_opt.clone()
        } else {
            String::from("")
        };
        let wep_key3_opt: Option<&String> = prop_cast(map, "wep-key3");
        let wep_key3 = if let Some(wep_key3_opt) = wep_key3_opt {
            wep_key3_opt.clone()
        } else {
            String::from("")
        };
        let wep_tx_keyidx_opt: Option<&u32> = prop_cast(map, "wep-tx-keyidx");
        Self {
            authentication_algorithm,
            group,
            key_management,
            leap_password,
            leap_password_flags,
            leap_username,
            pairwise,
            proto,
            psk,
            psk_flags,
            wep_key_flags,
            wep_key_type,
            wep_key0,
            wep_key1,
            wep_key2,
            wep_key3,
            wep_tx_keyidx: *wep_tx_keyidx_opt.unwrap_or(&0),
        }
    }

    fn to_propmap(&self) -> PropMap {
        let mut map = PropMap::new();
        if !self.authentication_algorithm.is_empty() {
            map.insert(
                "auth-alg".into(),
                Variant(Box::new(self.authentication_algorithm.clone())),
            );
        }
        map.insert("group".into(), Variant(Box::new(self.group.clone())));
        map.insert(
            "key-mgmt".into(),
            Variant(Box::new(self.key_management.to_string())),
        );
        if !self.leap_password.is_empty() {
            map.insert(
                "leap-password".into(),
                Variant(Box::new(self.leap_password.clone())),
            );
        }

        map.insert(
            "leap-password-flags".into(),
            Variant(Box::new(self.leap_password_flags.to_i32())),
        );

        if !self.leap_username.is_empty() {
            map.insert(
                "leap-username".into(),
                Variant(Box::new(self.leap_username.clone())),
            );
        }

        map.insert("pairwise".into(), Variant(Box::new(self.pairwise.clone())));
        map.insert("proto".into(), Variant(Box::new(self.proto.clone())));
        if !self.psk.is_empty() {
            map.insert("psk".into(), Variant(Box::new(self.psk.clone())));
        }
        if self.wep_key_type.to_i32() == 0 {
            map.insert(
                "wep-key-type".into(),
                Variant(Box::new(self.wep_key_type.to_i32())),
            );
        }

        if !self.wep_key0.is_empty() {
            map.insert("wep-key0".into(), Variant(Box::new(self.wep_key0.clone())));
        }
        if !self.wep_key1.is_empty() {
            map.insert("wep-key1".into(), Variant(Box::new(self.wep_key1.clone())));
        }
        if !self.wep_key2.is_empty() {
            map.insert("wep-key2".into(), Variant(Box::new(self.wep_key2.clone())));
        }
        if !self.wep_key3.is_empty() {
            map.insert("wep-key3".into(), Variant(Box::new(self.wep_key3.clone())));
        }
        map
    }
}
