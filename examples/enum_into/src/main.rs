
#[derive(Debug, Clone)]
struct MyObject {
  is : Option<isize>,
  st : Option<String>,
}

#[derive(Debug, Clone)]
enum CompoundIndex {
  SearchIsize(isize),
  SearchString(String),
}
use CompoundIndex::*;

impl Into<MyObject> for isize {
  fn into(self) -> MyObject {
    MyObject {
      is : Some(self),
      st : None,
    }
  }
}
impl Into<MyObject> for String {
  fn into(self) -> MyObject {
    MyObject {
      is : None,
      st : Some(self),
    }
  }
}

fn main() {
  let m0 : MyObject = 42.into();
  let m1 : MyObject = "Coredump.ch".to_owned().into();

  println!("Hello, {:?} and {:?}!", m0, m1);

  let v = vec![m0, m1];

  let number = SearchIsize(42);
  println!("\n Find with number: {:?} => {:?}", number, find(&v, &number));

  let string = SearchString("".into());
  println!("\n Find with number: {:?} => {:?}", string, find(&v, &string));
  let string = SearchString("Coredump.ch".into());
  println!("\n Find with number: {:?} => {:?}", string, find(&v, &string));
}

fn find(haystack : &Vec<MyObject>, needle : &CompoundIndex) -> Option<MyObject> {
  for ref hay in haystack {
    match needle {
      &SearchIsize(ref needle) => {
        if let Some(ref is) = hay.is {
          if is == needle {
            return Some( (*hay).clone() );
          }
        }
      },
      &SearchString(ref needle) => {
        if let Some(ref st) = hay.st {
          if st == needle {
            return Some( (*hay).clone() );
          }
        }
      },
    }
  }
  None
}
