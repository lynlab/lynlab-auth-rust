-- This file should undo anything in `up.sql`
ALTER TABLE `users`
DROP COLUMN `access_token_valid_until`,
DROP COLUMN `activate_token`,
DROP COLUMN `activate_token_valid_until`;
