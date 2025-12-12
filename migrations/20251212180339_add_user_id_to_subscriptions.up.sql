ALTER TABLE subscriptions
  ADD COLUMN user_id UUID NOT NULL,
  DROP CONSTRAINT subscriptions_email_key,
  ADD CONSTRAINT fkey_users_subscriptions
    FOREIGN KEY (user_id)
    REFERENCES users(user_id)
    ON UPDATE CASCADE
    ON DELETE CASCADE,
  ADD CONSTRAINT unique_subscriptions_email_and_user_id
    UNIQUE (email, user_id);
