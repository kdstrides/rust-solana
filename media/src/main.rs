#[derive(Debug)]
enum Media {
  Book { title: String, author: String },
  Movie { title: String, director: String },
  AudioBook { title: String },
}

impl Media {
  fn description(&self) -> String {
    match self {
      Media::AudioBook { title } => format!("Audio Book: {}", title),
      Media::Movie { title, director } => format!("Movie: {} by {}", title, director),
      Media::Book { title, author } => format!("Book: {} by {}", title, author),
    }
  }
}

fn print_media(media: Media) {
  println!("Media: {:#?}", media);
}

fn main() {
  let audio_book = Media::AudioBook {
    title: String::from("The Great Gatsby"),
  };

  let good_movie = Media::Movie {
    title: String::from("The Good Movie"),
    director: String::from("The Good Director"),
  };

  let bad_book = Media::Book {
    title: String::from("The Bad Book"),
    author: String::from("The Bad Author"),
  };

  println!("{}", audio_book.description());
  println!("{}", good_movie.description());
  println!("{}", bad_book.description());

  print_media(audio_book);
  print_media(good_movie);
  print_media(bad_book);
}
