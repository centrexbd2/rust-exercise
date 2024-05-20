use std::path::Path;

use libmdbx::{Database, NoWriteMap, WriteFlags};

fn main() {
    let db = Database::<NoWriteMap>::open(Path::new("_db")).unwrap();

    // write
    {
        let txn = db.begin_rw_txn().unwrap();
        let table = txn.open_table(None).unwrap();
        for i in 0..100 {
            txn.put(
                &table,
                &format!("key{}", i),
                &format!("data{}", i),
                WriteFlags::empty(),
            )
            .unwrap();
            // delete
            if i > 50 {
                txn.del(&table, &format!("key{}", i), None).unwrap();
            }
        }
        txn.commit().unwrap();
    }
    // read
    {
        let txn = db.begin_ro_txn().unwrap();
        let table = txn.open_table(None).unwrap();
        for i in 0..100 {
            println!(
                "{:?}",
                txn.get::<Vec<u8>>(&table, format!("key{}", i).as_bytes())
                    .unwrap()
            );
        }
    }
}