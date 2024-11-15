CREATE TABLE IF NOT EXISTS permission (
    key VARCHAR PRIMARY KEY,
    description VARCHAR NOT NULL
);

INSERT INTO permission(key, description)
VALUES
    ('create_alertsource', 'Can add new alert sources'),
    ('view_alertsource', 'Can view added alert sources'),
    ('update_alertsource', 'Can update existing alert sources'),
    ('delete_alertsource', 'Can delete existing alert sources'),
    ('create_user', 'Can add new user'),
    ('view_user', 'Can view added users'),
    ('update_user', 'Can update existing users'),
    ('delete_user', 'Can delete existing users'),
    ('create_selfsubscription', 'Can add new self subscriptions'),
    ('view_selfsubscription', 'Can view added self subscriptions'),
    ('update_selfsubscription', 'Can update added self subscriptions'),
    ('delete_selfsubscription', 'Can delete added self subscriptions'),
    ('create_allsubscription', 'Can add new subscription for any user'),
    ('view_allsubscription', 'Can view all subscriptions'),
    ('update_allsubscription', 'Can update all subscriptions'),
    ('delete_allsubscription', 'Can delete add subscriptions'),
    ('create_ir', 'Can add new IR'),
    ('view_ir', 'Can view added IR'),
    ('update_ir', 'Can update existing IR'),
    ('delete_ir', 'Can delete existing IR')
    
