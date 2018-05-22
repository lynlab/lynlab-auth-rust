-- Your SQL goes here
ALTER TABLE `users`
ADD COLUMN `access_token_valid_until` DATETIME AFTER `access_token`,
ADD COLUMN `activate_token` VARCHAR(16) AFTER `is_activated`,
ADD COLUMN `activate_token_valid_until` DATETIME AFTER `activate_token`;
