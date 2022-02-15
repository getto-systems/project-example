-- CreateTable
CREATE TABLE `user_password_reset_token_registered_destination` (
    `user_id` VARCHAR(36) NOT NULL,
    `reset_token` VARCHAR(36) NOT NULL,
    `email` VARCHAR(250) NOT NULL,

    UNIQUE INDEX `user_password_reset_token_registered_destination.reset_token_uni`(`reset_token`),
    PRIMARY KEY (`user_id`, `reset_token`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;
