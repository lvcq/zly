ALTER TABLE user_info ADD COLUMN LAST_LOGIN_TIME TIMESTAMP NOT NULL DEFAULT '2020-03-29 12:18:01';;
COMMENT ON COLUMN user_info.LAST_LOGIN_TIME IS '上次登录时间';;