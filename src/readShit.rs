use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::fs::File;
use std::process;

   // start_time: DateTime<Utc>,
   // end_time: DateTime<Utc>,
   // open: f64,
   // high: f64,
   // low: f64,
   // close: f64,
   // volume: f64,
   // trade_count: u64,
   //
   // Open time 	
   // Open 	
   // High 	
   // Low 	
   // Close 	
   // Volume 	
   // Close time 	
   // Quote asset volume 	
   // Number of trades 	
   // Taker buy base asset volume 	
   // Taker buy quote asset volume 	
   // Ignore

fn read_csv() -> Vec<(u64,u64,f64,f64,f64,f64,f64,u64)> {
    //Create File path from constant
    let file_path = Path::new("data/btcusd.csv");
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file_path);
    let ret = Vec::new();
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
        ret.push(record);
    };
    ret
}
