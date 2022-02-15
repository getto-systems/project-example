/*
  Warnings:

  - The primary key for the `ticket_discarded` table will be changed. If it partially fails, the table could be left without primary key constraint.

*/
-- AlterTable
ALTER TABLE `ticket_discarded` DROP PRIMARY KEY,
    ADD PRIMARY KEY (`ticket_id`, `discard_at`);
