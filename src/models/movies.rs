use chrono::DateTime;
use chrono::Utc;
use std::cmp;
use std::collections::HashMap;

type MovieProperty = HashMap<String, String>;

pub struct Movie {
    name: String,
    url: String,
    properties: MovieProperty,
    premiere_date: DateTime<Utc>,
}

pub struct MovieList {
    movies: Vec<Movie>,
    page_size: usize,
}

impl MovieList {
    pub fn new(args: Vec<Movie>) -> MovieList {
        MovieList {
            movies: args,
            page_size: 5,
        }
    }

    pub fn getPage(&self, page_number: usize) -> &[Movie] {
        let offset = (page_number - 1) * self.page_size;
        let tail = cmp::min(self.movies.len() - 1, offset + (self.page_size - 1));

        &self.movies[offset..tail]
    }

    pub fn totalPages(&self) -> u8 {
        ((self.movies.len() as f64) / (self.page_size as f64)).ceil() as u8
    }
}

impl Movie {
    pub fn new(
        name: String,
        url: String,
        properties: MovieProperty,
        premiere_date: DateTime<Utc>,
    ) -> Movie {
        Movie {
            name,
            url,
            properties,
            premiere_date,
        }
    }
}
