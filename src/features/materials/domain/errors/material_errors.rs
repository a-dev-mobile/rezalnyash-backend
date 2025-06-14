// //Что делает: Определяет все возможные ошибки в домене материалов. Создается первым, так как все остальные компоненты будут возвращать эти ошибки.

// use std::error::Error;
// use std::fmt;

// #[derive(Debug, Clone, PartialEq)]
// pub struct MaterialError {
//     pub kind: String,
//     pub message: String,
// }

// impl MaterialError {
//     pub fn new(kind: &str, message: &str) -> Self {
//         Self {
//             kind: kind.to_string(),
//             message: message.to_string(),
//         }
//     }

//     pub fn materials_not_found(id: &str) -> Self {
//         Self::new("NotFound", &format!("Materials not found with ID: {}", id))
//     }
//     pub fn invalid_dimensions(w: f64, h: f64) -> Self {
//         Self::new(
//             "InvalidDimensions",
//             &format!("Invalid dimensions: width {} and height {}", w, h),
//         )
//     }

//     pub fn repository_error(message: &str) -> Self {
//         Self::new("RepositoryError", message)
//     }

//     pub fn validation_error(message: &str) -> Self {
//         Self::new("ValidationError", message)
//     }
// }

// impl fmt::Display for MaterialError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}: {}", self.kind, self.message)
//     }
// }
// impl Error for MaterialError {}
