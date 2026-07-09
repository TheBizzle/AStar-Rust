use pf_core::interpreter::PathingMapString;

#[must_use]
pub fn all_maps() -> [PathingMapTest; 39] {
  [
    full_map_1(),
    full_map_2(),
    full_map_3(),
    full_map_4(),
    full_map_5(),
    full_map_6(),
    full_map_7(),
    full_map_8(),
    full_map_9(),
    full_map_10(),
    full_map_11(),
    full_map_12(),
    full_map_13(),
    full_map_14(),
    full_map_15(),
    full_map_16(),
    full_map_17(),
    full_map_18(),
    full_map_19(),
    full_map_20(),
    full_map_21(),
    full_map_22(),
    full_map_23(),
    full_map_24(),
    full_map_25(),
    full_map_26(),
    full_map_27(),
    full_map_28(),
    full_map_29(),
    full_map_30(),
    full_map_31(),
    full_map_32(),
    full_map_33(),
    full_map_34(),
    full_map_35(),
    full_map_36(),
    full_map_37(),
    full_map_38(),
    full_map_39(),
  ]
}

#[derive(Debug)]
pub struct PathingMapTest {
  pub dist: Option<u32>,
  pub mapstring: PathingMapString,
}

fn pms(delim: &str, contents: &str) -> PathingMapString {
  PathingMapString { contents: contents[0..].to_string(), delim: format!("{delim}\n") }
}

fn full_map_1() -> PathingMapTest {
  PathingMapTest { dist: Some(14), mapstring: pms("akjshdkjashldjaksdhljakds", "*             G") }
}

fn full_map_2() -> PathingMapTest {
  PathingMapTest {
    dist: Some(2),
    mapstring: pms(
      "asdf",
      &"
 *asdf
G asdf"[1..],
    ),
  }
}

fn full_map_3() -> PathingMapTest {
  PathingMapTest {
    dist: None,
    mapstring: pms(
      "|",
      &"
 %  *|
OG% %|
%%   |"[1..],
    ),
  }
}

fn full_map_4() -> PathingMapTest {
  PathingMapTest {
    dist: Some(6),
    mapstring: pms(
      "|",
      &"
 %  *|
OG% %|
%    |"[1..],
    ),
  }
}

fn full_map_5() -> PathingMapTest {
  PathingMapTest {
    dist: Some(39),
    mapstring: pms(
      "|",
      &"
               |
           *   |
               |
               |
%%%%%%%%%%     |
        GD     |
D DDDDDDDD     |
  D D    D     |
 DD      D     |
    D DDDD     |
DDDDD    D     |
    DDDD D     |
               |
               |
               "[1..],
    ),
  }
}

fn full_map_6() -> PathingMapTest {
  PathingMapTest {
    dist: Some(61),
    mapstring: pms(
      "|",
      &"
               |
           *   |
         O%%%%%|
               |
%%%%%%%%%%%%%% |
        GD     |
D DDDDDDDD %%%%|
  D D    D     |
 DD      D%%%% |
    D DDDD     |
DDDDD    D     |
    DDDD D     |
       % %     |
       % %     |
               "[1..],
    ),
  }
}

fn full_map_7() -> PathingMapTest {
  PathingMapTest { dist: None, mapstring: pms("|", "*DG") }
}

fn full_map_8() -> PathingMapTest {
  PathingMapTest { dist: Some(14), mapstring: pms("|", "G             *") }
}

fn full_map_9() -> PathingMapTest {
  PathingMapTest {
    dist: Some(14),
    mapstring: pms(
      "|",
      &"
*|
 |
 |
 |
 |
 |
 |
 |
 |
 |
 |
 |
 |
 |
G"[1..],
    ),
  }
}

fn full_map_10() -> PathingMapTest {
  PathingMapTest {
    dist: Some(14),
    mapstring: pms(
      "|",
      &"
G|
 |
 |
 |
 |
 |
 |
 |
 |
 |
 |
 |
 |
 |
*"[1..],
    ),
  }
}

fn full_map_11() -> PathingMapTest {
  PathingMapTest {
    dist: Some(7),
    mapstring: pms(
      "|",
      &"
       *      G"[1..],
    ),
  }
}

fn full_map_12() -> PathingMapTest {
  PathingMapTest {
    dist: Some(8),
    mapstring: pms(
      "|",
      &"
 |
 |
 |
 |
 |
 |
*|
 |
 |
 |
 |
 |
 |
 |
G"[1..],
    ),
  }
}

fn full_map_13() -> PathingMapTest {
  PathingMapTest {
    dist: Some(14),
    mapstring: pms(
      "|",
      &"
*             G|
               |
               |
               |
               "[1..],
    ),
  }
}

fn full_map_14() -> PathingMapTest {
  PathingMapTest {
    dist: Some(14),
    mapstring: pms(
      "|",
      &"
G             *|
               |
               |
               |
               "[1..],
    ),
  }
}

fn full_map_15() -> PathingMapTest {
  PathingMapTest {
    dist: Some(14),
    mapstring: pms(
      "|",
      &"
               |
               |
               |
               |
*             G"[1..],
    ),
  }
}

fn full_map_16() -> PathingMapTest {
  PathingMapTest {
    dist: Some(14),
    mapstring: pms(
      "|",
      &"
               |
               |
               |
               |
G             *"[1..],
    ),
  }
}

fn full_map_17() -> PathingMapTest {
  PathingMapTest {
    dist: Some(14),
    mapstring: pms(
      "|",
      &"
               |
               |
*             G|
               |
               "[1..],
    ),
  }
}

fn full_map_18() -> PathingMapTest {
  PathingMapTest {
    dist: Some(4),
    mapstring: pms(
      "|",
      &"
*              |
               |
               |
               |
G              "[1..],
    ),
  }
}

fn full_map_19() -> PathingMapTest {
  PathingMapTest {
    dist: Some(4),
    mapstring: pms(
      "|",
      &"
G              |
               |
               |
               |
*              "[1..],
    ),
  }
}

fn full_map_20() -> PathingMapTest {
  PathingMapTest {
    dist: Some(4),
    mapstring: pms(
      "|",
      &"
              *|
               |
               |
               |
              G"[1..],
    ),
  }
}

fn full_map_21() -> PathingMapTest {
  PathingMapTest {
    dist: Some(4),
    mapstring: pms(
      "|",
      &"
              G|
               |
               |
               |
              *"[1..],
    ),
  }
}

fn full_map_22() -> PathingMapTest {
  PathingMapTest {
    dist: Some(4),
    mapstring: pms(
      "|",
      &"
       *       |
               |
               |
               |
       G       "[1..],
    ),
  }
}

fn full_map_23() -> PathingMapTest {
  PathingMapTest {
    dist: Some(4),
    mapstring: pms(
      "|",
      &"
       G       |
               |
               |
               |
       *       "[1..],
    ),
  }
}

fn full_map_24() -> PathingMapTest {
  PathingMapTest {
    dist: Some(18),
    mapstring: pms(
      "|",
      &"
              G|
               |
               |
               |
*              "[1..],
    ),
  }
}

fn full_map_25() -> PathingMapTest {
  PathingMapTest {
    dist: Some(18),
    mapstring: pms(
      "|",
      &"
G              |
               |
               |
               |
              *"[1..],
    ),
  }
}

fn full_map_26() -> PathingMapTest {
  PathingMapTest {
    dist: Some(9),
    mapstring: pms(
      "|",
      &"
G              |
               |
       *       |
               |
               "[1..],
    ),
  }
}

fn full_map_27() -> PathingMapTest {
  PathingMapTest {
    dist: Some(20),
    mapstring: pms(
      "|",
      &"
GD DD   D      |
   DD  D  D D  |
 D      D      |
    D  D     D |
 D  D      D  *"[1..],
    ),
  }
}

fn full_map_28() -> PathingMapTest {
  PathingMapTest {
    dist: Some(4),
    mapstring: pms(
      "|",
      &"
              G|
             D |
             D |
             D |
             D*"[1..],
    ),
  }
}

fn full_map_29() -> PathingMapTest {
  PathingMapTest {
    dist: Some(32),
    mapstring: pms(
      "|",
      &"
G              |
               |
               |
DDDDDDDDDDDDDD |
*              "[1..],
    ),
  }
}

fn full_map_30() -> PathingMapTest {
  PathingMapTest {
    dist: Some(15),
    mapstring: pms(
      "|",
      &"
      D        |
      D        |
      D*D      |
      DDD      |
G              "[1..],
    ),
  }
}

fn full_map_31() -> PathingMapTest {
  PathingMapTest {
    dist: Some(13),
    mapstring: pms(
      "|",
      &"
               |
      D D      |
      D*D      |
      DDD      |
G              "[1..],
    ),
  }
}

fn full_map_32() -> PathingMapTest {
  PathingMapTest {
    dist: Some(13),
    mapstring: pms(
      "|",
      &"
        D      |
      D D      |
      D*D      |
      DDD      |
G              "[1..],
    ),
  }
}

fn full_map_33() -> PathingMapTest {
  PathingMapTest {
    dist: Some(9),
    mapstring: pms(
      "|",
      &"
      D        |
      D        |
      D*D      |
      D D      |
G              "[1..],
    ),
  }
}

fn full_map_34() -> PathingMapTest {
  PathingMapTest {
    dist: Some(9),
    mapstring: pms(
      "|",
      &"
      D        |
      D        |
       *D      |
      DDD      |
G              "[1..],
    ),
  }
}

fn full_map_35() -> PathingMapTest {
  PathingMapTest {
    dist: None,
    mapstring: pms(
      "|",
      &"
               |
      DDD      |
      D*D      |
      DDD      |
G              "[1..],
    ),
  }
}

fn full_map_36() -> PathingMapTest {
  PathingMapTest {
    dist: None,
    mapstring: pms(
      "|",
      &"
                                              |
                                              |
                                              |
                 DDDDDDDDDDDDD                |
                 D    D  D   D                |
                 D  D        D                |
                 D         D D                |
                 D D         D                |
                 D    *      D                |
                 D          DD                |
                 D   D       D                |
                 DD  D   D   D                |
                 DDDDDDDDDDDDD                |
                                              |
       G                                      |
                                              |
                                              "[1..],
    ),
  }
}

fn full_map_37() -> PathingMapTest {
  PathingMapTest {
    dist: None,
    mapstring: pms(
      "|",
      &"
               |
      DDD      |
      DGD      |
      DDD      |
*              "[1..],
    ),
  }
}

fn full_map_38() -> PathingMapTest {
  PathingMapTest {
    dist: None,
    mapstring: pms(
      "|",
      &"
                                              |
                                              |
                                              |
                 DDDDDDDDDDDDD                |
                 D    D  D   D                |
                 D  D        D                |
                 D         D D                |
                 D D         D                |
                 D    G      D                |
                 D          DD                |
                 D   D       D                |
                 DD  D   D   D                |
                 DDDDDDDDDDDDD                |
                                              |
       *                                      |
                                              |
                                              "[1..],
    ),
  }
}

fn full_map_39() -> PathingMapTest {
  PathingMapTest {
    dist: None,
    mapstring: pms(
      "|",
      &"
                                              |
 *                                            |
                DDDDDDDDDDDDDDDDDDDDDDDDDDDDDD|
                DDDDDDDDDDDDDDDDDDDDDDDDDDDDDD|
                DD    D  D                    |
                DD  D    f   DDDDDDDDDDDDDDDD |
                DD       f D DD             D |
                DD D      fffDD            D  |
                DD    G      DD             D |
                DD   D      DDD DDDDDDDDDDDDD |
                DD   D   D   DD DD            |
 DDDDDDDDDDDDDDDDDDDDDDDDDDDDDD DDDDDDDDDDDDD |
 DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD D D   D   D |
                          D   D   D D D D D D |
DDDDDDDDDDDDDDDDDDDDDDDDD   D     D   D   D   |
DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD|
                        DDDDDDDDDDDDDDDDDDDDDD"[1..],
    ),
  }
}
