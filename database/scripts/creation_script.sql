CREATE TABLE IF NOT EXISTS public.product
(
    id integer NOT NULL,
    name character varying(255) COLLATE pg_catalog."default",
    category character varying(255) COLLATE pg_catalog."default",
    quantity integer,
    price integer,
    CONSTRAINT products_pkey PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS public.sale
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
    total_price integer,
    CONSTRAINT sale_pkey PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS public.sale_item
(
    id integer NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 2147483647 CACHE 1 ),
    sale_id integer NOT NULL,
    product_id integer NOT NULL,
    quantity integer NOT NULL,
    product_price integer NOT NULL,
    CONSTRAINT sale_item_pkey PRIMARY KEY (id),
    CONSTRAINT fk_sale FOREIGN KEY (sale_id) REFERENCES public.sale (id) ON DELETE CASCADE,
    CONSTRAINT fk_product FOREIGN KEY (product_id) REFERENCES public.product (id)
);


INSERT INTO public.product(id, name, category, quantity, price) VALUES (1, 'Sifu', 'Games', 10, 50);
INSERT INTO public.product(id, name, category, quantity, price) VALUES (2, 'Sony XM4', 'Sound', 5, 250);
INSERT INTO public.product(id, name, category, quantity, price) VALUES (3, 'Playstation 5', 'Games', 5, 500);
INSERT INTO public.product(id, name, category, quantity, price) VALUES (4, 'Fl Studio', 'Software', 20, 250);
INSERT INTO public.product(id, name, category, quantity, price) VALUES (5, 'NVIDIA GeForce RTX 3080 Ti ', 'Hardware', 1, 900);