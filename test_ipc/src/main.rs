use arrow::error as arrow_error;
use arrow::ipc as arrow_ipc;
use arrow::record_batch;
use arrow2::array::Array;
use arrow2::chunk::Chunk;
use arrow2::datatypes::Schema;
use arrow2::error::Result;
use arrow2::io::ipc::read::{read_file_metadata, FileReader};
use std::fs::File;
use std::sync::Arc;

fn read_batches_arrow(path: &str) -> Vec<record_batch::RecordBatch> {
    let file = File::open(path).expect("file not found");
    let reader = arrow_ipc::reader::FileReader::try_new(file).unwrap();
    let batches = reader.collect::<arrow_error::Result<Vec<_>>>().unwrap();
    return batches;
}

fn read_batches_arrow2(path: &str) -> Result<(Schema, Vec<Chunk<Arc<dyn Array>>>)> {
    let mut file = File::open(path)?;
    let metadata = read_file_metadata(&mut file)?;
    let schema = metadata.schema.clone();
    let reader = FileReader::new(file, metadata, None);
    let columns = reader.collect::<Result<Vec<_>>>()?;
    Ok((schema, columns))
}
fn main() -> Result<()> {
    //this works
    let df_1 = read_batches_arrow2("test_julia.ipc");
    println!("{:?}", df_1);

    //this does not work
    let df_2 = read_batches_arrow("test_julia.ipc");
    println!("{:?}", df_2);
    Ok(())
}
