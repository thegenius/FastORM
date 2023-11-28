use sqlx::TransactionManager;
use sqlx::AnyPool;
use sqlx::Error;
use sqlx::Any;
use sqlx::Executor;
use sqlx::{Row, any::AnyRow};
use async_trait::async_trait;
use crate::{Entity, Primary, Selection, SelectedEntity, Mutation, Location, PageInfo, PagedList};
use crate::{merge_any_arguments};
use crate::v2::GenericDaoMapper;



pub struct Transaction <'a>{
    transaction: sqlx::Transaction<'a, sqlx::Any>
}

impl <'a> GenericDaoMapper for Transaction<'a>{}



impl <'a> Transaction<'a> {

    #[inline]
    pub async fn commit(self) -> Result<(), Error> {
        return Ok(self.transaction.commit().await?);
    }

    #[inline]
    pub async fn rollback(self) -> Result<(), Error> {
        return Ok(self.transaction.rollback().await?);
    }

    pub async fn select<'e, EX, P, S, SE>(
        &mut self,
        primary: P,
        selection: S,
    ) -> Result<Option<SE>, Error> 
    where EX: 'e + Executor<'e, Database = Any>, 
    P: Primary + Send, 
    S: Selection + Send, 
    SE: SelectedEntity + Send + Unpin {
        let result: Option<SE> = <Transaction as GenericDaoMapper>::select(&mut *self.transaction, primary, selection).await?;
        return Ok(result);
    }
}