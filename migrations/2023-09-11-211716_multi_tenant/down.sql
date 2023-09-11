-- This file should undo anything in `up.sql`
DROP INDEX highscore.game_index;
ALTER TABLE highscore DROP COLUMN game;