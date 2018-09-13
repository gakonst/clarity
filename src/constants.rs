use std::str::FromStr;
use types::BigEndianInt;

lazy_static! {
    pub static ref TT256 : BigEndianInt = BigEndianInt::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639936").unwrap(); // 2 ** 256
    pub static ref TT256M1 : BigEndianInt = BigEndianInt::from_str("115792089237316195423570985008687907853269984665640564039457584007913129639935").unwrap(); // 2 ** 256 - 1
    pub static ref TT255 : BigEndianInt= BigEndianInt::from_str("57896044618658097711785492504343953926634992332820282019728792003956564819968").unwrap(); //2 ** 255
    pub static ref SECP256K1P : BigEndianInt = BigEndianInt::from_str("115792089237316195423570985008687907853269984665640564039457584007908834671663").unwrap(); // 2**256 - 4294968273
    pub static ref SECPK1N : BigEndianInt = BigEndianInt::from_str("115792089237316195423570985008687907852837564279074904382605163141518161494337").unwrap();
}
