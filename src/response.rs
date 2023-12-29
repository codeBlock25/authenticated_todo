use sea_orm::prelude::DateTime;
use serde::Serialize;

#[derive(Serialize, Copy, Default, Clone)]
pub struct OkResponse<TData> {
    pub(crate) query_time: DateTime,
    pub(crate) data: TData,
}