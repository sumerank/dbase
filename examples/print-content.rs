extern crate dbase;

fn main() {
    let dbf_path = std::env::args().nth(1).expect("Path to file as first arg");
    let mut reader = dbase::Reader::from_path(&dbf_path).unwrap();
    println!("{}", reader.header().size_of_record);
    // let mut records = reader.iter_records().collect::<Result<Vec<_>,_>>().unwrap();
    //
    //
    // let r = records.clone();
    //
    // while records.len() < 500_000 {
    //     let mut tmp = r.clone();
    //     records.append(&mut tmp);
    // }
    //
    // let mut writer = dbase::TableWriterBuilder::from_reader(reader).build_with_file_dest("lol.dbf").unwrap();
    // writer.write_records(&records).unwrap();

    let t = std::time::Instant::now();
    let mut records = Vec::with_capacity(reader.header().num_records as usize);
    for record in reader.iter_records() {
        // let tt = std::time::Instant::now();
        records.push(record.unwrap());
        // println!("time to read one record: {:?}", tt.elapsed());
    }
    println!("Time to read via reader: {:?}", t.elapsed());

    let mut file = dbase::File::open_read_only(&dbf_path).unwrap();
    let t = std::time::Instant::now();
    let mut records = Vec::with_capacity(file.num_records() * file.fields().len());
    let num_fields = file.fields().len();
    let num_records = file.num_records();
    let mut iter = file.records();
    // let mut c = 0u64;
    loop {
        // let tt = std::time::Instant::now();
        let Some(mut record) = iter.next() else {
            break;
        };
        // println!("time to read one record: {:?}", tt.elapsed());

        for i in 0..num_fields {
            records.push(record.field(dbase::FieldIndex(i)).unwrap().read());
        }
    }
    println!("Time to read via file: {:?}", t.elapsed());

    let mut file = dbase::File::open_read_only(dbf_path).unwrap();
    let t = std::time::Instant::now();
    let mut records = Vec::with_capacity(file.num_records());
    let mut iter = file.records();
    while let Some(mut record) = iter.next() {
        records.push(record.read().unwrap());
    }
    println!("Time to read via file: {:?}", t.elapsed());

    // for (i, record_result) in reader.iter_records().enumerate() {
    //     println!("Record {}", i);
    //     let record = record_result.unwrap();
    //     for (name, value) in record {
    //         println!("\tname: {}, value: {:?}", name, value);
    //     }
    // }
}

//
// use std::fs::File;
// use std::io::{BufReader, SeekFrom};
// use std::io::prelude::*;
// use byteorder::WriteBytesExt;
//
// fn main() -> std::io::Result<()> {
//     let mut file = File::options().read(true).write(true).truncate(false).open("foo.txt")?;
//     let mut file_copy = BufReader::new(file.try_clone()?);
//
//     let mut contents = vec![0u8; 3];
//     file_copy.read_exact(&mut contents)?;
//     println!("contents: {:?}", contents);
//
//     // file.seek(SeekFrom::Start(2)).unwrap();
//     file.write_u8(33).unwrap();
//
//     // file_copy.seek(SeekFrom::Start(0)).unwrap();
//     file_copy.read_exact(&mut contents).unwrap();
//     println!("contents: {:?}", contents);
//
//     Ok(())
// }
