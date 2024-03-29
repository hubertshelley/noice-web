-- Add migration script here
CREATE TABLE `noice_db`.`noice_web_user`
(
    `id`       bigint       NOT NULL AUTO_INCREMENT COMMENT 'ID',
    `username` varchar(36)  NOT NULL UNIQUE COMMENT '用户名',
    `password` varchar(255) NULL COMMENT '密码',
    `name`     varchar(36)  NULL COMMENT '用户名称',
    PRIMARY KEY (`id`),
    UNIQUE INDEX `username_idx` (`username`) USING BTREE COMMENT '用户名索引'
) COMMENT = '用户表';