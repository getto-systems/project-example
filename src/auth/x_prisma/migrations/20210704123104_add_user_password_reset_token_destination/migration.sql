/*
  Warnings:

  - Added the required column `login_id` to the `user` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE `user` ADD COLUMN `login_id` VARCHAR(100) NOT NULL;

-- CreateTable
CREATE TABLE `user_password_reset_token_destination` (
    `user_id` VARCHAR(36) NOT NULL,
    `email` VARCHAR(250) NOT NULL,

    PRIMARY KEY (`user_id`, `email`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- AddForeignKey
ALTER TABLE `user_password_reset_token_destination` ADD FOREIGN KEY (`user_id`) REFERENCES `user`(`user_id`) ON DELETE CASCADE ON UPDATE CASCADE;
