-- Your SQL goes here
ALTER TABLE highscore ADD COLUMN game VARCHAR(255) NOT NULL;
CREATE INDEX game_index ON highscore(game);
