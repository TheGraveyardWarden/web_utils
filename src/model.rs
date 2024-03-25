use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::Unpin;
use async_trait::async_trait;
use mongodb::{
    bson::{
        oid::ObjectId,
        doc,
        Bson,
        document::Document
    },
    Collection,
    options::{
        FindOptions,
        FindOneOptions,
        AggregateOptions
    }
};
use crate::err::{Result, Error};

#[async_trait]
pub trait Model: DeserializeOwned + Unpin + Send + Sync + Serialize {
    async fn get_by_id(id: &ObjectId, coll: &Collection<Self>) -> Result<Self> {
        match coll.find_one(doc!{"_id": id}, None).await
        {
            Ok(Some(s)) => Ok(s),
            Ok(None) => Err(Error::NotFound),
            Err(err) => Err(Error::MongoError(err))
        }
    }

    async fn delete_by_id(id: &ObjectId, coll: &Collection<Self>) -> Result<()> {
        coll.delete_one(doc!{"_id": id}, None).await?;
        Ok(())
    }

    async fn insert(&self, coll: &Collection<Self>) -> Result<Bson> {
        match coll.insert_one(self, None).await {
            Ok(i) => Ok(i.inserted_id),
            Err(err) => Err(Error::MongoError(err))
        }
    }

    async fn get<T>(
        filter: impl Into<Option<Document>> + Send, 
        options: impl Into<Option<FindOptions>> + Send, 
        coll: &Collection<Self>
    ) -> Result<Vec<T>>
    where
        T: DeserializeOwned + Send
    {
        let mut cursor = coll.find(filter, options).await?.with_type::<T>();
        let mut tmp: Vec<T> = Vec::new();

        while cursor.advance().await? {
            tmp.push(cursor.deserialize_current()?)
        }

        Ok(tmp)
    }

    async fn get_one(
        filter: impl Into<Option<Document>> + Send, 
        options: impl Into<Option<FindOneOptions>> + Send, 
        coll: &Collection<Self>
    ) -> Result<Self>
    {
        match coll.find_one(filter, options).await {
            Ok(Some(d)) => Ok(d),
            Ok(None) => Err(Error::NotFound),
            Err(err) => Err(Error::MongoError(err))
        }
    }

    async fn aggregate<T>(
        pipeline: impl IntoIterator<Item = Document> + Send,
        options: impl Into<Option<AggregateOptions>> + Send,
        coll: &Collection<Self>
    ) -> Result<Vec<T>>
    where
        T: DeserializeOwned + Send
    {
        let mut cursor = coll.aggregate(pipeline, options).await?.with_type::<T>();

        let mut objs: Vec<T> = Vec::new();
        while cursor.advance().await? {
            objs.push(cursor.deserialize_current()?);
        }

        if objs.len() < 1 {
            return Err(Error::NotFound);
        }

        Ok(objs)
    }
}
