use crate::features::materials::domain::{
    entities::Thickness, errors::MaterialError, value_objects::ThicknessUid,
};

#[async_trait::async_trait]
pub trait ThicknessBehavior {
    /// Получить толщину по ID
    async fn get_thickness(&self, id: &ThicknessUid) -> Result<Thickness, MaterialError>;

    /// Получить все толщины
    async fn get_all_thicknesses(&self) -> Result<Vec<Thickness>, MaterialError>;

    /// Создать новую толщину
    async fn create_thickness(&self, thickness: Thickness) -> Result<Thickness, MaterialError>;

    /// Проверить существование толщины
    async fn exists(&self, id: &ThicknessUid) -> Result<bool, MaterialError>;

    /// Найти толщину по значению
    async fn find_by_value(&self, value: f64) -> Result<Option<Thickness>, MaterialError>;
}