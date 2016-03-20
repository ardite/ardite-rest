//! Handles the routing for the REST server. Not the exuting or handling, just
//! the routing.

/// A flattened list of all possible routes.
pub enum Route {
  /// The top level route. Generally it returns some meta information.
  Root,
  /// A route representing the lack of any other route.
  None
}

impl Route {
  /// Resolve a route from the root.
  pub fn resolve_from_root(path: Vec<String>) -> Self {
    Route::Root.resolve(path)
  }

  /// Resolve a route from a path relative to another route.
  pub fn resolve(self, mut path: Vec<String>) -> Self {
    use router::Route::*;

    if path.len() == 0 {
      return self;
    }

    match (self, path.pop().unwrap().as_str()) {
      (Root, "") => Root.resolve(path),
      _ => None
    }
  }
}
