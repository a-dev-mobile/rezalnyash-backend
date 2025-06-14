use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WidthUid {
    value: Uuid,
}

impl WidthUid {
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

impl std::fmt::Display for WidthUid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<Uuid> for WidthUid {
    fn from(uuid: Uuid) -> Self {
        Self::new(uuid)
    }
}

impl From<WidthUid> for Uuid {
    fn from(id: WidthUid) -> Self {
        id.value
    }
}