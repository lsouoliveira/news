pub struct Article {
    pub title: String,
    pub author: String,
    pub comments_count: u32,
    pub points: u32,
    pub created_at: String,
    pub url: String,
}

impl Article {
    pub fn new(title: String, author: String, comments_count: u32, points: u32, created_at: String, url: String) -> Article {
        Article {
            title,
            author,
            comments_count,
            points,
            created_at,
            url
        }
    }
}

pub struct Paginated <T> {
    pub page: u32,
    pub items_count: u32,
    pub data: Vec<T>,
}

impl <T> Paginated <T> {
    pub fn new(page: u32, items_count: u32, data: Vec<T>) -> Paginated <T> {
        Paginated {
            page,
            items_count,
            data
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_article() {
        use super::*;
        let article = Article::new(
            "The Rust Programming Language".to_string(),
            "Steve Klabnik".to_string(),
            100,
            200,
            "2020-01-01".to_string(),
            "https://www.rust-lang.org/".to_string(),
        );
        assert_eq!(article.title, "The Rust Programming Language");
        assert_eq!(article.author, "Steve Klabnik");
        assert_eq!(article.comments_count, 100);
        assert_eq!(article.points, 200);
        assert_eq!(article.created_at, "2020-01-01");
        assert_eq!(article.url, "https://www.rust-lang.org/");
    }

    #[test]
    fn test_paginated() {
        use super::*;
        let articles = vec![
            Article::new(
                "The Rust Programming Language".to_string(),
                "Steve Klabnik".to_string(),
                100,
                200,
                "2020-01-01".to_string(),
                "https://www.rust-lang.org/".to_string(),
            ),
            Article::new(
                "The Rust Programming Language".to_string(),
                "Steve Klabnik".to_string(),
                100,
                200,
                "2020-01-01".to_string(),
                "https://www.rust-lang.org/".to_string(),
            ),
        ];
        let paginated = Paginated::new(1, 2, articles);
        assert_eq!(paginated.page, 1);
        assert_eq!(paginated.items_count, 2);
        assert_eq!(paginated.data.len(), 2);
    }
}
