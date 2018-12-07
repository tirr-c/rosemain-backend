ALTER TABLE series
  DROP CONSTRAINT series_code_check,
  ADD  CONSTRAINT series_code_check CHECK (code ~* '^[a-z0-9]+(-[a-z0-9]+)*$');
