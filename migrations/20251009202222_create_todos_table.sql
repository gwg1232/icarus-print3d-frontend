CREATE TABLE todos (
    todo_id SERIAL PRIMARY KEY,
    task text NOT NULL,
    is_done boolean NOT NULL DEFAULT false,
    created_at timestamptz NOT NULL DEFAULT now(),
    author_id integer NOT NULL REFERENCES users(user_id) ON DELETE CASCADE
);

CREATE INDEX idx_todos_author_id ON todos(author_id);
CREATE INDEX idx_todos_created_at ON todos(created_at DESC);
