use crate::features::materials::domain::errors::types::MaterialError;


// // Конверсии из системных ошибок
// impl From<sqlx::Error> for MaterialError {
//     fn from(err: sqlx::Error) -> Self {
//         match err {
//             sqlx::Error::RowNotFound => MaterialError::DatabaseError {
//                 message: "Row not found".to_string(),
//             },
//             sqlx::Error::Database(db_err) => {
//                 // Проверяем на constraint violations для дублированных записей
//                 if db_err.code().is_some_and(|code| code == "23505") {
//                     MaterialError::DatabaseError {
//                         message: "Duplicate entry violation".to_string(),
//                     }
//                 } else {
//                     MaterialError::DatabaseError {
//                         message: db_err.to_string(),
//                     }
//                 }
//             }
//             _ => MaterialError::DatabaseError {
//                 message: err.to_string(),
//             },
//         }
//     }
// }
