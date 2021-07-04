/*
  Warnings:

  - The primary key for the `user_password_reset_token_destination` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - A unique constraint covering the columns `[login_id]` on the table `user` will be added. If there are existing duplicate values, this will fail.

*/
-- AlterTable
ALTER TABLE `user_password_reset_token_destination` DROP PRIMARY KEY,
    ADD PRIMARY KEY (`user_id`);

-- CreateTable
CREATE TABLE `user_password` (
    `user_id` VARCHAR(36) NOT NULL,
    `hashed_password` VARCHAR(96) NOT NULL,

    PRIMARY KEY (`user_id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `user_password_reset_token` (
    `user_id` VARCHAR(36) NOT NULL,
    `reset_token` VARCHAR(36) NOT NULL,
    `login_id` VARCHAR(100) NOT NULL,
    `expires` DATETIME(3) NOT NULL,
    `requested_at` DATETIME(3) NOT NULL,
    `reset_at` DATETIME(3),

    UNIQUE INDEX `user_password_reset_token.reset_token_unique`(`reset_token`),
    PRIMARY KEY (`user_id`, `reset_token`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateIndex
CREATE UNIQUE INDEX `user.login_id_unique` ON `user`(`login_id`);

-- AddForeignKey
ALTER TABLE `user_password` ADD FOREIGN KEY (`user_id`) REFERENCES `user`(`user_id`) ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `user_password_reset_token` ADD FOREIGN KEY (`user_id`) REFERENCES `user`(`user_id`) ON DELETE CASCADE ON UPDATE CASCADE;
