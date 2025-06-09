-- Table: public.sale

DROP TABLE IF EXISTS public.sale_test;

CREATE TABLE IF NOT EXISTS public.sale_test
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
    total_price integer,
    product_amount integer
    )

    TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.sale
    OWNER to postgres;

INSERT INTO public.sale_test(total_price, product_amount) VALUES (25, 1);
INSERT INTO public.sale_test(total_price, product_amount) VALUES (500, 2);
INSERT INTO public.sale_test(total_price, product_amount) VALUES (500, 1);