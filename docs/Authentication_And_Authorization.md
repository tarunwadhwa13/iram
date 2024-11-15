IRAM contains following tables for user authentication and authorization

1) **user** - Contains registered user details. Admin can enable, disable users and mark other users as admin users.
2) **groups** - Groups can be used to manage permissions for multiple users at once.
3) **user_groups** - a mapping table which contains information about user's group(s)
4) **permission** - Contains available permissions for performing operations in application. Supported permissions are already added in table.
5) **group_permission** - contains information about the permissions each group has. Admin can add/remove permissions from this table to handle user permissions in bulk

### How Authentication works - 
When IRAM received authentication request, it connects with authentication backend to validate the credentials. Authentication backend can either be the database or any other external authentication system (like LDAP or SAML). Once user is authenticated, its entry is verified in app's 'user' table. Auth module validates if user is active and returns user object in return

### How Authorization works - 
Application checks for expected permissions before performing operations. For every user, groups and fetched and group level permissions are evaluated. If user is authorized, necessary operation is performed. For admin users (where is_admin=true), no permissions are checked and all operations are allowed by default
