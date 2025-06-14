use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MaterialTypeUid {
    value: Uuid,
}

impl MaterialTypeUid {
    /// Создает новый ID с заданным UUID
    pub fn new(value: Uuid) -> Self {
        Self { value }
    }

    /// Создает новый случайный UUID
    pub fn generate() -> Self {
        Self {
            value: Uuid::new_v4(),
        }
    }

    pub fn value(&self) -> Uuid {
        self.value
    }
}

impl std::fmt::Display for MaterialTypeUid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<Uuid> for MaterialTypeUid {
    fn from(uuid: Uuid) -> Self {
        Self::new(uuid)
    }
}

impl From<MaterialTypeUid> for Uuid {
    fn from(id: MaterialTypeUid) -> Self {
        id.value
    }
}