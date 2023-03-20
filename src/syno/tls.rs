/*

SynoDL - CLI for Synology's DownloadStation
Copyright (C) 2015 - 2023  Stefan Ott

This program is free software: you can redistribute it and/or
modify it under the terms of the GNU General Public License as
published by the Free Software Foundation, version 3.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.

*/

use std::error;
#[cfg(not(feature = "insecure_tls"))]
use std::{io::{self, ErrorKind}};

use rustls::{self, ClientConfig, RootCertStore, Certificate};

#[cfg(feature = "custom_ca")]
use {
    std::{iter, fs::File, io::BufReader},
    rustls_pemfile::{Item, read_one}
};

#[cfg(feature = "insecure_tls")]
use {
    std::{time::SystemTime, sync::Arc},
    rustls::client::{ServerCertVerifier, ServerCertVerified},
    rustls::ServerName
};

#[cfg(test)]
use mockall::automock;

use crate::{
    Config
};


fn tls_default() -> Result<ClientConfig, Box<dyn error::Error>>
{
    let mut root_store = RootCertStore::empty();
    let certs = rustls_native_certs::load_native_certs().expect("Could not load platform certs");
    for cert in certs {
        let rustls_cert = Certificate(cert.0);
        root_store
            .add(&rustls_cert)
            .expect("Failed to add native certificate to root store");
    }

    let cfg = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    Ok(cfg)
}

#[cfg(feature = "custom_ca")]
fn tls_custom_cert(path: &String) -> Result<ClientConfig, Box<dyn error::Error>>
{
    let mut root_store = RootCertStore::empty();

    let mut f = File::open(&path)?;
    let mut crt = BufReader::new(&mut f);
    for item in iter::from_fn(|| read_one(&mut crt).transpose()) {
        match item.unwrap() {
            Item::X509Certificate(cert) => {
                let xx = Certificate(cert);
                root_store
                    .add(&xx)
                    .expect("Failed to add native certificate to root store");
            },
            _ => println!("unhandled item"),
        }
    }

    let cfg = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    Ok(cfg)
}

#[cfg(not(feature = "custom_ca"))]
fn tls_custom_cert(_path: &String) -> Result<ClientConfig, Box<dyn error::Error>>
{
    Err(Box::new(io::Error::new(ErrorKind::Other,
        "Please enable 'custom_ca' feature to specify your own CA certificate")))
}

#[cfg(feature = "insecure_tls")]
fn tls_ignore_cert() -> Result<ClientConfig, Box<dyn error::Error>>
{
    struct DummyVerifier { }

    impl DummyVerifier {
        fn new() -> Self {
            DummyVerifier { }
        }
    }

    impl ServerCertVerifier for DummyVerifier {
        fn verify_server_cert(
            &self,
            _: &Certificate,
            _: &[Certificate],
            _: &ServerName,
            _: &mut dyn Iterator<Item = &[u8]>,
            _: &[u8],
            _: SystemTime
        ) -> Result<ServerCertVerified, rustls::Error> {
            return Ok(ServerCertVerified::assertion());
        }
    }

    let dummy_verifier = Arc::new(DummyVerifier::new());

    let cfg = ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(dummy_verifier)
        .with_no_client_auth();

    Ok(cfg)
}

#[cfg_attr(test, automock)]
trait TLSCertificateBehaviour {
    fn default_certs(&self) -> Result<ClientConfig, Box<dyn error::Error>>;
    fn custom_cert(&self, path: &String) -> Result<ClientConfig, Box<dyn error::Error>>;
    fn ignore_cert(&self) -> Result<ClientConfig, Box<dyn error::Error>>;
}

struct MyCertificateBehaviour {
}

impl TLSCertificateBehaviour for MyCertificateBehaviour {
    fn default_certs(&self) -> Result<ClientConfig, Box<dyn error::Error>> {
        tls_default()
    }
    fn custom_cert(&self, path: &String) -> Result<ClientConfig, Box<dyn error::Error>> {
        tls_custom_cert(path)
    }
    fn ignore_cert(&self) -> Result<ClientConfig, Box<dyn error::Error>> {
        tls_ignore_cert()
    }
}

#[cfg(not(feature = "insecure_tls"))]
fn tls_ignore_cert() -> Result<ClientConfig, Box<dyn error::Error>>
{
    Err(Box::new(io::Error::new(ErrorKind::Other,
        "Please enable 'insecure_tls' feature to ignore the CA certificate")))
}

fn find_tls_config(cacert: &Option<String>, options: &impl TLSCertificateBehaviour)
    -> Result<ClientConfig, Box<dyn error::Error>> {
    match cacert {
        Some(f) => {
            if f == "ignore" {
                options.ignore_cert()
            } else {
                options.custom_cert(&f)
            }
        }
        None => {
            options.default_certs()
        }
    }
}

pub fn get_tls_config(cfg: &Config) -> Result<ClientConfig, Box<dyn error::Error>> {
    find_tls_config(&cfg.cacert, &MyCertificateBehaviour {})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_tls_config() {
        let root_store = RootCertStore::empty();
        let mut mock = MockTLSCertificateBehaviour::new();
        let client = ClientConfig::builder()
                    .with_safe_defaults()
                    .with_root_certificates(root_store)
                    .with_no_client_auth();
        let client2 = client.clone();
        let client3 = client.clone();

        mock.expect_default_certs()
            .times(1)
            .return_once(|| Ok(client));
        mock.expect_custom_cert()
            .times(0)
            .return_once(|_| Ok(client2));
        mock.expect_ignore_cert()
            .times(0)
            .return_once(|| Ok(client3));

        let _ = find_tls_config(&None, &mock);
    }

    #[test]
    fn test_tls_config_custom_certificate() {
        let root_store = RootCertStore::empty();
        let mut mock = MockTLSCertificateBehaviour::new();
        let client = ClientConfig::builder()
                    .with_safe_defaults()
                    .with_root_certificates(root_store)
                    .with_no_client_auth();
        let client2 = client.clone();
        let client3 = client.clone();

        mock.expect_default_certs()
            .times(0)
            .return_once(|| Ok(client));
        mock.expect_custom_cert()
            .times(1)
            .return_once(|_| Ok(client2));
        mock.expect_ignore_cert()
            .times(0)
            .return_once(|| Ok(client3));

        let _ = find_tls_config(&Some(String::from("/tmp/test.pem")), &mock);
    }

    #[test]
    fn test_tls_config_ignore_certificate() {
        let root_store = RootCertStore::empty();
        let mut mock = MockTLSCertificateBehaviour::new();
        let client = ClientConfig::builder()
                    .with_safe_defaults()
                    .with_root_certificates(root_store)
                    .with_no_client_auth();
        let client2 = client.clone();
        let client3 = client.clone();

        mock.expect_default_certs()
            .times(0)
            .return_once(|| Ok(client));
        mock.expect_custom_cert()
            .times(0)
            .return_once(|_| Ok(client2));
        mock.expect_ignore_cert()
            .times(1)
            .return_once(|| Ok(client3));

        let _ = find_tls_config(&Some(String::from("ignore")), &mock);
    }
}
