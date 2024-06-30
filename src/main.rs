mod chunked_iter;
use std::collections::HashMap;
use std::fs::File;
use std::{cmp, io};
use std::io::prelude::*;
use std::time::Instant;

use chunked_iter::chunked_iter;

fn main() -> io::Result<()> {
    const CHUNK_SIZE: usize = 1<<20;
    const FNAME: &str = "measurements-1000000000.txt";

    let start = Instant::now();

    let fp = File::open(FNAME)?;

    let mut locations: HashMap<[u8; 32], (i32,i32,i32,i32)> = HashMap::new();

    for line in chunked_iter::<CHUNK_SIZE>(fp) {
        //println!("{line:#?}");
        //let name = String::from_utf8(line.name.to_vec()).unwrap();
        //let temp = line.value;
        //println!("{name} {temp}");
        let result = locations.get_mut(&line.name);

        match result {
            Some((min,max,sum,count)) => {
                *sum+=line.value;
                *min = cmp::min(*min, line.value);
                *max = cmp::max(*max, line.value);
                *count+=1;
            },
            None => {
                locations.insert(line.name, (line.value,line.value,line.value,1));
            },
        }
    }

    let mut stations:Vec<([u8;32],(i32,i32,i32,i32))> = locations.into_iter().collect();

    stations.sort_by(|a,b| a.0.cmp(&b.0));

    for (loc,(min,max,sum,count)) in stations {
        let name = String::from_utf8(loc.to_vec()).unwrap();
        let min= (min as f32)/10.0;
        let max= (max as f32)/10.0;
        let mean= ((sum/count) as f32)/10.0;
        println!("{name} {min:.1} {max:.1} {mean:.1}");
    }

    let duration = start.elapsed();

    println!("duration: {duration:?}");
    Ok(())
}
