ALTER TABLE public.zly_file
    ADD COLUMN file_mime character varying(256) COLLATE pg_catalog."default" NOT NULL;

COMMENT ON COLUMN public.zly_file.file_mime
    IS '文件类型';