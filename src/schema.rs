table! {
    book (id) {
        id -> Uuid,
        series_id -> Uuid,
        order_in_series -> Int4,
    }
}

table! {
    book_author_map (id) {
        id -> Int4,
        book_id -> Uuid,
        person_id -> Uuid,
    }
}

table! {
    book_illustrator_map (id) {
        id -> Int4,
        book_id -> Uuid,
        person_id -> Uuid,
    }
}

table! {
    book_info (id) {
        id -> Uuid,
        book_id -> Uuid,
        lang -> Varchar,
        title -> Varchar,
        isbn -> Varchar,
        published_at -> Date,
    }
}

table! {
    book_info_translator_map (id) {
        id -> Int4,
        book_info_id -> Uuid,
        person_id -> Uuid,
    }
}

table! {
    person (id) {
        id -> Uuid,
        lang -> Varchar,
        name -> Varchar,
    }
}

table! {
    series (id) {
        id -> Uuid,
        parent_series_id -> Nullable<Uuid>,
        code -> Varchar,
        order_in_series -> Nullable<Int4>,
    }
}

table! {
    series_info (id) {
        id -> Uuid,
        series_id -> Uuid,
        lang -> Varchar,
        name -> Varchar,
    }
}

joinable!(book -> series (series_id));
joinable!(book_author_map -> book (book_id));
joinable!(book_author_map -> person (person_id));
joinable!(book_illustrator_map -> book (book_id));
joinable!(book_illustrator_map -> person (person_id));
joinable!(book_info -> book (book_id));
joinable!(book_info_translator_map -> book_info (book_info_id));
joinable!(book_info_translator_map -> person (person_id));
joinable!(series_info -> series (series_id));

allow_tables_to_appear_in_same_query!(
    book,
    book_author_map,
    book_illustrator_map,
    book_info,
    book_info_translator_map,
    person,
    series,
    series_info,
);
