ALTER TABLE subscriptions
  DROP CONSTRAINT unique_subscriptions_email_and_user_id,
  DROP COLUMN user_id,
  ADD CONSTRAINT subscriptions_email_key UNIQUE (email);
