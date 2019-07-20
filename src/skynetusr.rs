use discord::model::*;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct SkyNetUsr {
    pub usr: User
}

impl Eq for SkyNetUsr {}
impl Hash for SkyNetUsr {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.usr.id.hash(state);
        self.usr.discriminator.hash(state);
    }
}

impl PartialEq for SkyNetUsr {
    fn eq(&self, other: &Self) -> bool {
        self.usr.id == other.usr.id &&
            self.usr.discriminator == other.usr.discriminator
    }
}
