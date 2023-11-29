use std::{ops::Range, path::Path};

#[derive(Clone, Copy)]
pub struct CoolDoc(&'static str);

impl CoolDoc {
    /// Retrieves a slice of the Cool document's text.
    ///
    /// # Panics
    ///
    /// Panics if the provided range extends beyond the boundaries of the document's text.
    ///
    /// # Arguments
    ///
    /// * `range`: The range of indices within the document's text to extract a slice from.
    ///
    /// # Returns
    ///
    /// A static string slice representing the specified portion of the document's text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let cool_doc = CoolDoc::new("This is a cool document.");
    /// let slice = cool_doc.slice(0..5);
    /// assert_eq!(slice, "This");
    /// ```
    pub fn slice(self, range: Range<usize>) -> &'static str {
        &self.text()[range]
    }
    /// Retrieves the raw text of the Cool document.
    ///
    /// # Returns
    ///
    /// A reference to the static string slice representing the document's text.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let cool_doc = CoolDoc::new("This is a cool document.");
    /// let text = cool_doc.text();
    /// println!("{}", text);
    /// ```
    pub fn text(self) -> &'static str {
        self.0
    }

    /// Creates a `CoolDoc` from a file path.
    ///
    /// # Arguments
    ///
    /// * `path`: The path to the file containing the Cool document.
    ///
    /// # Returns
    ///
    /// A `Result` value containing either a `CoolDoc` instance if the file is successfully read,
    /// or an `std::io::Error` if there is an error reading the file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let cool_doc = CoolDoc::from_path("myfile.cool").unwrap();
    /// println!("{}", cool_doc.source());
    /// ```
    pub fn from_path(path: impl AsRef<Path>) -> std::io::Result<CoolDoc> {
        use std::fs::read_to_string;
        Ok(CoolDoc(read_to_string(path)?.leak()))
    }

    /// Creates a new `CoolDoc` instance from the provided static string slice.
    ///
    /// # Arguments
    ///
    /// * `text`: The static string slice representing the Cool document's text.
    ///
    /// # Returns
    ///
    /// A new `CoolDoc` instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let cool_doc = CoolDoc::new("This is a cool document.");
    /// println!("{}", cool_doc.text());
    /// ```
    pub fn new(text: &'static str) -> Self {
        Self(text)
    }

    /// Converts a `String` to a `CoolDoc` instance.
    ///
    /// # Arguments
    ///
    /// * `text`: The `String` containing the Cool document's text.
    ///
    /// # Returns
    ///
    /// A `CoolDoc` instance representing the provided `String`.
    ///
    /// # Safety
    ///
    /// The `String` is leaked and converted to a static string slice. This means that
    /// the original `String` will no longer be accessible, and it will be up to the
    /// caller to ensure that the static string slice remains valid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let text = String::from("This is a cool document.");
    /// let cool_doc = CoolDoc::from_string(text);
    /// println!("{}", cool_doc.text());
    /// ```
    pub fn from_string(text: String) -> Self {
        Self(text.leak())
    }
}

#[derive(Clone, Copy)]
struct Location {
    pub line: u32,
    pub column: u32,
    pub document: CoolDoc,
}

#[derive(Clone, Copy)]
struct Span {
    beg: usize,
    end: usize,
    pub document: CoolDoc,
}
