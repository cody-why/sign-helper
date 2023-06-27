/*
 * @Author: anger
 * @Date: 2023-06-27 07:32:04
 * @LastEditTime: 2023-06-27 09:44:10
 * @Description: 
 */

use rsa::{RsaPublicKey, pkcs8::DecodePublicKey, Pkcs1v15Encrypt};
pub struct Params{
    pub data: Vec<(String,String)>,
}
impl Default for Params {
    fn default() -> Self {
        Self {
            data: vec![
                ("appId".to_string(),"".to_string()),
                ("gameId".to_string(),"".to_string()),
                ("at".to_string(),"".to_string()),
            ]
        }
    }
    
}
impl Params {
   
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_params(&mut self, key: String, value: String) {
        self.data.push((key,value));
    }

    pub fn remove_params(&mut self, id: usize) {
        self.data.remove(id);
    }

    pub fn update_params(&mut self, id: usize, key: String, value: String) {
        self.data[id] = (key,value);
    }
    
    /// 生成签名
    pub fn generate_sign(&self, public_key: &str) -> String {
        // 过滤空值
        let mut data = self.data.iter().filter(|(k,v)|{!k.is_empty() && !v.is_empty()}).collect::<Vec<_>>();
        // 排序
        data.sort_by(|a,b|{
            a.0.cmp(&b.0)
        });
        // 拼接
        let params = data.iter()
        .map(|(k,v)| format!("{}={}",k.trim(),v.trim()))
        .collect::<Vec<String>>().join("&");
        let md5 = md5::compute(params.as_bytes());
        let md5 = format!("{:X}", md5);

        // rsa pkcs8
        let public_key = RsaPublicKey::from_public_key_pem(public_key).unwrap();
        
        let mut rng = rand::rngs::OsRng;
        let rsau8 = public_key.encrypt(&mut rng, Pkcs1v15Encrypt, md5.as_bytes()).unwrap();
        let sign = base64::encode(rsau8);
        // let json =  self.generate_json(&data, &sign);

        format!("params: {}\nmd5: {:}\nsign: {}\n", params, md5, sign)
    }

    #[allow(dead_code)]
    pub fn generate_json(&self,data: &Vec<&(String, String)>, sign: &str) -> String {
        let mut json = String::with_capacity(1024);
        json.push('{');
        for (k,v) in data {
            json.push_str(&format!("\"{}\":\"{}\",",k,v));
        }
        json.push_str(&format!("\"sign\":\"{}\"",sign));
        json.push('}');
        json
       
    }


}