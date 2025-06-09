-- Table: public.product

DROP TABLE IF EXISTS public.product_test;

CREATE TABLE IF NOT EXISTS public.product_test
(
    id integer NOT NULL,
    name character varying(255) COLLATE pg_catalog."default",
    category character varying(255) COLLATE pg_catalog."default",
    quantity integer,
    price integer
    )

    TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.product
    OWNER to postgres;

INSERT INTO public.product_test(id, name, category, quantity, price) VALUES (1, 'Test object 1', 'Test', 3, 50);
INSERT INTO public.product_test(id, name, category, quantity, price) VALUES (2, 'Test object 2', 'Test', 5, 100);
INSERT INTO public.product_test(id, name, category, quantity, price) VALUES (3, 'Test object 3', 'Case', 1, 25);