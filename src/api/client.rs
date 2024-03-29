use crate::api::structure::*;

use std::collections::HashMap;

use reqwest::Client;
use reqwest_middleware::{ClientBuilder, Result};
use http_cache_reqwest::{Cache, CacheMode, CACacheManager, HttpCache};

pub struct PIPED{
    api_url:String,
    region:String,
    client:reqwest_middleware::ClientWithMiddleware,
}

impl PIPED{
    pub fn new() -> PIPED{
        PIPED{
            api_url:"https://pipedapi.kavin.rocks".to_string(),
            region:"US".to_string(),
            client:ClientBuilder::new(Client::new()).with(
                        Cache(HttpCache {
                                 mode: CacheMode::Default,
                                 manager: CACacheManager::default(),
                                 options: None,
                            })).build()
        }
    }

    // Utilities
    fn create_url_from_endpoint(&self,endpoint:&str)->String{
        format!("{}/{}",self.api_url,endpoint)
    }

    // Authentication/Authenticated endpoints
    pub async fn login(&self,username:&str,password:&str)->Result<AuthResponse>{
        let url=self.create_url_from_endpoint("/login");
        let mut body = HashMap::new();
        body.insert("username",username);
        body.insert("password",password);
        let response = self.client.post(url).json(&body).send().await?;
        let result = response.json::<AuthResponse>().await?;
        Ok(result)
    }
    pub async fn register(&self,username:&str,password:&str)->Result<AuthResponse>{
        let url=self.create_url_from_endpoint("/register");
        let mut body = HashMap::new();
        body.insert("username",username);
        body.insert("password",password);
        let response = self.client.post(url).json(&body).send().await?;
        let result = response.json::<AuthResponse>().await?;
        Ok(result)
    }

    pub async fn feed(&self)->Result<FeedResponse>{
        let url=self.create_url_from_endpoint("/feed");
        let auth_token = "c0c64d6f-c2e7-4294-b1ad-d305eabb2227";
        let response = self.client.get(url).query(&[("authToken",auth_token)]).send().await?;
        let result = response.json::<FeedResponse>().await?;
        Ok(result)
    }
    
    // External
    pub async fn get_instances(&self) -> Result< Vec<PipedInstance>> {
        let request_url = "https://raw.githubusercontent.com/wiki/TeamPiped/Piped-Frontend/Instances.md";
        let response = self.client.get(request_url).send().await?;
        let data=response.text().await?;
        
        let lines=data.split('\n').collect::<Vec<&str>>();
    
        fn parse_line(line:&str) -> Option<PipedInstance>{
            let split = line.split('|').collect::<Vec<&str>>();
            if split.len() > 4 {
                Some(PipedInstance{
                    name: split[0].trim().to_string(),
                    url: split[1].trim().to_string(),
                    cdn: (split[3].trim()=="Yes"),
                    locations: split[2].trim().to_string()
                })
            }
            else{
                None
            }
        }
    
        print!("{}",lines.len());
        let mut parsed_lines:Vec<PipedInstance> = Vec::new();
    
        for i in 0..lines.len(){
            // skipping table headers and stuff
            if i < 4{
                continue;
            }
            let x = parse_line(lines[i]);
            if x.is_some(){
                parsed_lines.push(x.unwrap());
            }
        }
        Ok(parsed_lines)
    }
    
    // to get resources like images/thumbnails etc
    pub async fn get_resource(&self,url:&str)->Result<bytes::Bytes>{
        let response = self.client.get(url).send().await?;
        let data = response.bytes().await?;
        Ok(data)
    }
    
    // Base Piped API implementation
    pub async fn trending(&self)->Result< Vec<VideoDetail>>{
        let url = self.create_url_from_endpoint("/trending");
        let response = self.client.get(url).query(&[("region",&self.region)]).send().await?;
        let data = response.json::<Vec<VideoDetail>>().await?;
        print!("{:?}",data);
        Ok(data)
    }

    pub async fn stream(&self,video_id:&str)->Result<VideoStreamResponse>{
        let url = self.create_url_from_endpoint(&format!("/streams/{}",video_id));
        let response = self.client.get(url).send().await?;
        let data = response.json::<VideoStreamResponse>().await?;
        Ok(data)
        
    }

    pub async fn comments(&self,video_id:&str)->Result< Comments >{
        let url = self.create_url_from_endpoint(&format!("/comments/{}",video_id));
        let response = self.client.get(url).send().await?;
        let data = response.json::<Comments>().await?;
        Ok(data)
    }

    pub async fn channel_from_id(&self,channel_id:&str) -> Result<ChannelResponse>{
        let url = self.create_url_from_endpoint(&format!("/channel/{}",channel_id));
        let response = self.client.get(url).send().await?;
        let data = response.json::<ChannelResponse>().await?;
        Ok(data)
    }
    // broken 
    //pub async fn channel_from_name(&self,channel_name:&str)->Result<Channel>{
        //let url = self.create_url_from_endpoint(&format!("/c/{}",channel_name));
        //let response = self.client.get(url).send().await?;
        //let data = response.json::<Channel>().await?;
        //Ok(data)
    //}
    pub async fn channel_from_username(&self,username:&str)->Result<ChannelResponse>{
        let url = self.create_url_from_endpoint(&format!("/user/{}",username));
        let response = self.client.get(url).send().await?;
        let data = response.json::<ChannelResponse>().await?;
        Ok(data)
    }
    pub async fn suggestion(&self,query:&str) ->Result< Vec<String> >{
        let url = self.create_url_from_endpoint("/suggestions");
        let response = self.client.get(url).query(&[("query",query)]).send().await?;
        let data = response.json::<Vec<String>>().await?;
        Ok(data)
    }
    pub async fn search(&self,query:&str,filter:SearchFilters)->Result<SearchResponse>{
        let url = self.create_url_from_endpoint("/search");
        let response = self.client.get(url).query(&[("q",query),("filter",filter.to_string())]).send().await?;
        let result = response.json::<SearchResponse>().await?;
        Ok(result)
        
    }

}

