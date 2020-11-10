use crate::Result;
use jwks_client::keyset::KeyStore;

pub struct Jwks {
    ks: KeyStore,
}

impl Jwks {
    pub fn load_from_url(url: &str) -> Jwks {
        Jwks {
            ks: KeyStore::new_from(url).unwrap(),
        }
    }

    pub fn verify(&self, token: &str, auds: &Vec<&str>) -> Result<()> {
        match self.ks.verify(token) {
            Ok(jwt) => {
                if jwt.expired().unwrap_or(false) {
                    bail!("jwt expired");
                }
                println!("{:?}",jwt.payload());
                let _auds = jwt.payload().get_array("aud");
                if _auds.is_none(){
                    bail!("no aud");
                }
                let _auds = _auds.unwrap();
                let mut found = false;
                for a in auds {
                    for b in _auds{
                        let b = &b.as_str().unwrap_or("");
                        if a == b{
                            found = true;
                            break; 
                        }
                    }
                }
                if !found {
                    bail!("invalid aud");
                }
                Ok(())
            }
            Err(e) => {
                bail!(format!("key decoding failed: {:?}", e));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_verify() {
        let jwks = Jwks::load_from_url("https://static.nextbillion.io/jwks/nb.ai.pub?2");
        let r = jwks.verify("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6Im5iLmFpIn0.eyJleHAiOjg2NTYwNDk5ODgwMy41NzAxLCJhdWQiOlsicnVzdHRlc3QiXSwiY2lkIjoicnVzdHRlc3QifQ.GrxxibMezoILJ4v109tmetScOWYc2y5vdS42s-FqhbsWqG9oJAIYKnXj-ne0M9VCw09shKWu396QGwq4xyHMM1EPlyd7C4xEE4TMiT8oTXpjH8GHsw_cJtP2JMjucU4RpRuJmq-MaNZ2uUxHrOSlG8iFjRPtdSCVp47pGAgJ6rUT6W8inO0v54LwfBLf6a3bIydTnWa4GaP5__3lFne-DJ2g0KxJDrdu4M7pWewWQP2h21UF8T_WP6ofpc3E4TEab_37LA_O8Aqt34ITCJZDYsJQ8u4OcQ_QhshASxhX4L0t448Yj3WSH4B2ArORuqeY9m_r-UdOrz2SFArplGi59Q",&vec!["rusttest"]);
        println!("verify result: {:?}",r);
        assert!(r.is_ok());
    }
}
