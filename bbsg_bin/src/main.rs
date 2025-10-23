use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //fn main() -> Result<(), Box<dyn Error>> {
    let now = std::time::SystemTime::now();
    let a1 = env::args().nth(1).unwrap_or("?".to_string());
    //let a2 = env::args().nth(2).unwrap_or("?".to_string());
    //let a3 = env::args().nth(3).unwrap_or("?".to_string());
    /*
    let vv3: Vec<Vec<f32>> = vec![
        vec![1f32, 1f32, 1f32],
        vec![1f32, 1f32, 1f32],
        vec![1f32, 1f32, 1f32],
    ];
    let vv6: Vec<Vec<f64>> = vv3
        .iter()
        .map(|v| v.iter().map(|f| *f as f64).collect())
        .collect();
    */
    match a1.as_str() {
        "stage_13" => {
            bbsg_a02::stg1::stage_01()?;
            bbsg_a02::stg2::stage_02()?;
            bbsg_a02::stg3::stage_03()?;
        }
        "stage_23" => {
            bbsg_a02::stg2::stage_02()?;
            bbsg_a02::stg3::stage_03()?;
        }
        "stage_03" => bbsg_a02::stg3::stage_03()?,
        "stage_02" => bbsg_a02::stg2::stage_02()?,
        "stage_01" => bbsg_a02::stg1::stage_01()?,
        "web1" => bbsg_a02::p09::web1().await?,
        n => {
            println!("'{}' NG command", n);
        }
    }
    let se = now.elapsed().unwrap().as_secs();
    let mi = se / 60;
    println!("time {se} sec = {mi} min");
    Ok(())
}
