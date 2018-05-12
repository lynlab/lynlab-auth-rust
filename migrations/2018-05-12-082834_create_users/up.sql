-- Your SQL goes here
CREATE TABLE `users` (
    `id` VARCHAR(40) PRIMARY KEY NOT NULL,
    `username` VARCHAR(40) NOT NULL,
    `password_hash` VARBINARY(255) NOT NULL,
    `password_salt` VARCHAR(32) NOT NULL,
    `email` VARCHAR(255) NOT NULL,
    `access_token` VARCHAR(255),
    `is_activated` BOOLEAN NOT NULL DEFAULT false ,
    `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    UNIQUE KEY `unique_username` (`username`),
    INDEX `idx_access_token` (`access_token`)
) DEFAULT CHARSET=utf8mb4;
