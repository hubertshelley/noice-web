-- Add migration script here
CREATE TABLE `noice_db`.`noice_web_tenant_user`
(
    `id`            bigint      NOT NULL COMMENT 'ID',
    `user_id`       bigint      NOT NULL COMMENT '用户ID',
    `tenant_id`     bigint      NOT NULL COMMENT '租户ID',
    `employee_code` varchar(36) NOT NULL COMMENT '租户员工码',
    `employee_name` varchar(36) NULL COMMENT '租户员工姓名',
    `nick_name`     varchar(36) NULL COMMENT '租户员工昵称',
    PRIMARY KEY (`id`),
    UNIQUE INDEX `id_idx` (`id`) USING BTREE COMMENT 'ID索引',
    UNIQUE INDEX `user_tenant_idx` (`user_id`, `tenant_id`) USING BTREE COMMENT '用户租户索引',
    UNIQUE INDEX `code_idx` (`employee_code`) USING BTREE COMMENT '员工码索引'
) COMMENT = '租户员工表';

CREATE TABLE `noice_db`.`noice_web_tenant`
(
    `id`       bigint      NOT NULL COMMENT 'ID',
    `name`     varchar(36) NOT NULL COMMENT '租户名称',
    `owner_id` bigint      NOT NULL COMMENT '拥有者ID',
    PRIMARY KEY (`id`),
    UNIQUE INDEX `id_idx` (`id`) USING BTREE COMMENT 'ID索引'
) COMMENT = '租户表';