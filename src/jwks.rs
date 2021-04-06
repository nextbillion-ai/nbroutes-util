use crate::Result;
use jwks_client::jwt::Jwt;
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

    pub fn verify_without_auds(&self, token: &str) -> Result<Jwt> {
        let verify_res = self.ks.verify(token);
        if let Err(e) = verify_res {
            bail!(format!("key decoding failed: {:?}", e));
        }

        let jwt = verify_res.unwrap();
        if jwt.expired().unwrap_or(true) {
            // warn!("jwt is expired");
            bail!("jwt expired");
        }

        Ok(jwt)
    }

    pub fn verify(&self, token: &str, auds: &Vec<&str>) -> Result<()> {
        let jwt = self.verify_without_auds(token)?;

        let mut found = false;
        let _auds = jwt.payload().get_array("aud");
        match _auds {
            None => {
                let _aud = jwt.payload().get_str("aud");
                if _aud.is_none() {
                    bail!("no aud");
                }
                let _aud = _aud.unwrap();
                for a in auds {
                    if a == &_aud {
                        found = true;
                        break;
                    }
                }
            }
            Some(v) => {
                'outer: for a in auds {
                    for b in v {
                        let b = &b.as_str().unwrap_or("");
                        if a == b {
                            found = true;
                            break 'outer;
                        }
                    }
                }
            }
        }

        if !found {
            bail!("invalid aud");
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_verify() {
        let jwks = Jwks::load_from_url("https://static.nextbillion.io/jwks/nb.ai.pub?2");
        let r = jwks.verify("eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6Im5iLmFpIn0.eyJleHAiOjg2NTYwNDk5ODgwMy41NzAxLCJhdWQiOlsicnVzdHRlc3QiXSwiY2lkIjoicnVzdHRlc3QifQ.GrxxibMezoILJ4v109tmetScOWYc2y5vdS42s-FqhbsWqG9oJAIYKnXj-ne0M9VCw09shKWu396QGwq4xyHMM1EPlyd7C4xEE4TMiT8oTXpjH8GHsw_cJtP2JMjucU4RpRuJmq-MaNZ2uUxHrOSlG8iFjRPtdSCVp47pGAgJ6rUT6W8inO0v54LwfBLf6a3bIydTnWa4GaP5__3lFne-DJ2g0KxJDrdu4M7pWewWQP2h21UF8T_WP6ofpc3E4TEab_37LA_O8Aqt34ITCJZDYsJQ8u4OcQ_QhshASxhX4L0t448Yj3WSH4B2ArORuqeY9m_r-UdOrz2SFArplGi59Q",&vec!["rusttest"]);
        println!("verify result: {:?}", r);
        assert!(r.is_ok());
    }
    #[test]
    fn test_verify_old() {
        let jwks = Jwks::load_from_url("https://static.nextbillion.io/jwks/nb.ai.pub?2");
        let r = jwks.verify("eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6Im5iLmFpIn0.eyJpc3MiOiJuZXh0YmlsbGlvbi5haSIsInN1YiI6Im5leHRiaWxsaW9uYWkiLCJjaWQiOiJNZWdhQ2FiIiwia2lkIjoiMjA3MTAwMzAxNDk2NzY1NDQwIiwiYXVkIjoibmIiLCJpYXQiOjE1OTU2NzczNTksImV4cCI6MTYyNzIxMzM1OX0.EfRNArJbOblIFXSchb6zF7phNbOb-1JCdAsf9T7pkU0jvPTUNU4Z9bd6GZOnfqorzvobewO_SVAKgpF8Mgqu8g2AzKCQHMvLTuWseyl_as5lzxJJTmMnJhrb2UckD67ycVfVf5ZADXq2QlawT-ffmzPvBOFQaXDCxG2GRVznrqkOoTvaunlyOv9s_HKDTVnYpDm3pKptIlyY-mNj7uC5CWezWI5_a6jr2-RRttEdzziokVl8gfN1Jn67gki34S2ANeRAI0Le2dSWyge66mEC72HGPqk6joiWFa6CZL5dQlOh095XK4fVOfxbZOFu80XcrA5R_eZFcXucmSoGf9dq5Q",&vec!["nb"]);
        println!("verify result: {:?}", r);
        assert!(r.is_ok());
    }
}
