mod entities;

use entities::{note::ActiveModel, prelude::*, *};
use sea_orm::*;
use time::OffsetDateTime;

pub async fn insert(title: &String, text: &String, db: &DatabaseConnection) -> Result<InsertResult<ActiveModel>, DbErr> {

    let note = note::ActiveModel {
        title: ActiveValue::Set(title.to_owned()),
        text: ActiveValue::Set(text.to_owned()),
        created_at: ActiveValue::Set(OffsetDateTime::now_utc().to_string()),
        ..Default::default()
    };

    let res = Note::insert(note).exec(db).await?;

    Ok(res)
}

pub async fn set_done(id: i32, db: &DatabaseConnection) -> Result<note::Model, DbErr> {
    let note: Option<note::Model> = Note::find_by_id(id).one(db).await?;
    let mut note: note::ActiveModel = note.unwrap().into();
    note.done = Set(true);

    let note: note::Model = note.update(db).await?;

    Ok(note)
}
