-- CreateTable
CREATE TABLE `ticket_discarded` (
    `user_id` VARCHAR(36) NOT NULL,
    `ticket_id` VARCHAR(36) NOT NULL,
    `discard_at` DATETIME(3) NOT NULL,

    PRIMARY KEY (`ticket_id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- AddForeignKey
ALTER TABLE `ticket_discarded` ADD FOREIGN KEY (`user_id`) REFERENCES `user`(`user_id`) ON DELETE CASCADE ON UPDATE CASCADE;
