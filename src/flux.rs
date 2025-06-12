// use reqwest;
use anyhow::Result;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use chrono::Local;
use reqwest::blocking::Client;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Write};

use crate::cli;
// use std::path::PathBuf;

// use directories::UserDirs;

pub trait Generate {
    fn generate(&self, payload: Payload) -> Result<String>;
}

#[derive(Debug, Serialize, Clone)]
pub struct Payload {
    prompt: Option<String>,
    input_image: Option<String>, // base64 encoded
    seed: Option<i32>,
    aspect_ratio: Option<String>, // 21:9 - 9:21
    output_format: Option<String>,
    webhook_url: Option<String>,
    webhook_secret: Option<String>,
    prompt_upsampling: bool,
    safety_tolerance: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct GenerationResponse {
    id: String,
    polling_url: String,
}

#[derive(Serialize)]
pub struct PollingRequest {
    id: String,
}

#[derive(Debug, Deserialize)]
pub struct PollingInnerResult {
    sample: String,
    prompt: String,
    seed: i32,
    start_time: f64,
    end_time: f64,
    duration: f64,
}

#[derive(Debug, Deserialize)]
pub struct PollingResponse {
    id: String,
    status: String,
    result: Option<PollingInnerResult>,
    progress: Option<i32>,
    details: Option<serde_json::Value>,
}

pub struct Kontext {
    model: String,
    payload: Payload,
}

impl Default for Payload {
    fn default() -> Self {
        Payload {
            prompt: Some("A cute cat".to_string()),
            input_image: None,
            seed: None,
            aspect_ratio: Some("1:1".to_string()),
            output_format: Some("png".to_string()),
            webhook_url: None,
            webhook_secret: None,
            prompt_upsampling: false,
            safety_tolerance: Some(6),
        }
    }
}

impl Generate for Kontext {
    fn generate(&self, payload: Payload) -> Result<String> {
        // let payload = Payload {
        //     prompt: Some("an action hero looking up at alien ships".to_string()),
        //     ..Default::default()
        // };
        //
        // println!("{:?}", payload);

        let api_key = env::var("BFL_API_KEY").expect("Set BFL_API_KEY");

        let mut headers = HeaderMap::new();
        headers.insert("x-key", HeaderValue::from_str(&api_key)?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let client = Client::new();

        let model_url = format!("https://api.bfl.ai/v1/{}", self.model);

        // println!("URL: {}", model_url);

        let res = client
            .post(model_url)
            .headers(headers.clone())
            .json(&payload)
            .send()?
            .error_for_status()?
            .json::<GenerationResponse>()?;

        // println!("Status: {:?}", res);

        let polling_id = PollingRequest { id: res.id };
        let url = res.polling_url;

        loop {
            let polling_res = client
                .get(&url)
                .headers(headers.clone())
                .json(&polling_id)
                .send()?
                .error_for_status()?
                .json::<PollingResponse>()?;

            // println!("{:?}", polling_res);

            match polling_res.status.as_str() {
                "Task not found" => println!("Task not found"),
                "Pending" => {
                    io::stdout().flush().unwrap();
                    print!(".");
                }
                "Request Moderation" => println!("Request Moderation"),
                "Content Moderated" => println!("Moderated"),
                "Error" => println!("There was an Error"),
                _ => {
                    println!("\nFinished Generating");
                    let image_url = match polling_res.result {
                        Some(val) => val.sample,
                        None => {
                            println!("Couldn't find image download link");
                            continue;
                        }
                    };

                    let now = Local::now();

                    let image_path =
                        format!("{}{}", now.format("%d-%m-%Y_%H-%M-%S"), "_flux-gen.png");

                    let response = reqwest::blocking::get(image_url);

                    // println!("{:?}", response);

                    let mut image = match response {
                        Ok(resp) => resp,
                        Err(resp) => {
                            println!("Error getting image");
                            continue;
                        }
                    };

                    // if image.status().is_success() {
                    //     println!("")
                    // }
                    if !image.status().is_success() {
                        println!("Error fetching image");
                        continue;
                    }

                    // println!("Got image");

                    // std::env::current_dir()?.join(&image_path)
                    let mut file =
                        std::fs::File::create(std::env::current_dir()?.join(&image_path))?;

                    std::io::copy(&mut image, &mut file)?;

                    println!("Image saved to {:?}", image_path);

                    break;
                } // _ => println!("Something else happened"),
            }
            // std::thread::sleep(std::time::Duration::from_millis(500));
        }

        // let b64img = client.get("https://api.bfl.ai/v1/get_result").headers(headers).json()

        Ok(STANDARD.encode("test"))

        // let res: Response = reqwest::Client::new()
        //     .post()
    }
}

pub fn create(args: cli::Args) -> Result<String> {
    let b64img = match args.image {
        Some(img) => {
            let content = std::fs::read(img)?;
            Some(STANDARD.encode(content))
        }
        None => None,
    };

    let model_string = match args.model {
        cli::Model::Kontext => "flux-kontext-max",
        cli::Model::KontextMax => "flux-kontext-max",
        cli::Model::KontextPro => "flux-kontext-pro",
    };

    let model = Kontext {
        model: model_string.to_string(),
        payload: Payload {
            prompt: args.prompt,
            input_image: b64img,
            seed: args.seed,
            aspect_ratio: args.aspect_ratio,
            prompt_upsampling: args.prompt_upsampling,
            ..Default::default()
        },
    };

    let image = model.generate(model.payload.clone());

    Ok("\nGeneration Success".to_string())
}

// use std::fs::File;
// use std::io::BufWriter;
// use image::{RgbImage, ImageOutputFormat};
//
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Get user directories
//     let user_dirs = UserDirs::new().ok_or("Could not get user directories")?;
//
//     // Get the pictures directory
//     let pictures_dir = user_dirs.picture_dir()
//         .ok_or("Could not locate the Pictures directory")?;
//
//     // Make the full path
//     let mut image_path = PathBuf::from(pictures_dir);
//     image_path.push("my_image.png");
//
//     // Create dummy image (for demonstration)
//     let img = RgbImage::new(100, 100);
//
//     // Save the image
//     let file = File::create(&image_path)?;
//     let w = BufWriter::new(file);
//     img.write_to(&mut BufWriter::new(w), ImageOutputFormat::Png)?;
//
//     println!("Image saved to {:?}", image_path);
//     Ok(())
// }
