-- CreateTable
CREATE TABLE `user_granted_role` (
    `user_id` VARCHAR(36) NOT NULL,
    `role` VARCHAR(50) NOT NULL,

    PRIMARY KEY (`user_id`, `role`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- AddForeignKey
ALTER TABLE `user_granted_role` ADD FOREIGN KEY (`user_id`) REFERENCES `user`(`user_id`) ON DELETE CASCADE ON UPDATE CASCADE;
