CREATE TABLE series (
  id UUID PRIMARY KEY,
  parent_series_id UUID REFERENCES series (id),
  code VARCHAR NOT NULL,
  order_in_series INTEGER NOT NULL
);
CREATE TABLE book (
  id UUID PRIMARY KEY,
  series_id UUID NOT NULL REFERENCES series (id),
  order_in_series INTEGER NOT NULL
);
CREATE TABLE series_info (
  id UUID PRIMARY KEY,
  series_id UUID NOT NULL REFERENCES series (id),
  lang VARCHAR NOT NULL,
  name VARCHAR NOT NULL
);
CREATE TABLE book_info (
  id UUID PRIMARY KEY,
  book_id UUID NOT NULL REFERENCES book (id),
  lang VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  isbn VARCHAR NOT NULL,
  published_at DATE NOT NULL
);
CREATE TABLE person (
  id UUID PRIMARY KEY,
  lang VARCHAR NOT NULL,
  name VARCHAR NOT NULL
);
CREATE TABLE book_author_map (
  id SERIAL PRIMARY KEY,
  book_id UUID NOT NULL REFERENCES book (id),
  person_id UUID NOT NULL REFERENCES person (id),
  UNIQUE (book_id, person_id)
);
CREATE TABLE book_illustrator_map (
  id SERIAL PRIMARY KEY,
  book_id UUID NOT NULL REFERENCES book (id),
  person_id UUID NOT NULL REFERENCES person (id),
  UNIQUE (book_id, person_id)
);
CREATE TABLE book_info_translator_map (
  id SERIAL PRIMARY KEY,
  book_info_id UUID NOT NULL REFERENCES book_info (id),
  person_id UUID NOT NULL REFERENCES person (id),
  UNIQUE (book_info_id, person_id)
);
