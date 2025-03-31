-- Add migration script here
-- Create the users table
CREATE TABLE tb_users (
                          user_id BIGSERIAL PRIMARY KEY,
                          username VARCHAR(50) UNIQUE NOT NULL,
                          email VARCHAR(100) UNIQUE NOT NULL,
                          password_hash VARCHAR(255) NOT NULL,
                          first_name VARCHAR(50) NOT NULL,
                          last_name VARCHAR(50) NOT NULL,
                          is_active BOOLEAN NOT NULL DEFAULT TRUE,
                          is_verified BOOLEAN NOT NULL DEFAULT FALSE,

    -- Matches Rust's `Option<DateTime<Utc>>`
                          last_login TIMESTAMPTZ NULL,

    -- Matches Rust's `DateTime<Utc>` with milliseconds serialization
                          created_at TIMESTAMPTZ NOT NULL,
                          updated_at TIMESTAMPTZ NOT NULL,

    -- Matches Rust's `Option<String>`
                          phone_number VARCHAR(20) NULL,

    -- Matches Rust's `Option<NaiveDate>`
                          date_of_birth DATE NULL,

    -- Matches Rust's `Option<String>`
                          profile_picture_url VARCHAR(255) NULL,
                          bio TEXT NULL,

    -- Matches Rust's `role: String`
                          role VARCHAR(20) NOT NULL DEFAULT 'user'
);

-- Ensure `created_at` and `updated_at` have correct values on insert
ALTER TABLE tb_users
    ALTER COLUMN created_at SET DEFAULT NOW();

ALTER TABLE tb_users
    ALTER COLUMN updated_at SET DEFAULT NOW();

-- Create indexes for faster queries
CREATE INDEX idx_tb_users_email ON tb_users(email);
CREATE INDEX idx_tb_users_username ON tb_users(username);

-- Create update trigger to auto-update `updated_at`
CREATE OR REPLACE FUNCTION update_tb_users_modified_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_tb_users_modtime
    BEFORE UPDATE ON tb_users
    FOR EACH ROW
    EXECUTE FUNCTION update_tb_users_modified_column();
