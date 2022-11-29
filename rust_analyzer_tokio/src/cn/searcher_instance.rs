use std::fs::{File, read};
use std::io::{BufReader, BufRead};
use std::{fs, io::{Result, Seek}, collections::HashMap};

fn _read_index_file(file_name: &String) -> Result<HashMap<usize, (u64, u64)>> {
    let mut x: HashMap<usize, (u64, u64)> = HashMap::new();
    let y = fs::read_to_string(file_name)?;
    let mut count = 0;
    for line in y.lines() {
        let (start, end) = line.split_once(",").unwrap() ;
        x.insert(count, (start.parse().unwrap(), end.parse().unwrap()));
        count += 1;
    }
    Ok(x)
}

fn _search_index_file() {

}

fn _search_log_reading_entire_file(file_name: &String, reference: Vec<(u64, u64)>) -> Result<Vec<String>> {
    let z = read(file_name)?;
    let mut match_log: Vec<String> = Vec::new();
    for (start, end) in reference {
        match_log.push(std::str::from_utf8(&z[start as usize..end as usize]).unwrap().to_string());
    }
    Ok(match_log)
}

fn _search_log_file_from_pointer(file_name: &String, reference: Vec<(u64, u64)>) -> Result<Vec<String>> {
    let mut z = BufReader::new(File::open(file_name)?);
    let mut match_log: Vec<String> = Vec::new();
    let mut y = z.stream_position()?;
    for (start, end) in reference {
        let mut str = String::with_capacity((end-start).try_into().unwrap());
        if start == y {
        } else if start > y {
            z.seek(std::io::SeekFrom::Current((start-y).try_into().unwrap()))?;          
        } else {
            z.seek(std::io::SeekFrom::Start(start))?;
        }
        z.read_line(&mut str)?; 
        match_log.push(str);
        y = end;
    }
    Ok(match_log)
}
