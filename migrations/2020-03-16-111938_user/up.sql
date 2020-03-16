CREATE TABLE user_info(
	    CREATED_TIME DATE,
	    USER_NAME VARCHAR(128) NOT NULL,
	    UPDATED_TIME DATE,
	    USER_ID VARCHAR(32) NOT NULL,
	    PASSWORD VARCHAR(1024) NOT NULL,
	    PRIMARY KEY (USER_ID)
);;

COMMENT ON TABLE user_info IS '用户信息表';;
COMMENT ON COLUMN user_info.CREATED_TIME IS '创建时间';;
COMMENT ON COLUMN user_info.USER_NAME IS '用户名称';;
COMMENT ON COLUMN user_info.UPDATED_TIME IS '更新时间';;
COMMENT ON COLUMN user_info.USER_ID IS '用户唯一标识符';;
COMMENT ON COLUMN user_info.PASSWORD IS '用户密码';;

CREATE TABLE role(
	    CREATED_BY VARCHAR(32),
	    CREATED_TIME DATE,
	    UPDATED_BY VARCHAR(32),
	    UPDATED_TIME DATE,
	    ROLE_ID VARCHAR(32) NOT NULL,
	    ROLE_NAME VARCHAR(32) NOT NULL,
	    PRIMARY KEY (ROLE_ID)
);;

COMMENT ON TABLE role IS '角色表';;
COMMENT ON COLUMN role.CREATED_BY IS '创建人';;
COMMENT ON COLUMN role.CREATED_TIME IS '创建时间';;
COMMENT ON COLUMN role.UPDATED_BY IS '更新人';;
COMMENT ON COLUMN role.UPDATED_TIME IS '更新时间';;
COMMENT ON COLUMN role.ROLE_ID IS '角色ID';;
COMMENT ON COLUMN role.ROLE_NAME IS '角色名称';;

CREATE TABLE user_role(
	    CREATED_BY VARCHAR(32),
	    CREATED_TIME DATE,
	    UPDATED_BY VARCHAR(32),
	    UPDATED_TIME DATE,
	    ID VARCHAR(32) NOT NULL,
	    USER_ID VARCHAR(32) NOT NULL,
	    ROLE_ID VARCHAR(32),
	    PRIMARY KEY (ID)
);;

COMMENT ON TABLE user_role IS '用户-角色关联表';;
COMMENT ON COLUMN user_role.CREATED_BY IS '创建人';;
COMMENT ON COLUMN user_role.CREATED_TIME IS '创建时间';;
COMMENT ON COLUMN user_role.UPDATED_BY IS '更新人';;
COMMENT ON COLUMN user_role.UPDATED_TIME IS '更新时间';;
COMMENT ON COLUMN user_role.ID IS 'ID';;
COMMENT ON COLUMN user_role.USER_ID IS '用户ID';;
COMMENT ON COLUMN user_role.ROLE_ID IS '角色ID';;

CREATE TABLE user_group(
	    CREATED_BY VARCHAR(32),
	    CREATED_TIME DATE,
	    UPDATED_BY VARCHAR(32),
	    UPDATED_TIME DATE,
	    USER_GROUP_ID VARCHAR(32) NOT NULL,
	    USER_GROUP_NAME VARCHAR(128),
	    PRIMARY KEY (USER_GROUP_ID)
);;

COMMENT ON TABLE user_group IS '用户组信息表';;
COMMENT ON COLUMN user_group.CREATED_BY IS '创建人';;
COMMENT ON COLUMN user_group.CREATED_TIME IS '创建时间';;
COMMENT ON COLUMN user_group.UPDATED_BY IS '更新人';;
COMMENT ON COLUMN user_group.UPDATED_TIME IS '更新时间';;
COMMENT ON COLUMN user_group.USER_GROUP_ID IS '用户组ID';;
COMMENT ON COLUMN user_group.USER_GROUP_NAME IS '用户组名称';;

CREATE TABLE user_group_user_info(
	    CREATED_BY VARCHAR(32),
	    CREATED_TIME DATE,
	    UPDATED_BY VARCHAR(32),
	    UPDATED_TIME DATE,
	    ID VARCHAR(32) NOT NULL,
	    USER_GROUP_ID VARCHAR(32) NOT NULL,
	    USER_ID VARCHAR(32) NOT NULL,
	    PRIMARY KEY (USER_GROUP_ID,USER_ID)
);;

COMMENT ON TABLE user_group_user_info IS '用户组用户信息关联表';;
COMMENT ON COLUMN user_group_user_info.CREATED_BY IS '创建人';;
COMMENT ON COLUMN user_group_user_info.CREATED_TIME IS '创建时间';;
COMMENT ON COLUMN user_group_user_info.UPDATED_BY IS '更新人';;
COMMENT ON COLUMN user_group_user_info.UPDATED_TIME IS '更新时间';;
COMMENT ON COLUMN user_group_user_info.ID IS 'ID';;
COMMENT ON COLUMN user_group_user_info.USER_GROUP_ID IS '用户组ID';;
COMMENT ON COLUMN user_group_user_info.USER_ID IS '用户ID';;

CREATE TABLE user_group_role(
	    CREATED_BY VARCHAR(32),
	    CREATED_TIME DATE,
	    UPDATED_BY VARCHAR(32),
	    UPDATED_TIME DATE,
	    ID VARCHAR(32) NOT NULL,
	    USER_GROUP_ID VARCHAR(32) NOT NULL,
	    ROLE_ID VARCHAR(32) NOT NULL,
	    PRIMARY KEY (USER_GROUP_ID,ROLE_ID)
);;

COMMENT ON TABLE user_group_role IS '用户组角色关联表';;
COMMENT ON COLUMN user_group_role.CREATED_BY IS '创建人';;
COMMENT ON COLUMN user_group_role.CREATED_TIME IS '创建时间';;
COMMENT ON COLUMN user_group_role.UPDATED_BY IS '更新人';;
COMMENT ON COLUMN user_group_role.UPDATED_TIME IS '更新时间';;
COMMENT ON COLUMN user_group_role.ID IS 'ID';;
COMMENT ON COLUMN user_group_role.USER_GROUP_ID IS '用户组ID';;
COMMENT ON COLUMN user_group_role.ROLE_ID IS '角色ＩＤ';;

CREATE TABLE menu(
	    CREATED_BY VARCHAR(32),
	    CREATED_TIME DATE,
	    UPDATED_BY VARCHAR(32),
	    UPDATED_TIME DATE,
	    ID VARCHAR(32) NOT NULL,
	    MENU_NAME VARCHAR(32) NOT NULL,
	    MENU_URL VARCHAR(128),
	    MENU_P_ID VARCHAR(32),
	    PRIMARY KEY (ID)
);;

COMMENT ON TABLE menu IS '菜单表';;
COMMENT ON COLUMN menu.CREATED_BY IS '创建人';;
COMMENT ON COLUMN menu.CREATED_TIME IS '创建时间';;
COMMENT ON COLUMN menu.UPDATED_BY IS '更新人';;
COMMENT ON COLUMN menu.UPDATED_TIME IS '更新时间';;
COMMENT ON COLUMN menu.ID IS 'ID';;
COMMENT ON COLUMN menu.MENU_NAME IS '菜单名称';;
COMMENT ON COLUMN menu.MENU_URL IS '菜单URL';;
COMMENT ON COLUMN menu.MENU_P_ID IS '父菜单ID';;

CREATE TABLE page_element(
	    CREATED_BY VARCHAR(32),
	    CREATED_TIME DATE,
	    UPDATED_BY VARCHAR(32),
	    UPDATED_TIME DATE,
	    ELEMENT_ID VARCHAR(32) NOT NULL,
	    ELEMENT_NAME VARCHAR(128) NOT NULL,
	    PRIMARY KEY (ELEMENT_ID)
);;

COMMENT ON TABLE page_element IS '页面元素表';;
COMMENT ON COLUMN page_element.CREATED_BY IS '创建人';;
COMMENT ON COLUMN page_element.CREATED_TIME IS '创建时间';;
COMMENT ON COLUMN page_element.UPDATED_BY IS '更新人';;
COMMENT ON COLUMN page_element.UPDATED_TIME IS '更新时间';;
COMMENT ON COLUMN page_element.ELEMENT_ID IS 'ID';;
COMMENT ON COLUMN page_element.ELEMENT_NAME IS '元素名称';;

CREATE TABLE power(
	    CREATED_BY VARCHAR(32),
	    CREATED_TIME DATE,
	    UPDATED_BY VARCHAR(32),
	    UPDATED_TIME DATE,
	    ID VARCHAR(32) NOT NULL,
	    POWER_TYPE VARCHAR(32) NOT NULL,
	    POWER_CODE VARCHAR(32),
	    PRIMARY KEY (ID)
);;

COMMENT ON TABLE power IS '权限表';;
COMMENT ON COLUMN power.CREATED_BY IS '创建人';;
COMMENT ON COLUMN power.CREATED_TIME IS '创建时间';;
COMMENT ON COLUMN power.UPDATED_BY IS '更新人';;
COMMENT ON COLUMN power.UPDATED_TIME IS '更新时间';;
COMMENT ON COLUMN power.ID IS 'ID';;
COMMENT ON COLUMN power.POWER_TYPE IS '权限类型';;
COMMENT ON COLUMN power.POWER_CODE IS '权限编码';;

CREATE TABLE power_menu(
	    CREATED_BY VARCHAR(32),
	    CREATED_TIME DATE,
	    UPDATED_BY VARCHAR(32),
	    UPDATED_TIME DATE,
	    ID VARCHAR(32) NOT NULL,
	    MENU_ID VARCHAR(32) NOT NULL,
	    POWER_ID VARCHAR(32) NOT NULL,
	    PRIMARY KEY (ID)
);;

COMMENT ON TABLE power_menu IS '权限菜单关联表';;
COMMENT ON COLUMN power_menu.CREATED_BY IS '创建人';;
COMMENT ON COLUMN power_menu.CREATED_TIME IS '创建时间';;
COMMENT ON COLUMN power_menu.UPDATED_BY IS '更新人';;
COMMENT ON COLUMN power_menu.UPDATED_TIME IS '更新时间';;
COMMENT ON COLUMN power_menu.ID IS 'ID';;
COMMENT ON COLUMN power_menu.MENU_ID IS '菜单ID';;
COMMENT ON COLUMN power_menu.POWER_ID IS '权限ID';;

CREATE TABLE power_page_element(
	    CREATED_BY VARCHAR(32),
	    CREATED_TIME DATE,
	    UPDATED_BY VARCHAR(32),
	    UPDATED_TIME DATE,
	    ID VARCHAR(32) NOT NULL,
	    POWER_ID VARCHAR(32) NOT NULL,
	    ELEMENT_ID VARCHAR(32) NOT NULL,
	    PRIMARY KEY (ID)
);;

COMMENT ON TABLE power_page_element IS '权限页面元素关联表';;
COMMENT ON COLUMN power_page_element.CREATED_BY IS '创建人';;
COMMENT ON COLUMN power_page_element.CREATED_TIME IS '创建时间';;
COMMENT ON COLUMN power_page_element.UPDATED_BY IS '更新人';;
COMMENT ON COLUMN power_page_element.UPDATED_TIME IS '更新时间';;
COMMENT ON COLUMN power_page_element.ID IS 'ID';;
COMMENT ON COLUMN power_page_element.POWER_ID IS '权限ID';;
COMMENT ON COLUMN power_page_element.ELEMENT_ID IS '页面元素ID';;

CREATE TABLE opetation(
	    CREATED_BY VARCHAR(32),
	    CREATED_TIME DATE,
	    UPDATED_BY VARCHAR(32),
	    UPDATED_TIME DATE,
	    ID VARCHAR(32) NOT NULL,
	    OPT_NAME VARCHAR(128) NOT NULL,
	    OPT_CODE VARCHAR(128),
	    OPT_P_ID VARCHAR(32),
	    PRIMARY KEY (ID)
);;

COMMENT ON TABLE opetation IS '功能操作表';;
COMMENT ON COLUMN opetation.CREATED_BY IS '创建人';;
COMMENT ON COLUMN opetation.CREATED_TIME IS '创建时间';;
COMMENT ON COLUMN opetation.UPDATED_BY IS '更新人';;
COMMENT ON COLUMN opetation.UPDATED_TIME IS '更新时间';;
COMMENT ON COLUMN opetation.ID IS 'ID';;
COMMENT ON COLUMN opetation.OPT_NAME IS '操作名称';;
COMMENT ON COLUMN opetation.OPT_CODE IS '操作编码';;
COMMENT ON COLUMN opetation.OPT_P_ID IS '父操作';;

CREATE TABLE power_opt(
	    CREATED_BY VARCHAR(32),
	    CREATED_TIME DATE,
	    UPDATED_BY VARCHAR(32),
	    UPDATED_TIME DATE,
	    ID VARCHAR(32) NOT NULL,
	    POWER_ID VARCHAR(32) NOT NULL,
	    OPT_ID VARCHAR(32) NOT NULL,
	    PRIMARY KEY (ID)
);;

COMMENT ON TABLE power_opt IS '操作权限表';;
COMMENT ON COLUMN power_opt.CREATED_BY IS '创建人';;
COMMENT ON COLUMN power_opt.CREATED_TIME IS '创建时间';;
COMMENT ON COLUMN power_opt.UPDATED_BY IS '更新人';;
COMMENT ON COLUMN power_opt.UPDATED_TIME IS '更新时间';;
COMMENT ON COLUMN power_opt.ID IS 'ID';;
COMMENT ON COLUMN power_opt.POWER_ID IS '权限ID';;
COMMENT ON COLUMN power_opt.OPT_ID IS '操作ID';;

CREATE TABLE role_power(
	    CREATED_BY VARCHAR(32),
	    CREATED_TIME DATE,
	    UPDATED_BY VARCHAR(32),
	    UPDATED_TIME DATE,
	    ID VARCHAR(32) NOT NULL,
	    ROLE_ID VARCHAR(32) NOT NULL,
	    POWER_ID VARCHAR(32) NOT NULL,
	    PRIMARY KEY (ID)
);;

COMMENT ON TABLE role_power IS '角色权限关联表';;
COMMENT ON COLUMN role_power.CREATED_BY IS '创建人';;
COMMENT ON COLUMN role_power.CREATED_TIME IS '创建时间';;
COMMENT ON COLUMN role_power.UPDATED_BY IS '更新人';;
COMMENT ON COLUMN role_power.UPDATED_TIME IS '更新时间';;
COMMENT ON COLUMN role_power.ID IS 'ID';;
COMMENT ON COLUMN role_power.ROLE_ID IS '角色ID';;
COMMENT ON COLUMN role_power.POWER_ID IS '权限ID';;

