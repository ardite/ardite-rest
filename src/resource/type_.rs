use ardite::schema;

use resource::Resource;

pub struct Type<'a> {
  type_: &'a schema::Type
}

impl<'a> Type<'a> {
  pub fn new(type_: &'a schema::Type) -> Self {
    Type {
      type_: type_
    }
  }
}

impl<'a> Resource for Type<'a> {

}
