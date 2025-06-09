-- Table: public.sale

DROP TABLE IF EXISTS public.product_test;
DROP TABLE IF EXISTS public.sale_test;

CREATE TABLE IF NOT EXISTS public.product_test
(
    id integer NOT NULL,
    name character varying(255) COLLATE pg_catalog."default",
    category character varying(255) COLLATE pg_catalog."default",
    quantity integer,
    price integer
);

CREATE TABLE IF NOT EXISTS public.sale_test
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
    total_price integer,
    product_amount integer
);

INSERT INTO public.product_test(id, name, category, quantity, price) VALUES (1, 'Test object 1', 'Test', 3, 50);
INSERT INTO public.product_test(id, name, category, quantity, price) VALUES (2, 'Test object 2', 'Test', 5, 100);
INSERT INTO public.product_test(id, name, category, quantity, price) VALUES (3, 'Test object 3', 'Case', 1, 25);

INSERT INTO public.sale_test(total_price) VALUES (25);
INSERT INTO public.sale_test(total_price) VALUES (250);
INSERT INTO public.sale_test(total_price) VALUES (500);