use ardite::schema;

use resource::Resource;

pub struct Collection<'a> {
  collection: &'a schema::Collection
}

impl<'a> Collection<'a> {
  pub fn new(collection: &'a schema::Collection) -> Self {
    Collection {
      collection: collection
    }
  }
}

impl<'a> Resource for Collection<'a> {

}
