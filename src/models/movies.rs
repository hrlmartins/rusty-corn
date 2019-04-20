use std::cmp;

#[derive(Clone)]
pub struct Movie {
    pub name: String,
    pub url: String,
    pub image_url: String,
}

#[derive(Clone)]
pub struct MovieList {
    movies: Vec<Movie>,
    page_size: u8,
}

impl MovieList {
    pub fn new(args: Vec<Movie>) -> MovieList {
        MovieList {
            movies: args,
            page_size: 5,
        }
    }

    pub fn get_page(&self, page_number: u8) -> &[Movie] {
        let offset = (page_number - 1) * self.page_size;
        let tail = cmp::min(
            self.movies.len() - 1,
            (offset + (self.page_size - 1)) as usize,
        );

        &self.movies[offset as usize..=tail]
    }

    pub fn total_pages(&self) -> u8 {
        ((self.movies.len() as f64) / (self.page_size as f64)).ceil() as u8
    }
}

impl Movie {
    pub fn new(name: String, url: String, image_url: String) -> Movie {
        Movie {
            name,
            url,
            image_url,
        }
    }
}
