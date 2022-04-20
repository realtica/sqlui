use crossbeam_channel::bounded;
use log::info;
use mysql::prelude::*;
use mysql::*;
use once_cell::unsync::OnceCell;
#[derive(Clone, Default)]
pub struct MySqlManager {
    pub pool: OnceCell<Pool>,
    pub db_name: String,
    pub table_selected: String,
    pub filter: String,
    pub connection_string: String,
    pub offset: u32,
    pub limit: u32,
    pub number_of_pages: u32,
    pub page: u32,
    pub number_of_rows: u32,
}

impl MySqlManager {
    pub async fn _create_pool(&mut self) -> Result<Pool> {
        let pool = self.pool.get_or_try_init(|| {
            let opts = Opts::from_url(self.connection_string.as_str())?;
            let p = Pool::new(opts);
            p
        })?;
        Ok(pool.clone())
    }
    // pub async fn _conn(url: String) -> Result<Self> {
    //     let opts = Opts::from_url(url.as_str())?;
    //     let pool = Pool::new(opts)?;
    //     // let conn = pool.get_conn()?;
    //     let split_iterator = url.split("/");
    //     let split: Vec<&str> = split_iterator.collect();
    //     let db_name = split[split.len() - 1].to_string();
    //     let result = MySqlManager {
    //         pool,
    //         db_name,
    //         table_selected: String::new(),
    //         filter: String::new(),
    //     };
    //     Ok(result)
    // }
    // pub fn conn(url: String) -> Self {
    //     let (tx, rx) = bounded(1);
    //     smol::block_on(async {
    //         let res = MySqlManager::_conn(url).await;
    //         let _ = tx.send(res.unwrap());
    //     });
    //     let db_manager: MySqlManager = rx.recv().unwrap();
    //     db_manager
    // }
    pub async fn _get_tables_from_db(&mut self) -> Result<Vec<String>> {
        log::info!("get_all_tables");
        let query_string = format!(
            "SELECT TABLE_NAME AS name 
FROM INFORMATION_SCHEMA.TABLES 
WHERE TABLE_SCHEMA = '{}' ORDER BY TABLE_NAME ASC;",
            self.db_name
        );
        let pool: Pool = self._create_pool().await?;
        let rows: Vec<Row> = pool.get_conn()?.query(query_string)?;
        info!("{:?}", pool);
        let mut tables: Vec<String> = Vec::new();
        for row in rows {
            info!("{:?}", row[0]);
            tables.push(from_value::<String>(row[0].clone()));
        }
        Ok(tables)
    }
    pub fn get_tables_from_db(&mut self) -> Vec<String> {
        let (tx, rx) = bounded(1);
        smol::block_on(async {
            let res = self._get_tables_from_db().await;
            let _ = tx.send(res.unwrap());
        });
        let tables: Vec<String> = rx.recv().unwrap();
        tables
    }
    pub async fn _get_columns_from_table(&mut self) -> Result<(Vec<String>, Vec<i32>)> {
        log::info!("get_all_columns");
        let query_string = format!(
            "SELECT COLUMN_NAME, DATA_TYPE 
  FROM INFORMATION_SCHEMA.COLUMNS
  WHERE TABLE_SCHEMA = '{}' AND TABLE_NAME = '{}';",
            self.db_name, self.table_selected
        );
        let pool: Pool = self._create_pool().await?;
        let rows: Vec<Row> = pool.get_conn()?.query(query_string)?;
        // info!("{}", query_result.affected_rows());
        let mut columns: Vec<String> = Vec::new();
        let mut types: Vec<i32> = Vec::new();
        for col in rows {
            info!("{}", from_value::<String>(col[0].clone()));
            info!("{}", from_value::<String>(col[1].clone()));
            columns.push(from_value::<String>(col[0].clone()));
            // types.push(from_value::<String>(col[1].clone()));
            let col_size = match from_value::<String>(col[1].clone()).as_str() {
                "int" => 70,
                "varchar" => 250,
                "char" => 150,
                "timestamp" => 170,
                "datetime" => 170,
                "text" => 300,
                _ => 100,
            };
            types.push(col_size);
        }
        Ok((columns, types))
    }
    pub async fn _get_rows_from_table(&mut self) -> Result<Vec<Row>> {
        log::info!("get rows..");
        let mut filter = String::from("");
        if self.filter.len() > 0 {
            filter = format!("WHERE {}", self.filter);
        }
        let query_string = format!(
            "SELECT * FROM {} {} LIMIT 100 OFFSET {};",
            self.table_selected, filter, self.offset
        );
        let pool: Pool = self._create_pool().await?;
        let rows: Vec<Row> = pool.get_conn()?.query(query_string)?;
        Ok(rows)
    }

    pub fn get_columns_from_table(&mut self) -> (Vec<String>, Vec<i32>) {
        let (tx, rx) = bounded(1);
        smol::block_on(async {
            let res = self._get_columns_from_table().await;
            let _ = tx.send(res.unwrap());
        });
        let (columns, types) = rx.recv().unwrap();
        (columns, types)
    }

    pub fn get_rows_from_table(&mut self) -> Vec<Row> {
        let (tx, rx) = bounded(1);
        smol::block_on(async {
            let res = self._get_rows_from_table().await;
            let _ = tx.send(res.unwrap());
        });
        let rows: Vec<Row> = rx.recv().unwrap();
        rows
    }

    pub fn select_table(&mut self, table: String) {
        self.table_selected = table;
        self.filter = String::new();
        self.init_values();
    }
    
    pub fn init_values(&mut self) {
        self.limit = 100;
        self.offset = 0;
        self.page = 1;
        self.set_number_of_pages();
    }

    pub fn next(&mut self) {
        self.offset += self.limit;
        self.page += 1;
    }

    pub fn prev(&mut self) {
        self.offset -= self.limit;
        self.page -= 1;
    }

    pub async fn _set_number_of_pages(&mut self) -> Result<Option<u32>> {
        let mut filter = String::from("");
        if self.filter.len() > 0 {
            filter = format!("WHERE {}", self.filter);
        }
        let query_string = format!("SELECT COUNT(*) FROM {} {};", self.table_selected, filter);
        let pool: Pool = self._create_pool().await?;
        let rows: Option<u32> = pool.get_conn()?.query_first(query_string)?;
        self.number_of_rows = rows.unwrap();
        self.number_of_pages = ((rows.unwrap() as f32 / self.limit as f32) + 1.0).floor() as u32;
        println!("pages={}", self.number_of_pages);
        Ok(rows)
    }

    pub fn set_number_of_pages(&mut self) -> u32 {
        let (tx, rx) = bounded(1);
        smol::block_on(async {
            let res = self._set_number_of_pages().await;
            let _ = tx.send(res.unwrap());
        });
        let n: Option<u32> = rx.recv().unwrap();
        n.unwrap()
    }
}
