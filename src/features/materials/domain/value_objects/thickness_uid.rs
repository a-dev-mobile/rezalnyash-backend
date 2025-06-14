use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThicknessUid {
    value: Uuid,
}

impl ThicknessUid {
    pub fn new(value: Uuid) -> Self {
        Self { value }
    }

    pub fn generate() -> Self {
        Self {
            value: Uuid::new_v4(),
        }
    }

    pub fn value(&self) -> Uuid {
        self.value
    }
}

impl std::fmt::Display for ThicknessUid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<Uuid> for ThicknessUid {
    fn from(uuid: Uuid) -> Self {
        Self::new(uuid)
    }
}

impl From<ThicknessUid> for Uuid {
    fn from(id: ThicknessUid) -> Self {
        id.value
    }
}