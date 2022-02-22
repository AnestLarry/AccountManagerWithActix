extern crate chrono;
extern crate rusqlite;

use chrono::Utc;
use rusqlite::{Connection, Error, params, Statement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Data {
    pub address: String,
    pub account: String,
    pub password: String,
    pub email: String,
    pub date: String,
    pub text: String,
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Data [Address: {0}, Account: {1}, Password: {2}, Date: {3}, Text: {4}]",
               self.address, self.account, self.password, self.date, self.text)
    }
}


pub struct SQLOperator {
    conn: Connection,
}

impl SQLOperator {
    pub fn new() -> Self {
        let conn = Connection::open("Database.db").unwrap();
        return SQLOperator {
            conn,
        };
    }
    pub fn add_item(&self, d: &Data) -> Result<usize, (Error, [String; 5])> {
        let r = self.conn.execute(
            "insert into Data(Address, Account, Password, Email, Date, Text) \
                values(?1,?2,?3,?4,?5,?6);",
            params![d.address, d.account, d.password,d.email,
                Utc::now().format("%Y-%m-%d--%H-%M-%S--%A").to_string(), d.text],
        );
        match r {
            Ok(i) => Result::Ok(i),
            Err(e) => Result::Err((e, [d.address.clone(), d.account.clone(), d.password.clone(), d.email.clone(), d.text.clone()]))
        }
    }
    pub fn search_item(&self, key: String, key_word: String) -> Result<Vec<Data>, Error> {
        let mut res: Vec<Data> = Vec::new();
        let mut stmt: Statement;
        if key == "Text" {
            stmt = self.conn.prepare("select Address,Account,Password,Email,Date,Text \
                from Data \
                where Text LIKE ?1 ORDER BY Date DESC;")?;
        } else {
            stmt = self.conn.prepare(&format!("select Address,Account,Password,Email,Date,Text \
                from Data \
                where {0} = ?1 ORDER BY Date DESC;", key))?;
        }
        let data_iter = stmt.query_map(params![key_word], |row| {
            Ok(Data {
                address: row.get(0)?,
                account: row.get(1)?,
                password: row.get(2)?,
                email: row.get(3).unwrap_or("".into()),
                date: row.get(4)?,
                text: row.get(5).unwrap_or("".into()),
            })
        }).unwrap();
        for data in data_iter {
            res.push(data.unwrap());
        }
        Result::Ok(res)
    }
    pub fn remove_item(&self, date: String) -> Result<(usize, Vec<Data>), Error> {
        let datas = self.search_item("Date".into(), date.to_string()).unwrap();
        let r = self.conn.execute("delete from Data where Date = ?1;", params![date]);
        match r {
            Ok(i) => Result::Ok((i, datas)),
            Err(e) => Result::Err(e)
        }
    }
    pub fn update_item(&self, text: String, date: String) -> Result<(bool, usize), Error> {
        let stmt = self.conn.execute("update Data set Text=?1 where Date=?2;",
                                     params![text,date]);
        match stmt {
            Ok(r) => Result::Ok((true, r)),
            Err(e) => Result::Err(e)
        }
    }
    #[allow(non_snake_case)]
    pub fn Data_of(ad: String, ac: String, p: String, e: String, d: String, t: String) -> Data {
        Data {
            address: ad,
            account: ac,
            password: p,
            email: e,
            date: d,
            text: t,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn SQLOperator_test() {
        SQLOperator__add_item();
        SQLOperator__search_item();
        SQLOperator__update_item();
        SQLOperator__remove_item();
    }

    fn SQLOperator__add_item() {
        let sql: SQLOperator = SQLOperator::new();
        let d = Data {
            address: "test.com".into(),
            account: "test".into(),
            password: "pd".into(),
            email: "".into(),
            date: "".into(),
            text: "forTest".into(),
        };
        assert_eq!(sql.add_item(&d)?, 1);
    }

    fn SQLOperator__search_item() {
        let sql: SQLOperator = SQLOperator::new();
        let address = sql.search_item("Address".into(), "test.com".into())?
            .pop()?.address;
        let account = sql.search_item("Account".into(), "test".into())?
            .pop()?.account;
        let password = sql.search_item("Password".into(), "pd".into())?
            .pop()?.password;
        let text = sql.search_item("Text".into(), "forTest".into())?
            .pop()?.text;
        assert_eq!(address, "test.com");
        assert_eq!(account, "test");
        assert_eq!(password, "pd");
        assert_eq!(text, "forTest");
    }

    fn SQLOperator__update_item() {
        let sql: SQLOperator = SQLOperator::new();
        let data = sql.search_item("Address".into(), "test.com".into())?
            .pop()?;
        let r = sql.update_item(data.text, data.date)?;
        assert_eq!(r, (true, 1));
    }

    fn SQLOperator__remove_item() {
        let sql: SQLOperator = SQLOperator::new();
        let data = sql.search_item("Address".into(), "test.com".into())?
            .pop()?;
        let r = sql.remove_item(data.date)?;
        assert_eq!(r.0, 1);
    }
}
