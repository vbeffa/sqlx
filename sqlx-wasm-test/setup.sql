CREATE DOMAIN month_id AS INT2 CHECK (1 <= value AND value <= 12);
CREATE TYPE year_month AS (year INT4, month month_id);
CREATE DOMAIN winter_year_month AS year_month CHECK ((value).month <= 3);
