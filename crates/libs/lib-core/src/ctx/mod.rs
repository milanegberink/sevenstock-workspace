use uuid::Uuid;

pub struct Ctx {
    sub: Uuid,
}

impl Ctx {
    pub fn new(sub: Uuid) -> Self {
        Self { sub }
    }
}

impl Ctx {
    pub fn sub(&self) -> &Uuid {
        &self.sub
    }
}
