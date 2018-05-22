-- Your SQL goes here
ALTER TABLE `users`
ADD COLUMN `access_token_valid_until` DATETIME AFTER `access_token`,
ADD COLUMN `activation_token` VARCHAR(16) AFTER `is_activated`,
ADD COLUMN `activation_token_valid_until` DATETIME AFTER `activation_token`,
ADD COLUMN `activation_redirection_url` VARCHAR(255) AFTER `activation_token_valid_until`;
