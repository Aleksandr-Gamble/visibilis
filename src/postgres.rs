/*The postgres module of bi provides traits that make it easy to query
a Postgres database for a given struct */
//use async_trait::async_trait;
use std::{vec::Vec, marker::Sync};
use async_trait::async_trait;
use serde::Serialize;
use nexum::{core::GenericError, postgres::{self as rpg, Client as PGClient, MissingRowError}};
use tokio_postgres::{row::Row, types::{ToSql}};


/// The fulltext trait makes it easy to perform fulltext searches using Postgres
pub trait FullText {
    fn query_fulltext() -> &'static str;
    fn rowfunc_fulltext(row: &Row) -> Self;
}

pub async fn exec_fulltext<T: FullText>(client: &PGClient, phrase: &str) -> Result<Vec<T>, GenericError> {
    let query = T::query_fulltext();
    let ts_expr = rpg::ts_expression(phrase);
    let mut hits = Vec::new();
    let rows = client.query(query,&[&ts_expr]).await?;
    for row in rows {
        let hit = T::rowfunc_fulltext(&row);
        hits.push(hit);
    }
    Ok(hits)
}

/// The WhoWhatWhere returns a reference to one item of a given type
#[derive(Serialize)]
pub struct WhoWhatWhere<PK: Serialize+std::marker::Send > {
    pub data_type: &'static str,
    pub pk: PK,
    pub name: String
}

/// The autocomp trait maks it easy to return a vec of WhoWhatWhere referencing a given type
#[async_trait]
pub trait AutoComp<PK: Serialize+std::marker::Send >: std::marker::Send {
    fn query_autocomp() -> &'static str;
    fn rowfunc_autocomp(row: &Row) -> WhoWhatWhere<PK>;
    async fn exec_autocomp(client: &PGClient, phrase: &str) -> Result<Vec<WhoWhatWhere<PK>>, GenericError> {
        let query = Self::query_autocomp();
        let ts_expr = rpg::ts_expression(phrase);
        let mut hits = Vec::new();
        let rows = client.query(query,&[&ts_expr]).await?;
        for row in rows {
            let hit = Self::rowfunc_autocomp(&row);
            hits.push(hit);
        }
        Ok(hits)
    }
}

pub async fn exec_autocomp<PK: Serialize+std::marker::Send , T: AutoComp<PK>>(client: &PGClient, phrase: &str) -> Result<Vec<WhoWhatWhere<PK>>, GenericError> {
    let query = T::query_autocomp();
    let ts_expr = rpg::ts_expression(phrase);
    let mut hits = Vec::new();
    let rows = client.query(query,&[&ts_expr]).await?;
    for row in rows {
        let hit = T::rowfunc_autocomp(&row);
        hits.push(hit);
    }
    Ok(hits)
}

/// the get by PK trait makes it easy to return an instance of a struct given its primary key
pub trait GetByPK {
    fn query_get_by_pk() -> &'static str;       // a query to return the struct
    fn rowfunc_get_by_pk(row: &Row) -> Self;    // returns the struct
}

pub async fn get_by_pk<T: GetByPK>(client: &PGClient, params: &[&(dyn ToSql+Sync)]) -> Result<T, GenericError> {
    let query = T::query_get_by_pk();
    let rows = client.query(query, params).await?;
    let row = rows.get(0).ok_or(MissingRowError{message:"could not get by PK".to_string()})?;
    let x = T::rowfunc_get_by_pk(row);
    Ok(x)
}

