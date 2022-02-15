insert into user(user_id, login_id) values(uuid(), 'admin');
insert into user_granted_role(user_id, role) select user_id, 'admin' from user where login_id = 'admin';
insert into user_granted_role(user_id, role) select user_id, 'dev-docs' from user where login_id = 'admin';
insert into user_password_reset_token_destination(user_id, email) select user_id, 'admin@getto.systems' from user where login_id = 'admin';
