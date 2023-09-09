CREATE TABLE records (
  id VARCHAR(50) NOT NULL PRIMARY KEY,

  version TEXT NOT NULL,
  base_url TEXT NOT NULL,

  platform_id TEXT NOT NULL,
  game_id TEXT NOT NULL,
  encryption_key TEXT NOT NULL,
  metadata TEXT NOT NULL,
  keyframes TEXT NOT NULL,
  game_data_chunks TEXT NOT NULL,
  storage_path TEXT NOT NULL,

  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

  UNIQUE(platform_id, game_id)
);
