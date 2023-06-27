/*
 * @Author: plucky
 * @Date: 2022-10-27 08:43:53
 * @LastEditTime: 2023-06-27 09:43:28
 * @Description:
 */
#![allow(dead_code)]

#[cfg(test)]
mod tests1 {

    // vec的排序
    fn data_from_vec()->String{
        let mut data = vec![
            ("appId".to_string(), "10".to_string()),
            ("gameId".to_string(), "100".to_string()),
            ("at".to_string(), "1000".to_string()),
        ];
        // 排序
        data.sort_by(|a,b|{
            a.cmp(b)
        });
        // 过滤空值,拼接字符串
        // appId=10&at=1000&gameId=100
        data.iter().filter(|v|{!v.1.is_empty()}).map(|(k,v)| format!("{}={}",k.trim(),v.trim())).collect::<Vec<String>>().join("&")
    }

    // btreemap的自动key排序
    fn data_from_map()->String{
        use std::collections::BTreeMap;
        let mut data = BTreeMap::new();
        data.insert("appId","10".to_string());
        data.insert("gameId","100".to_string());
        data.insert("at","1000".to_string());
        
        // 过滤空值,拼接字符串
        data.iter().filter(|v|{!v.1.is_empty()}).map(|(k,v)| format!("{}={}",k.trim(),v.trim())).collect::<Vec<String>>().join("&")
    }
    
    // 用rsa库测试rsa签名
    #[test]
    fn test_encrypt_by_rsa() {
        use rsa::{RsaPublicKey, pkcs8::DecodePublicKey, Pkcs1v15Encrypt};
        
        let params = data_from_map();
        println!("params: {}", params);
        // 1. md5
        let md5 = md5::compute(params.as_bytes());
        let md5 = format!("{:X}", md5);
        println!("md5: {}", md5);
        // let public_key = include_str!("../public_key.pem");
        let public_key = "MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCj3V68uVdsB5GhhrMQRAanapY9PtGwKsdFPXrNjG/27zp7obYbe0paN4Urs83w5HM4mo3uPLRCJpiTwxkqdwmQ4Mp5DNdb9k2gmnHJG2EcM4/annEdVyg4b3MeKxzCNRUGNWH4Ybic5AA5rjYC/BI0GHUWjbq9+odMKRUyddsvFwIDAQAB";
        // 格式pem,间隔64个字符换行
        let public_key = public_key.as_bytes().chunks(64).map(|c| std::str::from_utf8(c).unwrap()).collect::<Vec<&str>>().join("\n");
        let public_key = format!("-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----", public_key);
        // rsa pkcs8
        let public_key = RsaPublicKey::from_public_key_pem(&public_key).unwrap();
        
        let mut rng = rand::rngs::OsRng;
        let rsau8 = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, md5.as_bytes()).unwrap();
        let sign = base64::encode(rsau8);

        println!("sign: {}", sign);

    }

    // 用openssl测试rsa签名
    #[test]
    fn test_encrypt_by_openssl() {
        use openssl::rsa::{Rsa, Padding};
        
        let params = data_from_vec();
        println!("params: {}", params);

        // 1. md5
        let md5 = md5::compute(params.as_bytes());
        let md5 = format!("{:X}", md5);
        println!("md5: {}", md5);
        // let public_key = include_str!("../public_key.pem");
        let public_key = "MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCj3V68uVdsB5GhhrMQRAanapY9PtGwKsdFPXrNjG/27zp7obYbe0paN4Urs83w5HM4mo3uPLRCJpiTwxkqdwmQ4Mp5DNdb9k2gmnHJG2EcM4/annEdVyg4b3MeKxzCNRUGNWH4Ybic5AA5rjYC/BI0GHUWjbq9+odMKRUyddsvFwIDAQAB";
        // 格式pem,间隔64个字符换行
        let public_key = public_key.as_bytes().chunks(64).map(|c| std::str::from_utf8(c).unwrap()).collect::<Vec<&str>>().join("\n");
        let public_key = format!("-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----", public_key);
        
        let rsa = Rsa::public_key_from_pem(public_key.as_bytes()).unwrap();
        let mut rsa_u8 = vec![0; rsa.size() as usize];
        let len = rsa.public_encrypt(md5.as_bytes(), &mut rsa_u8, Padding::PKCS1).unwrap();
        rsa_u8.truncate(len);
        let sign = base64::encode(rsa_u8);

        println!("sign: {}", sign);
        
    }


    // 用openssl创建rsa公钥私钥
    #[test]
    fn test_generate_key_by_openssl() {
        use openssl::rsa::Rsa;
        
        let rsa = Rsa::generate(1024).unwrap();
        let public_key = rsa.public_key_to_pem().unwrap();
        let private_key = rsa.private_key_to_pem().unwrap();
        println!("public_key: {}", std::str::from_utf8(&public_key).unwrap());
        println!("private_key: {}", std::str::from_utf8(&private_key).unwrap());
        
    }

    // 用rsa库生成rsa公钥私钥
    #[test]
    fn test_generate_key_by_rsa() {
        use rsa::{RsaPrivateKey, pkcs1::EncodeRsaPrivateKey,pkcs8::LineEnding,pkcs1::EncodeRsaPublicKey};
        
        let private_key = RsaPrivateKey::new(&mut rand::rngs::OsRng, 1024).unwrap();
        let public_key = private_key.to_public_key();
        let public_key = public_key.to_pkcs1_pem(LineEnding::LF).unwrap();
        // let public_key = private_key.to_public_key_pem(LineEnding::LF).unwrap_or_default();
        let private_key:String = private_key.to_pkcs1_pem(LineEnding::LF).unwrap().to_string();
        println!("public_key: {}", public_key);
        println!("private_key: {}", private_key);
        
    }
    
}
