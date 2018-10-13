#[derive(Debug)]
pub struct EER {
    pub decls: Vec<Decl>,
}

#[derive(Debug)]
pub enum Decl {
    Entity(Entity),
    Relation(Relation),
}

#[derive(Debug)]
pub struct Entity {
    pub name: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub subattributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct Relation {

}
