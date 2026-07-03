// First version created by Claude Sonnet 5

#[cfg(test)]
mod tests {
  use core::coordinate::{Breadcrumb, Coordinate};

  fn check(desc: &str, crumbs: &Breadcrumb, expected: &[Coordinate]) {
    let actual = crumbs.to_sequence();
    assert_eq!(actual, expected, "{desc}");
  }

  #[test]
  fn simple_source_1() {
    check(
      "Simple source 1",
      &Breadcrumb::Source {
        coord: Coordinate { x: 0, y: 0 },
      },
      &[Coordinate { x: 0, y: 0 }],
    );
  }

  #[test]
  fn simple_source_2() {
    check(
      "Simple source 2",
      &Breadcrumb::Source {
        coord: Coordinate { x: 1, y: 1 },
      },
      &[Coordinate { x: 1, y: 1 }],
    );
  }

  #[test]
  fn simple_source_3() {
    check(
      "Simple source 3",
      &Breadcrumb::Source {
        coord: Coordinate { x: 3, y: 8 },
      },
      &[Coordinate { x: 3, y: 8 }],
    );
  }

  #[test]
  fn two_item_crumb() {
    check(
      "Two-item crumb",
      &Breadcrumb::Crumb {
        to: Coordinate { x: 0, y: 0 },
        from: Box::new(Breadcrumb::Source {
          coord: Coordinate { x: 3, y: 8 },
        }),
      },
      &[Coordinate { x: 3, y: 8 }, Coordinate { x: 0, y: 0 }],
    );
  }

  #[test]
  fn three_item_crumb() {
    check(
      "Three-item crumb",
      &Breadcrumb::Crumb {
        to: Coordinate { x: 1, y: 7 },
        from: Box::new(Breadcrumb::Crumb {
          to: Coordinate { x: 0, y: 0 },
          from: Box::new(Breadcrumb::Source {
            coord: Coordinate { x: 3, y: 8 },
          }),
        }),
      },
      &[
        Coordinate { x: 3, y: 8 },
        Coordinate { x: 0, y: 0 },
        Coordinate { x: 1, y: 7 },
      ],
    );
  }
}
