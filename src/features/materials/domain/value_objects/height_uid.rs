use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeightUid {
    value: Uuid,
}

impl HeightUid {
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

impl std::fmt::Display for HeightUid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<Uuid> for HeightUid {
    fn from(uuid: Uuid) -> Self {
        Self::new(uuid)
    }
}

impl From<HeightUid> for Uuid {
    fn from(id: HeightUid) -> Self {
        id.value
    }
}