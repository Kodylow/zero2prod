-- migrations/{timestamp}_create_subscriptions_table.sql 
-- Create Subscriptions Table 
CREATE TABLE subscriptions(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    subscribed_at timestamptz NOT NULL
);