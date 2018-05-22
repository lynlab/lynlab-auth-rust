-- This file should undo anything in `up.sql`
ALTER TABLE `users`
DROP COLUMN `access_token_valid_until`,
DROP COLUMN `activation_token`,
DROP COLUMN `activation_token_valid_until`,
DROP COLUMN `activation_redirection_url`;
