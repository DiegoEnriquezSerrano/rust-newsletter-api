CREATE TABLE user_profiles(
    bio TEXT NOT NULL DEFAULT '',
    description VARCHAR(200) NOT NULL DEFAULT '',
    display_name VARCHAR(70) NOT NULL DEFAULT '',
    user_profile_id UUID NOT NULL PRIMARY KEY,
    user_id UUID NOT NULL
      REFERENCES users(user_id)
      ON UPDATE CASCADE
      ON DELETE CASCADE
      UNIQUE
);
