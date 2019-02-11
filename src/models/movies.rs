pub struct Movie {
    name: String,
    url: String,
    properties: MovieProperty,
    premiere_date: DateTime<Utc>,
}

type MovieProperty = HashMap<String, String>;