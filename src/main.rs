use chrono::{DateTime, Datelike, FixedOffset, Local, Utc};
use typst::syntax::{FileId, Source};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};
use typst::{
    foundations::{Bytes, Datetime},
    layout::PagedDocument,
};
use typst_pdf::PdfOptions;

fn main() {
    let source = std::fs::read_to_string("./example/hi.typ").unwrap();
    let warned_result = typst::compile::<PagedDocument>(&Earth::new(source));

    if !warned_result.warnings.is_empty() {
        println!("=== WARNINGS ===");
        for (i, d) in warned_result.warnings.iter().enumerate() {
            println!("{i}: {:?}: {}", d.severity, d.message);
        }
    }

    let doc = match warned_result.output {
        Ok(doc) => doc,
        Err(_) => {
            println!("Got error, halting.");
            return;
        }
    };

    let pdf = typst_pdf::pdf(&doc, &PdfOptions::default()).unwrap();
    std::fs::write("./example/out.pdf", pdf).unwrap();
    println!("done");
}

/// A simple Typst world that loads a single font
pub struct Earth {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    main: FileId,
    source: Source,
    now: DateTime<Utc>,
}

impl Earth {
    pub fn new(source_text: String) -> Self {
        // Load the font from the specified path
        let font_path = "target/newcm-7.0.4/otf/NewCM08-Regular.otf";
        let font_data = std::fs::read(font_path).expect("Failed to read font file");

        let font_bytes = Bytes::new(font_data);
        let fonts = Font::iter(font_bytes).collect::<Vec<_>>();

        let book = LazyHash::new(FontBook::from_fonts(&fonts));

        // Create a source with the input text
        let source = Source::detached(source_text);
        let main = source.id();

        Self {
            library: LazyHash::new(Library::default()),
            book,
            fonts,
            main,
            source,
            now: Utc::now(),
        }
    }
}

impl World for Earth {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    fn main(&self) -> FileId {
        self.main
    }

    fn source(&self, id: FileId) -> typst::diag::FileResult<Source> {
        if id == self.main {
            Ok(self.source.clone())
        } else {
            Err(typst::diag::FileError::NotFound(
                id.vpath().as_rootless_path().into(),
            ))
        }
    }

    fn file(&self, id: FileId) -> typst::diag::FileResult<Bytes> {
        Err(typst::diag::FileError::NotFound(
            id.vpath().as_rootless_path().into(),
        ))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let with_offset = match offset {
            None => self.now.with_timezone(&Local).fixed_offset(),
            Some(hours) => {
                let seconds = i32::try_from(hours).ok()?.checked_mul(3600)?;
                self.now.with_timezone(&FixedOffset::east_opt(seconds)?)
            }
        };
        Datetime::from_ymd(
            with_offset.year(),
            with_offset.month().try_into().ok()?,
            with_offset.day().try_into().ok()?,
        )
    }
}
