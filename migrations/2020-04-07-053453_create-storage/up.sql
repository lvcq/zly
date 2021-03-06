CREATE TABLE storage(
    CREATED_TIME TIMESTAMP NOT NULL,
    UPDATED_TIME TIMESTAMP NOT NULL,
    STORAGE_ID VARCHAR(128) NOT NULL,
    STORAGE_NAME VARCHAR(128) NOT NULL,
    CREATE_ID VARCHAR(32) NOT NULL,
    PRIMARY KEY (STORAGE_ID)
);;

COMMENT ON COLUMN storage.CREATED_TIME IS '创建时间';;
COMMENT ON COLUMN storage.UPDATED_TIME IS '更新时间';;
COMMENT ON COLUMN storage.STORAGE_ID IS '空间ID';;
COMMENT ON COLUMN storage.STORAGE_NAME IS '空间名称';;
COMMENT ON COLUMN storage.CREATE_ID IS '创建者ID';;

CREATE TABLE storage_user(
    CREATED_TIME TIMESTAMP NOT NULL,
    UPDATED_TIME TIMESTAMP NOT NULL,
    USER_ID VARCHAR(128) NOT NULL,
    STORAGE_ID VARCHAR(128) NOT NULL,
    PRIMARY KEY (USER_ID,STORAGE_ID)
);;

COMMENT ON COLUMN storage_user.CREATED_TIME IS '创建时间';;
COMMENT ON COLUMN storage_user.UPDATED_TIME IS '更新时间';;
COMMENT ON COLUMN storage_user.USER_ID IS '用户ID';;
COMMENT ON COLUMN storage_user.STORAGE_ID IS '空间ID';;