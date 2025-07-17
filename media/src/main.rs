#[derive(Debug)]
enum Media {
  Book { title: String, author: String },
  Movie { title: String, director: String },
  Audiobook { title: String },
  Podcast(u32),
  Placeholder,
}

impl Media {
  fn description(&self) -> String {
    match self {
      Media::Book { title, author } => {
        format!("Book: {} {}", title, author)
      }
      Media::Movie { title, director } => {
        format!("Movie: {} {}", title, director)
      }
      Media::Audiobook { title } => {
        format!("Audiobook: {}", title)
      }
      Media::Podcast(id) => {
        format!("Podcast: {}", id)
      }
      Media::Placeholder => {
        format!("Placeholder")
      }
    }
  }
}

#[derive(Debug)]
struct Catalog {
  items: Vec<Media>,
}

impl Catalog {
  fn new() -> Self {
    Catalog { items: vec![] }
  }

  fn add(&mut self, media: Media) {
    self.items.push(media);
  }

  fn get_by_index(&self, index: usize) -> MightHaveAValue {
    if self.items.len() > index {
      // Good! We have somethign to return
      MightHaveAValue::ThereIsAValue(&self.items[index])
    } else {
      // Bad! We don't have anything to return!!!
      MightHaveAValue::NoValueAvailable
    }
  }
}

enum MightHaveAValue<'a> {
  ThereIsAValue(&'a Media),
  NoValueAvailable,
}

fn print_media(media: Media) {
  println!("{:#?}", media);
}

fn main() {
  let audiobook = Media::Audiobook {
    title: String::from("An Audiobook"),
  };
  let good_movie = Media::Movie {
    title: String::from("Good Movie"),
    director: String::from("Good Director"),
  };
  let bad_book = Media::Book {
    title: String::from("Bad Book"),
    author: String::from("Bad Author"),
  };
  let podcast = Media::Podcast(10);
  let placeholder = Media::Placeholder;
  let mut catalog = Catalog::new();

  catalog.add(audiobook);
  catalog.add(good_movie);
  catalog.add(bad_book);
  catalog.add(podcast);
  catalog.add(placeholder);

  if let MightHaveAValue::ThereIsAValue(value) = catalog.get_by_index(1) {
    println!("Item in pattern match: {:#?}", value)
  } else {
    println!("No value!!!!!!!!!!");
  }
}
