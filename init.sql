DROP TABLE IF EXISTS `example_user`;
CREATE TABLE `example_user` (
    `user_id` bigint NOT NULL AUTO_INCREMENT COMMENT '用户ID' primary key ,
    `user_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '用户账号',
    `nick_name` varchar(30) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL COMMENT '用户昵称'

) ENGINE=InnoDB AUTO_INCREMENT=5147 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户表';
LOCK TABLES `example_user` WRITE;
INSERT INTO `example_user` (`user_id`, `user_name`, `nick_name`) VALUES (1,'demo_user','john');
UNLOCK TABLES;