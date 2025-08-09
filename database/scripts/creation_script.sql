DROP TABLE IF EXISTS product CASCADE;
DROP TABLE IF EXISTS sale CASCADE;
DROP TABLE IF EXISTS sale_item CASCADE;
DROP TABLE IF EXISTS store CASCADE;
DROP TABLE IF EXISTS store_product CASCADE;


CREATE TABLE IF NOT EXISTS public.store
(
    id integer NOT NULL,
    name character varying NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS public.store_account
(
    username character varying NOT NULL,
    password character varying NOT NULL,
    PRIMARY KEY (username)
);

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

CREATE TABLE IF NOT EXISTS public.store_product
(
    store_id integer NOT NULL,
    product_id integer NOT NULL,
    quantity integer NOT NULL,
    PRIMARY KEY (store_id, product_id),
    CONSTRAINT fk_store FOREIGN KEY (store_id) REFERENCES public.store(id) ON DELETE CASCADE,
    CONSTRAINT fk_product FOREIGN KEY (product_id) REFERENCES public.product(id) ON DELETE CASCADE
);

-- Insert store
INSERT INTO public.store(id, name) VALUES (1, 'HQ');
INSERT INTO public.store(id, name) VALUES (2, 'Store 1');
INSERT INTO public.store(id, name) VALUES (3, 'Store 2');

-- Insert store accounts
INSERT INTO public.store_account(username, password) VALUES ('storehq', 'admin' );
INSERT INTO public.store_account(username, password) VALUES ('store1', 'password' );
INSERT INTO public.store_account(username, password) VALUES ('store2', 'password' );

-- Insert products
INSERT INTO public.product(id, name, category, quantity, price) VALUES (1, 'Sifu', 'Games', 10, 50);
INSERT INTO public.product(id, name, category, quantity, price) VALUES (2, 'Sony XM4', 'Sound', 5, 250);
INSERT INTO public.product(id, name, category, quantity, price) VALUES (3, 'Playstation 5', 'Games', 5, 500);
INSERT INTO public.product(id, name, category, quantity, price) VALUES (4, 'Fl Studio', 'Software', 20, 250);
INSERT INTO public.product(id, name, category, quantity, price) VALUES (5, 'NVIDIA GeForce RTX 3080 Ti', 'Hardware', 1, 900);

-- Stock for Store 1
INSERT INTO public.store_product(store_id, product_id, quantity) VALUES (2, 1, 10);
INSERT INTO public.store_product(store_id, product_id, quantity) VALUES (2, 2, 10);
INSERT INTO public.store_product(store_id, product_id, quantity) VALUES (2, 3, 10);
INSERT INTO public.store_product(store_id, product_id, quantity) VALUES (2, 4, 10);
INSERT INTO public.store_product(store_id, product_id, quantity) VALUES (2, 5, 10);

-- Stock for Store 2
INSERT INTO public.store_product(store_id, product_id, quantity) VALUES (3, 1, 10);
INSERT INTO public.store_product(store_id, product_id, quantity) VALUES (3, 2, 10);
INSERT INTO public.store_product(store_id, product_id, quantity) VALUES (3, 3, 10);
INSERT INTO public.store_product(store_id, product_id, quantity) VALUES (3, 4, 10);
INSERT INTO public.store_product(store_id, product_id, quantity) VALUES (3, 5, 10);