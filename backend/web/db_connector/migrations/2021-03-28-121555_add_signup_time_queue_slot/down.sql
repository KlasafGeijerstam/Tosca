-- This file should undo anything in `up.sql`
ALTER TABLE queue_slot_users DROP COLUMN q_time;
