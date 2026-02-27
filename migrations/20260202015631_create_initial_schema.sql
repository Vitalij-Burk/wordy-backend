-- USERS
CREATE TABLE users (
	id UUID PRIMARY KEY,
	hashed_password TEXT NOT NULL,
	key TEXT NOT NULL UNIQUE,
	name TEXT NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- WORD_PAIRS
CREATE TABLE word_pairs (
	id UUID PRIMARY KEY,
	user_id UUID NOT NULL,
	target_text TEXT NOT NULL,
	source_text TEXT NOT NULL,
	target_language TEXT NOT NULL,
	source_language TEXT NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT NOW(),
	FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX word_pairs_user_id_idx ON word_pairs(user_id);
